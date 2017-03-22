# musical_keyboard [![Build Status](https://travis-ci.org/RustAudio/musical_keyboard.svg?branch=master)](https://travis-ci.org/RustAudio/musical_keyboard) [![Crates.io](https://img.shields.io/crates/v/musical_keyboard.svg)](https://crates.io/crates/musical_keyboard) [![Crates.io](https://img.shields.io/crates/l/musical_keyboard.svg)](https://github.com/RustAudio/musical_keyboard/blob/master/LICENSE-MIT) [![docs.rs](https://docs.rs/musical_keyboard/badge.svg)](https://docs.rs/musical_keyboard/)

A small lib for converting keyboard input into musical notes.

```Rust
extern crate musical_keyboard;
use musical_keyboard::{Letter, MusicalKeyboard, NoteEvent, Key};

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
```

The behaviour is modelled on Logic Studio X's musical keyboard.


License
-------

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


**Contributions**

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
