use crate::color::Color;
use crate::prelude::Component;
use quick_xml::Reader;
use quick_xml::events::Event;
use thiserror::Error;

#[derive(Default, Clone)]
struct Style {
    tag: String,
    color: Option<Color>,
    bold: bool,
    italic: bool,
    underlined: bool,
    strikethrough: bool,
    obfuscated: bool,
}

#[derive(Debug, Error)]
pub enum MiniMessageError {
    #[error("Failed to parse MiniMessage: {source}. Input: {input}")]
    QuickXml {
        source: quick_xml::Error,
        input: String,
    },
    #[error("Encoding error while parsing MiniMessage: {source}. Input: {input}")]
    Encoding {
        source: quick_xml::encoding::EncodingError,
        input: String,
    },
}

enum ParsedTag {
    Color(Color),
    Bold,
    Italic,
    Underlined,
    Strikethrough,
    Obfuscated,
    Newline,
    Reset,
    None,
}

fn parse_tag(tag: &str) -> ParsedTag {
    let lower = tag.to_ascii_lowercase();
    let lower_str = lower.as_str();

    match lower_str {
        "bold" | "b" => ParsedTag::Bold,
        "italic" | "i" | "em" => ParsedTag::Italic,
        "underlined" | "u" => ParsedTag::Underlined,
        "strikethrough" | "st" => ParsedTag::Strikethrough,
        "obfuscated" | "obf" => ParsedTag::Obfuscated,
        "newline" => ParsedTag::Newline,
        "reset" | "r" => ParsedTag::Reset,
        _ => {
            if let Some(color) = Color::parse(lower_str) {
                return ParsedTag::Color(color);
            }

            for prefix in ["color:", "colour:", "c:"] {
                if let Some(color) = lower_str.strip_prefix(prefix).and_then(Color::parse) {
                    return ParsedTag::Color(color);
                }
            }

            ParsedTag::None
        }
    }
}

fn matches_close_tag(opened_tag: &str, closed_tag: &str) -> bool {
    let opened = opened_tag.to_ascii_lowercase();
    let closed = closed_tag.to_ascii_lowercase();

    if opened == closed {
        return true;
    }

    if (closed == "color" || closed == "colour" || closed == "c")
        && (opened.starts_with("color:")
            || opened.starts_with("colour:")
            || opened.starts_with("c:")
            || opened.starts_with('#')
            || Color::parse(&opened).is_some())
    {
        return true;
    }

    if (closed == "bold" || closed == "b") && (opened == "bold" || opened == "b") {
        return true;
    }
    if (closed == "italic" || closed == "i" || closed == "em")
        && (opened == "italic" || opened == "i" || opened == "em")
    {
        return true;
    }
    if (closed == "underlined" || closed == "u") && (opened == "underlined" || opened == "u") {
        return true;
    }
    if (closed == "strikethrough" || closed == "st")
        && (opened == "strikethrough" || opened == "st")
    {
        return true;
    }
    if (closed == "obfuscated" || closed == "obf") && (opened == "obfuscated" || opened == "obf") {
        return true;
    }

    false
}

