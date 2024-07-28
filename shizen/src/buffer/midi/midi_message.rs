use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
#[non_exhaustive]
#[repr(u8)]
pub enum MidiMessage {
    NoteOn { note_number: u8, velocity: u8 } = Self::NOTE_ON,
    NoteOff { note_number: u8, velocity: u8 } = Self::NOTE_OFF,
    ControlChange { controller_number: u8, velocity: u8 } = Self::CONTROL_CHANGE,
}

impl MidiMessage {
    pub const NOTE_ON: u8 = 0x80; // 0x80..0x8F
    pub const NOTE_OFF: u8 = 0x90; // 0x90..0x9F
    pub const CONTROL_CHANGE: u8 = 0xB0; // 0xB0..0xBF

    pub const fn from_bytes(bytes: [u8; 3]) -> Self {
        match bytes[0] & 0xF0 {
            0x80 => Self::NoteOff {
                note_number: bytes[1],
                velocity: 0, // disregard bytes[2] because velocity always = 0 for NoteOff
            },
            0x90 => Self::NoteOn {
                note_number: bytes[1],
                velocity: bytes[2],
            },
            0xB0 => Self::ControlChange {
                controller_number: bytes[1],
                velocity: bytes[2],
            },
            _ => unimplemented!(),
        }
    }

    /// Mutes a MIDI note by setting its velocity to 0
    pub fn mute(&mut self) {
        match self {
            MidiMessage::NoteOn { velocity, .. } => *velocity = 0,
            MidiMessage::ControlChange { velocity, .. } => *velocity = 0,
            MidiMessage::NoteOff { .. } => (), // already muted, 0 velocity
        }
    }

    /// Transposes a MIDI note by a given interval.
    ///
    /// The [`+`](`Add::add`), [`-`](`Sub::sub`), [`+=`](`AddAssign::add_assign`), and [`-=`](`SubAssign::sub_assign`) operators can also be used to transpose a note.
    ///
    /// # Example
    /// ```
    /// # use shizen::prelude::MidiMessage;
    /// let mut note_on = MidiMessage::NoteOn { note_number: 60, velocity: 100 };
    /// note_on.transpose(-5);
    /// note_on += 10;
    ///
    /// assert_eq!(note_on, MidiMessage::NoteOn { note_number: 65, velocity: 100 });
    /// ```
    pub fn transpose(&mut self, interval: i8) {
        if let MidiMessage::NoteOn { note_number, .. } = self {
            let original_note = *note_number as i8;
            let new_note = (original_note + interval).clamp(0, 127) as u8;
            *note_number = new_note;
        }
    }

    #[inline]
    #[must_use]
    pub const fn is_note_on(&self) -> bool {
        matches!(self, MidiMessage::NoteOn { .. })
    }

    #[inline]
    #[must_use]
    pub const fn is_note_off(&self) -> bool {
        matches!(self, MidiMessage::NoteOff { .. })
    }

    #[inline]
    #[must_use]
    pub const fn is_control_change(&self) -> bool {
        matches!(self, MidiMessage::ControlChange { .. })
    }
}

impl From<[u8; 3]> for MidiMessage {
    fn from(bytes: [u8; 3]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl Add<i8> for MidiMessage {
    type Output = Self;

    fn add(mut self, rhs: i8) -> Self::Output {
        self.transpose(rhs);
        self
    }
}

impl Sub<i8> for MidiMessage {
    type Output = Self;

    fn sub(mut self, rhs: i8) -> Self::Output {
        self.transpose(-rhs);
        self
    }
}

impl AddAssign<i8> for MidiMessage {
    fn add_assign(&mut self, rhs: i8) {
        self.transpose(rhs);
    }
}

impl SubAssign<i8> for MidiMessage {
    fn sub_assign(&mut self, rhs: i8) {
        self.transpose(-rhs);
    }
}
