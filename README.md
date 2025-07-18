# bevy_stdin

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/bevyengine/bevy#license)
[![Crates.io](https://img.shields.io/crates/v/bevy_stdin.svg)](https://crates.io/crates/bevy_stdin)
[![Downloads](https://img.shields.io/crates/d/bevy_stdin.svg)](https://crates.io/crates/bevy_stdin)
[![Docs](https://docs.rs/bevy_stdin/badge.svg)](https://docs.rs/bevy_stdin/latest/bevy_stdin/)

Terminal input for the [Bevy game engine](https://bevy.org/), using [crossterm](https://docs.rs/crossterm/latest/crossterm/) for cross-platform support.

Input is exposed via resources: `ButtonInput<KeyCode>` and `ButtonInput<KeyModifiers>`.

## Example usage

```rust
use bevy::prelude::*;
use crossterm::event::{KeyCode, KeyModifiers};

fn terminal_system(
    key: Res<ButtonInput<KeyCode>>,
    modifiers: Res<ButtonInput<KeyModifiers>>,
) {
    if modifiers.justPressed(KeyModifiers::CONTROL) && key.justPressed(KeyCode::Char('a')) {
    }
}
```
