# musical_keyboard [![Build Status](https://travis-ci.org/RustAudio/musical_keyboard.svg?branch=master)](https://travis-ci.org/RustAudio/musical_keyboard)

A small lib for converting keyboard input into musical notes. i.e.

```Rust

if let Some((letter, octave, amplitude)) = musical_keyboard.handle_input(ctrl, key) {
    println!("amplitude: {}, letter: {}, octave: {}", amplitude, letter, octave);
}

```

The behaviour is modelled on Logic Studio X's musical keyboard.

`musical_keyboard` uses:

- [Piston's `input` crate](https://github.com/PistonDevelopers/input) for the keyboard input.
- [pitch_calc](https://github.com/RustAudio/pitch_calc) for it's Letter and Octave types.
