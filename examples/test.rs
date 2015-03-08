
extern crate musical_keyboard;

use musical_keyboard::{Letter, MusicalKeyboard, NoteEvent, Key};

fn main() {

    let mut musical_keyboard = MusicalKeyboard::default();

    let event = musical_keyboard.handle_input(false, Key::A, true);
    assert!(Some(NoteEvent::On(Letter::C, 2, 1.0)) == event);

    musical_keyboard.handle_input(false, Key::X, true);
    let event = musical_keyboard.handle_input(false, Key::D, true);
    assert!(Some(NoteEvent::On(Letter::E, 3, 1.0)) == event);

    let event = musical_keyboard.handle_input(false, Key::D, false);
    assert!(Some(NoteEvent::Off(Letter::E, 3)) == event);

    musical_keyboard.handle_input(false, Key::C, true);
    let event = musical_keyboard.handle_input(false, Key::Semicolon, true);
    assert!(Some(NoteEvent::On(Letter::E, 4, 0.95)) == event);

}