pub fn parse_mini_message(input: &str) -> Result<Component, MiniMessageError> {
    let wrapped_input = format!("<root>{input}</root>");
    let mut reader = Reader::from_str(&wrapped_input);
    reader.config_mut().check_end_names = false;

    let mut flat_components = Vec::new();
    let mut style_stack: Vec<Style> = vec![Style::default()];

    loop {
        match reader
            .read_event()
            .map_err(|e| MiniMessageError::QuickXml {
                source: e,
                input: input.to_string(),
            })? {
            Event::Start(e) => {
                let tag_name = e.name().as_ref().to_string();

                match parse_tag(&tag_name) {
                    ParsedTag::Newline => {
                        if let Some(current_style) = style_stack.last() {
                            flat_components.push(Component {
                                text: "\n".to_string(),
                                color: current_style.color.clone(),
                                bold: current_style.bold,
                                italic: current_style.italic,
                                underlined: current_style.underlined,
                                strikethrough: current_style.strikethrough,
                                obfuscated: current_style.obfuscated,
                                extra: vec![],
                            });
                        }
                    }
                    ParsedTag::Reset => {
                        style_stack = vec![Style::default()];
                    }
                    ParsedTag::Color(color) => {
                        let mut new_style = style_stack.last().cloned().unwrap_or_default();
                        new_style.tag = tag_name;
                        new_style.color = Some(color);
                        style_stack.push(new_style);
                    }
                    ParsedTag::Bold => {
                        let mut new_style = style_stack.last().cloned().unwrap_or_default();
                        new_style.tag = tag_name;
                        new_style.bold = true;
                        style_stack.push(new_style);
                    }
                    ParsedTag::Italic => {
                        let mut new_style = style_stack.last().cloned().unwrap_or_default();
                        new_style.tag = tag_name;
                        new_style.italic = true;
                        style_stack.push(new_style);
                    }
                    ParsedTag::Underlined => {
                        let mut new_style = style_stack.last().cloned().unwrap_or_default();
                        new_style.tag = tag_name;
                        new_style.underlined = true;
                        style_stack.push(new_style);
                    }
                    ParsedTag::Strikethrough => {
                        let mut new_style = style_stack.last().cloned().unwrap_or_default();
                        new_style.tag = tag_name;
                        new_style.strikethrough = true;
                        style_stack.push(new_style);
                    }
                    ParsedTag::Obfuscated => {
                        let mut new_style = style_stack.last().cloned().unwrap_or_default();
                        new_style.tag = tag_name;
                        new_style.obfuscated = true;
                        style_stack.push(new_style);
                    }
                    ParsedTag::None => {}
                }
            }
            Event::End(e) => {
                let tag_name = e.name().as_ref().to_string();
                if let Some(pos) = style_stack
                    .iter()
                    .rposition(|s| matches_close_tag(&s.tag, &tag_name))
                {
                    style_stack.truncate(pos);
                }
            }
            Event::Text(e) => {
                let text = e.as_ref().to_string();
                if text.is_empty() {
                    continue;
                }

                if let Some(current_style) = style_stack.last() {
                    flat_components.push(Component {
                        text,
                        color: current_style.color.clone(),
                        bold: current_style.bold,
                        italic: current_style.italic,
                        underlined: current_style.underlined,
                        strikethrough: current_style.strikethrough,
                        obfuscated: current_style.obfuscated,
                        extra: vec![],
                    });
                }
            }
            Event::Empty(e) => {
                let tag_name = e.name().as_ref().to_string();
                match parse_tag(&tag_name) {
                    ParsedTag::Newline => {
                        if let Some(current_style) = style_stack.last() {
                            flat_components.push(Component {
                                text: "\n".to_string(),
                                color: current_style.color.clone(),
                                bold: current_style.bold,
                                italic: current_style.italic,
                                underlined: current_style.underlined,
                                strikethrough: current_style.strikethrough,
                                obfuscated: current_style.obfuscated,
                                extra: vec![],
                            });
                        }
                    }
                    ParsedTag::Reset => {
                        style_stack = vec![Style::default()];
                    }
                    _ => {}
                }
            }
            Event::Eof => break,
            _ => (),
        }
    }

    if flat_components.is_empty() {
        Ok(Component::default())
    } else {
        Ok(Component {
            extra: flat_components,
            ..Component::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::NamedColor;

    #[test]
    fn test_example_from_prompt_nested() {
        let input = "<red><bold>Hello,</bold></red> <blue>world!</blue>";
        let result = parse_mini_message(input).unwrap();

        let expected = Component {
            extra: vec![
                Component {
                    text: "Hello,".to_string(),
                    color: Some(Color::Named(NamedColor::Red)),
                    bold: true,
                    ..Component::default()
                },
                Component {
                    text: " ".to_string(),
                    ..Component::default()
                },
                Component {
                    text: "world!".to_string(),
                    color: Some(Color::Named(NamedColor::Blue)),
                    ..Component::default()
                },
            ],
            ..Component::default()
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_hex_colors() {
        let input =
            "<#ff0088>Hex color</#ff0088> <color:#123456>Verbose hex</color> <c:#abcdef>Short</c>";
        let result = parse_mini_message(input).unwrap();

        let expected = Component {
            extra: vec![
                Component {
                    text: "Hex color".to_string(),
                    color: Some(Color::Hex(0xFF, 0x00, 0x88)),
                    ..Component::default()
                },
                Component {
                    text: " ".to_string(),
                    ..Component::default()
                },
                Component {
                    text: "Verbose hex".to_string(),
                    color: Some(Color::Hex(0x12, 0x34, 0x56)),
                    ..Component::default()
                },
                Component {
                    text: " ".to_string(),
                    ..Component::default()
                },
                Component {
                    text: "Short".to_string(),
                    color: Some(Color::Hex(0xAB, 0xCD, 0xEF)),
                    ..Component::default()
                },
            ],
            ..Component::default()
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_json_serialization_nested() {
        let input = "<red><bold>Hello,</bold></red> <blue>world!</blue>";
        let component = parse_mini_message(input).unwrap();
        let json_output = serde_json::to_string(&component).unwrap();

        let expected_json = r#"{"text":"","extra":[{"text":"Hello,","color":"red","bold":true},{"text":" "},{"text":"world!","color":"blue"}]}"#;

        assert_eq!(json_output, expected_json);
    }

    #[test]
    fn test_plain_text_nested() {
        let input = "Just some plain text.";
        let result = parse_mini_message(input).unwrap();
        let expected = Component {
            extra: vec![Component {
                text: "Just some plain text.".to_string(),
                ..Component::default()
            }],
            ..Component::default()
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_nested_tags_nested() {
        let input = "<red>This is red, <bold>and this is bold red.</bold> Back to red.</red>";
        let result = parse_mini_message(input).unwrap();
        let expected = Component {
            text: String::new(),
            extra: vec![
                Component {
                    text: "This is red, ".to_string(),
                    color: Some(Color::Named(NamedColor::Red)),
                    ..Component::default()
                },
                Component {
                    text: "and this is bold red.".to_string(),
                    color: Some(Color::Named(NamedColor::Red)),
                    bold: true,
                    ..Component::default()
                },
                Component {
                    text: " Back to red.".to_string(),
                    color: Some(Color::Named(NamedColor::Red)),
                    ..Component::default()
                },
            ],
            ..Component::default()
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_input_nested() {
        let input = "";
        let result = parse_mini_message(input).unwrap();
        let expected = Component::default();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_non_closing_tags() {
        let input = "<red><bold>Non-closing tags<italic> are supported</bold></red>";
        let result = parse_mini_message(input).unwrap();
        let expected = Component {
            text: String::new(),
            extra: vec![
                Component {
                    text: "Non-closing tags".to_string(),
                    color: Some(Color::Named(NamedColor::Red)),
                    bold: true,
                    ..Component::default()
                },
                Component {
                    text: " are supported".to_string(),
                    color: Some(Color::Named(NamedColor::Red)),
                    bold: true,
                    italic: true,
                    ..Component::default()
                },
            ],
            ..Component::default()
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_newline_tag() {
        let input = "First line.<newline>Second line.";
        let result = parse_mini_message(input).unwrap();
        let expected = Component {
            extra: vec![
                Component {
                    text: "First line.".to_string(),
                    ..Component::default()
                },
                Component {
                    text: "\n".to_string(),
                    ..Component::default()
                },
                Component {
                    text: "Second line.".to_string(),
                    ..Component::default()
                },
            ],
            ..Component::default()
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_newline_self_closing_tag() {
        let input = "<green>Hello<newline/>world!</green>";
        let result = parse_mini_message(input).unwrap();
        let expected = Component {
            extra: vec![
                Component {
                    text: "Hello".to_string(),
                    color: Some(Color::Named(NamedColor::Green)),
                    ..Component::default()
                },
                Component {
                    text: "\n".to_string(),
                    color: Some(Color::Named(NamedColor::Green)),
                    ..Component::default()
                },
                Component {
                    text: "world!".to_string(),
                    color: Some(Color::Named(NamedColor::Green)),
                    ..Component::default()
                },
            ],
            ..Component::default()
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_reset_tag() {
        let input = "<red><bold>Red and bold<reset>Normal text</reset>";
        let result = parse_mini_message(input).unwrap();
        let expected = Component {
            extra: vec![
                Component {
                    text: "Red and bold".to_string(),
                    color: Some(Color::Named(NamedColor::Red)),
                    bold: true,
                    ..Component::default()
                },
                Component {
                    text: "Normal text".to_string(),
                    ..Component::default()
                },
            ],
            ..Component::default()
        };
        assert_eq!(result, expected);
    }
}
