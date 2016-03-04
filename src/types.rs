use std::os::raw::c_int;
use std::fmt::{Display, Formatter, Error as FormatError};
use std::error::Error as StdError;
use std::convert::{From, Into};
use std::result;

use ffi;

pub type PortMidiDeviceId = i32;

pub type Result<T> = result::Result<T, Error>;
impl From<ffi::PmError> for Result<()> {
    fn from(err: ffi::PmError) -> Self {
        match err {
            ffi::PmError::PmNoError | ffi::PmError::PmGotData => Ok(()),
            _ => Err(Error::PortMidi(err)),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    PortMidi(ffi::PmError),
    Unknown,
    Unimplemented,
}
impl From<ffi::PmError> for Error {
    fn from(err: ffi::PmError) -> Self {
        match err {
            err @ _ => Error::PortMidi(err),
        }
    }
}

// Midi events
// -----------
/// Represents a single midi message, see also `MidiEvent`
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MidiMessage {
    pub status: u8,
    pub data1: u8,
    pub data2: u8,
}
impl From<[u8; 3]> for MidiMessage {
    fn from(raw: [u8; 3]) -> Self {
        MidiMessage {
            status: raw[0],
            data1: raw[1],
            data2: raw[2],
        }
    }
}
/// This can be used for `c_int` as well as `ffi::PmMessage` because these are only type aliases
impl From<i32> for MidiMessage {
    fn from(raw: i32) -> Self {
        MidiMessage {
            status: ((raw & 0x00_FF_00_00) >> 16) as u8,
            data1: ((raw & 0x00_00_FF_00) >> 8) as u8,
            data2: (raw & 0x00_00_00_FF) as u8,
        }
    }
}
impl Into<i32> for MidiMessage {
    fn into(self) -> i32 {
        (((self.data2 as i32) << 16)) | (((self.data1 as i32) << 8)) | self.status as i32
    }
}

/// Represents a time stamped midi event. See also `MidiMessage`
///
/// See the PortMidi documentation for how SysEx and midi realtime messages
/// are handled
///
/// TODO: what to do about the timestamp?
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MidiEvent {
    pub message: MidiMessage,
    pub timestamp: ffi::PmTimestamp,
}

impl MidiEvent {
    pub fn wrap(event: ffi::PmEvent) -> MidiEvent {
        MidiEvent {
            message: MidiMessage::from(event.message),
            timestamp: event.timestamp,
        }
    }

    pub fn unwrap(&self) -> ffi::PmEvent {
        ffi::PmEvent {
            message: MidiMessage::into(self.message),
            timestamp: self.timestamp,
        }
    }
}
