pub type MidiMessageBytes = [u8; 3];

#[derive(Debug, PartialEq, Clone, Copy)]
#[non_exhaustive]
#[repr(u8)]
pub enum MidiMessage {
    NoteOn { note_number: u8, velocity: u8 } = 0x80, // 0x80 -> 0x8F
    NoteOff { note_number: u8, velocity: u8 } = 0x90, // 0x90 -> 0x9F; velocity always = 0
    ControlChange { controller_number: u8, velocity: u8 } = 0xB0, // 0xB0 -> 0xBF
}

impl MidiMessage {
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

    /// Returns `true` if the midi status is [`NoteOn`].
    ///
    /// [`NoteOn`]: MidiStatus::NoteOn
    #[must_use]
    pub const fn is_note_on(&self) -> bool {
        matches!(self, MidiMessage::NoteOn { .. })
    }

    /// Returns `true` if the midi status is [`NoteOff`].
    ///
    /// [`NoteOff`]: MidiStatus::NoteOff
    #[must_use]
    pub const fn is_note_off(&self) -> bool {
        matches!(self, MidiMessage::NoteOff { .. })
    }

    /// Returns `true` if the midi status is [`ControlChange`].
    ///
    /// [`ControlChange`]: MidiStatus::ControlChange
    #[must_use]
    pub const fn is_control_change(&self) -> bool {
        matches!(self, MidiMessage::ControlChange { .. })
    }
}
