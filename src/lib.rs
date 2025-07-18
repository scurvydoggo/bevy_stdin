//! Terminal input for the [Bevy game engine](https://bevy.org/), using
//! [crossterm](https://docs.rs/crossterm/latest/crossterm/) for cross-platform support.
//!
//! Input is exposed via resources: `ButtonInput<KeyCode>` and `ButtonInput<KeyModifiers>`.

use bevy::{
    prelude::*,
    app::AppExit,
    input::ButtonInput
};
use crossbeam_channel::{bounded, Receiver};
use crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::thread;
use std::time::Duration;

/// Adds terminal input to an App
pub struct StdinPlugin;

/// Restore terminal state on shutdown
impl Drop for StdinPlugin {
    fn drop(&mut self) {
        crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
    }
}

#[derive(Event, Deref)]
struct StdinEvent(KeyEvent);

#[derive(Resource, Deref)]
struct StreamReceiver(Receiver<StdinEvent>);

impl Plugin for StdinPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ButtonInput::<KeyCode>::default());
        app.insert_resource(ButtonInput::<KeyModifiers>::default());
        app.add_systems(Startup, setup);
        app.add_systems(PreUpdate, read_stream);
        app.add_systems(Update, ctrl_c);
    }
}

// This system sets up the channel and writes to it from a stdin polling thread
fn setup(mut commands: Commands) {
    let (tx, rx) = bounded::<StdinEvent>(1);
    commands.insert_resource(StreamReceiver(rx));

    // Raw mode is necessary to read key events without waiting for Enter
    crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode");

    thread::spawn(move || {
        let timeout = Duration::from_millis(100);
        loop {
            if event::poll(timeout).expect("Failed to poll stdin") {
                let e = event::read().expect("Failed to read stdin event");
                if let event::Event::Key(key) = e {
                    tx.send(StdinEvent(key)).expect("Failed to transmit key event");
                }
            }
        }
    });
}

// This system reads from the channel and submits key events to bevy
fn read_stream(
    stdin_keys: Res<StreamReceiver>,
    mut key_input: ResMut<ButtonInput<KeyCode>>,
    mut modifier_input: ResMut<ButtonInput<KeyModifiers>>,
) {
    key_input.reset_all();
    modifier_input.reset_all();

    for key in stdin_keys.try_iter() {
        match key.kind {
            KeyEventKind::Press => {
                key_input.press(key.code);
                modifier_input.press(key.modifiers);
            }
            KeyEventKind::Release => {
                key_input.release(key.code);
                modifier_input.release(key.modifiers);
            }
            KeyEventKind::Repeat => {}
        }
    }
}

/// Monitor for Ctrl+C and shut down bevy
fn ctrl_c(
    key: Res<ButtonInput<KeyCode>>,
    modifier: Res<ButtonInput<KeyModifiers>>,
    mut ev_exit: EventWriter<AppExit>,
) {
    if modifier.just_pressed(KeyModifiers::CONTROL) && key.just_pressed(KeyCode::Char('c')) {
        ev_exit.write(AppExit::Success);
    }
}
