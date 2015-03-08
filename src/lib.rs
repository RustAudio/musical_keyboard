//!
//!  musical_keyboard.rs
//!
//!  Created by Mitchell Nordine at 01:43PM on October 15, 2014.
//!
//!

extern crate input;
extern crate "pitch_calc" as pitch;

pub use pitch::{Letter, Octave};
pub use input::keyboard::Key;

pub type Velocity = f32;

/// A struct used for creating musical `Note`s via the computer keyboard.
#[derive(Copy, Clone, Debug)]
pub struct MusicalKeyboard {
    /// The current base octave for the keyboard.
    pub octave: Octave,
    /// The current velocity for the generated notes.
    pub velocity: Velocity,
    /// Whether or not the keyboard is currently active.
    pub is_active: bool,
}

/// The event that is returned from 
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NoteEvent {
    On(Letter, Octave, Velocity),
    Off(Letter, Octave),
}

impl MusicalKeyboard {

    /// Constructor for MusicalKeyboard.
    pub fn new(octave: Octave, velocity: Velocity, is_active: bool) -> MusicalKeyboard {
        MusicalKeyboard {
            octave: octave,
            velocity: velocity,
            is_active: is_active,
        }
    }

    /// Default constructor for MusicalKeyboard.
    pub fn default() -> MusicalKeyboard {
        MusicalKeyboard {
            octave: 2,
            velocity: 1.0,
            is_active: true,
        }
    }

    /// Handle keyboard input. This will check the given key for the following:
    /// - Z will step the octave down.
    /// - X will step the octave up.
    /// - C will step the velocity down.
    /// - V will step the velocity up.
    /// - Ctrl + K will toggle the keyboard on and off.
    /// - Home-row and some of the top row will trigger notes or release them depending on is_pressed.
    pub fn handle_input(&mut self, ctrl: bool, key: Key, is_pressed: bool) -> Option<NoteEvent> {
        let is_active = self.is_active;
        match (ctrl, key, is_active, is_pressed) {
            (true,  Key::K, _   , true)  => self.is_active = self.is_active != true,
            (false, Key::Z, true, true)  => if self.octave > -2 { self.octave -= 1 },
            (false, Key::X, true, true)  => if self.octave < 12 { self.octave += 1 },
            (false, Key::C, true, true)  => if self.velocity > 0.0 { self.velocity -= 0.05 },
            (false, Key::V, true, true)  => if self.velocity < 1.0 { self.velocity += 0.05 },
            (false, _,      true, true)  => return self.maybe_note_on(key),
            (false, _,      true, false) => return self.maybe_note_off(key),
            _ => (),
        }
        None
    }

    /// Translates a key into it's respective note.
    /// This key pattern is an attempt at modelling a piano's keys, where Key::A is a piano's C.
    pub fn maybe_note(&self, key: Key) -> Option<(Letter, Octave)> {
        let (octave, letter): (Octave, Letter) = match key {
            Key::A         => (0, Letter::C),
            Key::W         => (0, Letter::Csh),
            Key::S         => (0, Letter::D),
            Key::E         => (0, Letter::Dsh),
            Key::D         => (0, Letter::E),
            Key::F         => (0, Letter::F),
            Key::T         => (0, Letter::Fsh),
            Key::G         => (0, Letter::G),
            Key::Y         => (0, Letter::Gsh),
            Key::H         => (0, Letter::A),
            Key::U         => (0, Letter::Ash),
            Key::J         => (0, Letter::B),
            Key::K         => (1, Letter::C),
            Key::O         => (1, Letter::Csh),
            Key::L         => (1, Letter::D),
            Key::P         => (1, Letter::Dsh),
            Key::Semicolon => (1, Letter::E),
            Key::Quote     => (1, Letter::F),
            _ => return None,
        };
        Some((letter, octave + self.octave))
    }

    /// Translates a pressed key to a note on event.
    pub fn maybe_note_on(&self, key: Key) -> Option<NoteEvent> {
        self.maybe_note(key).map(|(letter, octave)| NoteEvent::On(letter, octave, self.velocity))
    }

    /// Translates a released key to a note off event.
    pub fn maybe_note_off(&self, key: Key) -> Option<NoteEvent> {
        self.maybe_note(key).map(|(letter, octave)| NoteEvent::Off(letter, octave))
    }

}

