extern crate musical_keyboard;

use musical_keyboard::{Letter, MusicalKeyboard, NoteOn, NoteOff, Key};

fn main() {
    let mut musical_keyboard = MusicalKeyboard::default();

    let on = musical_keyboard.key_pressed(Key::A);
    assert_eq!(Some(NoteOn { letter: Letter::C, octave: 2, velocity: 1.0 }), on);

    musical_keyboard.key_pressed(Key::X);
    let on = musical_keyboard.key_pressed(Key::D);
    assert_eq!(Some(NoteOn { letter: Letter::E, octave: 3, velocity: 1.0 }), on);

    let off = musical_keyboard.key_released(Key::D);
    assert_eq!(Some(NoteOff { letter: Letter::E, octave: 3 }), off);

    musical_keyboard.key_pressed(Key::C);
    let on = musical_keyboard.key_pressed(Key::Semicolon);
    assert_eq!(Some(NoteOn { letter: Letter::E, octave: 4, velocity: 0.95 }), on);
}
