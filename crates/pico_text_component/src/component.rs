use crate::color::Color;
use minecraft_protocol::prelude::{BinaryWriter, BinaryWriterError, EncodePacket, ProtocolVersion};
use pico_nbt::Value;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default, Clone)]
pub struct Component {
    #[serde(default)]
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(skip_serializing_if = "is_false", default)]
    pub bold: bool,
    #[serde(skip_serializing_if = "is_false", default)]
    pub italic: bool,
    #[serde(skip_serializing_if = "is_false", default)]
    pub underlined: bool,
    #[serde(skip_serializing_if = "is_false", default)]
    pub strikethrough: bool,
    #[serde(skip_serializing_if = "is_false", default)]
    pub obfuscated: bool,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub extra: Vec<Component>,
}

const fn is_false(b: &bool) -> bool {
    !*b
}

impl Component {
    pub fn new<S>(content: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            text: content.into(),
            ..Default::default()
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap_or_default()
    }

    pub fn to_version_json(&self, protocol_version: ProtocolVersion) -> String {
        if protocol_version.is_after_inclusive(ProtocolVersion::V1_16) || protocol_version.is_any()
        {
            self.to_json()
        } else {
            self.to_fallback().to_json()
        }
    }

    pub fn to_nbt(&self) -> Value {
        pico_nbt::to_value(self).unwrap()
    }

    pub fn to_fallback(&self) -> Self {
        Self {
            text: self.text.clone(),
            color: self.color.as_ref().map(|c| Color::Named(c.to_fallback())),
            bold: self.bold,
            italic: self.italic,
            underlined: self.underlined,
            strikethrough: self.strikethrough,
            obfuscated: self.obfuscated,
            extra: self.extra.iter().map(|e| e.to_fallback()).collect(),
        }
    }

    pub fn to_legacy(&self) -> String {
        #[derive(Serialize)]
        struct TextComponent {
            #[serde(default)]
            text: String,
        }
        serde_json::to_string(&TextComponent {
            text: self.to_legacy_impl(true),
        })
        .unwrap_or_default()
    }

    fn to_legacy_impl(&self, is_root: bool) -> String {
        let mut s = String::new();

        if !is_root {
            s.push('§');
            s.push('r');
        }

        if let Some(color) = &self.color {
            s.push('§');
            s.push(color.legacy_char());
        }

        if self.bold {
            s.push('§');
            s.push('l');
        }
        if self.italic {
            s.push('§');
            s.push('o');
        }
        if self.underlined {
            s.push('§');
            s.push('n');
        }
        if self.strikethrough {
            s.push('§');
            s.push('m');
        }
        if self.obfuscated {
            s.push('§');
            s.push('k');
        }

        s.push_str(&self.text);

        for extra in &self.extra {
            s.push_str(&extra.to_legacy_impl(false));
        }

        s
    }
}

impl EncodePacket for Component {
    fn encode(
        &self,
        writer: &mut BinaryWriter,
        protocol_version: ProtocolVersion,
    ) -> Result<(), BinaryWriterError> {
        if protocol_version.is_after_inclusive(ProtocolVersion::V1_20_3) {
            self.to_nbt().encode(writer, protocol_version)?;
        } else {
            self.to_version_json(protocol_version)
                .encode(writer, protocol_version)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_version_json_fallback() {
        let comp = Component {
            text: "Hello".to_string(),
            color: Some(Color::Hex(0xFF, 0x55, 0x55)),
            ..Default::default()
        };

        let json_modern = comp.to_version_json(ProtocolVersion::V1_16);
        assert!(json_modern.contains("\"color\":\"#ff5555\""));
        let json_any = comp.to_version_json(ProtocolVersion::Any);
        assert!(json_any.contains("\"color\":\"#ff5555\""));

        let json_legacy = comp.to_version_json(ProtocolVersion::V1_15_2);
        assert!(json_legacy.contains("\"color\":\"red\""));
        assert!(!json_legacy.contains("#ff5555"));

        let json_v1_8 = comp.to_version_json(ProtocolVersion::V1_8);
        assert!(json_v1_8.contains("\"color\":\"red\""));
    }

    #[test]
    fn test_to_legacy_with_hex_color() {
        let comp = Component {
            text: "World".to_string(),
            color: Some(Color::Hex(0x55, 0xFF, 0x55)),
            ..Default::default()
        };

        let legacy = comp.to_legacy();

        assert!(legacy.contains("§aWorld"));
    }

    #[test]
    fn test_component_encode_packet_across_versions() {
        let comp = Component {
            text: "Test".to_string(),
            color: Some(Color::Hex(0x00, 0x00, 0x00)),
            ..Default::default()
        };

        let mut writer_v1_12 = BinaryWriter::new();
        comp.encode(&mut writer_v1_12, ProtocolVersion::V1_12_2)
            .unwrap();
        let str_v1_12 = String::from_utf8_lossy(writer_v1_12.as_slice());
        assert!(str_v1_12.contains("\"color\":\"black\""));

        let mut writer_v1_16 = BinaryWriter::new();
        comp.encode(&mut writer_v1_16, ProtocolVersion::V1_16)
            .unwrap();
        let str_v1_16 = String::from_utf8_lossy(writer_v1_16.as_slice());
        assert!(str_v1_16.contains("\"color\":\"#000000\""));
    }
}
