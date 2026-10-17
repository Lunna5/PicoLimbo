# Message Formatting

PicoLimbo supports **MiniMessage** formatting for styling text messages displayed to players. MiniMessage provides a modern, readable syntax for text formatting that's more intuitive than legacy color codes.

## What is MiniMessage?

MiniMessage is a text formatting system that uses XML-like tags to apply colors and formatting to text. Instead of cryptic codes like `§a` or `&c`, you can use descriptive tags like `<green>` or `<red>`.
Learn more about MiniMessage on [Adventure's documentation](https://docs.advntr.dev/minimessage/index.html).

> [!NOTE]
> MiniMessage is the recommended formatting method. Legacy color codes (like `§a` or `&c`) are still supported but may be deprecated in future versions.

## Basic Syntax

MiniMessage uses angle brackets `<>` to define formatting tags:

:::code-group
```xml
<red>This text is red</red>
<green>This text is green</green>
<#ff5555>Custom hex color</#ff5555>
<color:#3498db>Alternative hex syntax</color>
<bold>This text is bold</bold>
```
:::

## Supported Features

PicoLimbo currently supports a **subset** of MiniMessage features:

### ✅ Supported
- **Colors** - All standard Minecraft colors (e.g. `<red>`, `<gold>`, `<blue>`) and custom hexadecimal colors (`<#RRGGBB>` or `<color:#RRGGBB>`)
- **Formatting** - `<bold>`, `<italic>`, `<underlined>`, `<strikethrough>`, `<obfuscated>`, and `<reset>`
- **New lines** - `<newline>`

> [!NOTE]
> **Version Compatibility & Fallback**: Hexadecimal colors (`#RRGGBB`) are natively supported on Minecraft Java Edition 1.16 and newer. For older clients (versions 1.7.2 through 1.15.2) and legacy text output, PicoLimbo automatically falls back to the closest standard Minecraft color using Euclidean RGB color matching.

### ❌ Not Yet Supported
- Gradients and rainbow
- Hover events
- Click events
- Custom fonts
- Keybinds
- Translatable components

## Examples

:::code-group
```toml [server.toml]
welcome_message = "<#55ff55>Welcome to <bold><#ffaa00>PicoLimbo</#ffaa00></bold>!</#55ff55>"
```
:::
