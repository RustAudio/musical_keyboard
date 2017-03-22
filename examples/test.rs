
extern crate musical_keyboard;

use musical_keyboard::{Letter, MusicalKeyboard, NoteEvent, Key};

fn main() {
    let mut musical_keyboard = MusicalKeyboard::default();

    let event = musical_keyboard.key_pressed(Key::A);
    assert_eq!(Some(NoteEvent::On(Letter::C, 2, 1.0)), event);

    musical_keyboard.handle_input(Key::X, true);
    let event = musical_keyboard.key_pressed(Key::D);
    assert_eq!(Some(NoteEvent::On(Letter::E, 3, 1.0)), event);

    let event = musical_keyboard.key_released(Key::D);
    assert_eq!(Some(NoteEvent::Off(Letter::E, 3)), event);

    musical_keyboard.handle_input(Key::C, true);
    let event = musical_keyboard.key_pressed(Key::Semicolon);
    assert_eq!(Some(NoteEvent::On(Letter::E, 4, 0.95)), event);
}

