mod file_dump;
pub use file_dump::*;
mod machine_control;
pub use machine_control::*;
mod notation;
pub use notation::*;
pub use sample_dump::*;
mod sample_dump;
pub use sample_dump::*;
mod show_control;
pub use show_control::*;
mod tuning;
pub use tuning::*;

use super::time_code::*;
use super::util::*;

#[derive(Debug, Clone, PartialEq)]
pub enum SystemExclusiveMsg {
    Commercial {
        id: SysExID,
        data: Vec<u8>,
    },
    NonCommercial {
        data: Vec<u8>,
    },
    UniversalRealTime {
        device: DeviceID,
        msg: UniversalRealTimeMsg,
    },
    UniversalNonRealTime {
        device: DeviceID,
        msg: UniversalNonRealTimeMsg,
    },
}

impl SystemExclusiveMsg {
    pub fn to_midi(&self) -> Vec<u8> {
        let mut r: Vec<u8> = vec![];
        self.extend_midi(&mut r);
        r
    }

    pub fn extend_midi(&self, v: &mut Vec<u8>) {
        v.push(0xF0);
        match self {
            SystemExclusiveMsg::Commercial { id, data } => {
                id.extend_midi(v);
                data.iter().for_each(|d| v.push(to_u7(*d)));
            }
            SystemExclusiveMsg::NonCommercial { data } => {
                v.push(0x7D);
                data.iter().for_each(|d| v.push(to_u7(*d)));
            }
            SystemExclusiveMsg::UniversalRealTime { device, msg } => {
                v.push(0x7F);
                v.push(device.to_u8());
                msg.extend_midi(v);
            }
            SystemExclusiveMsg::UniversalNonRealTime { device, msg } => {
                v.push(0x7E);
                v.push(device.to_u8());
                msg.extend_midi(v);
            }
        }
        v.push(0xF7);
    }

    /// Ok results return a MidiMsg and the number of bytes consumed from the input
    pub fn from_midi(_m: &[u8]) -> Result<(Self, usize), &str> {
        Err("TODO: not implemented")
    }
}

impl From<&SystemExclusiveMsg> for Vec<u8> {
    fn from(m: &SystemExclusiveMsg) -> Vec<u8> {
        m.to_midi()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// If second byte is None, it is a one-byte ID
pub struct SysExID(u8, Option<u8>);

impl SysExID {
    fn extend_midi(&self, v: &mut Vec<u8>) {
        if let Some(second) = self.1 {
            v.push(0x00);
            v.push(to_u7(self.0));
            v.push(to_u7(second));
        } else {
            v.push(to_u7(self.0))
        }
    }
}

impl From<u8> for SysExID {
    fn from(a: u8) -> Self {
        Self(a, None)
    }
}

impl From<(u8, u8)> for SysExID {
    fn from((a, b): (u8, u8)) -> Self {
        Self(a, Some(b))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeviceID {
    AllCall,
    Device(u8),
}

impl DeviceID {
    fn to_u8(&self) -> u8 {
        match self {
            Self::AllCall => 0x7F,
            Self::Device(x) => to_u7(*x),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UniversalRealTimeMsg {
    TimeCodeFull(TimeCode),
    TimeCodeUserBits(UserBits),
    ShowControl(ShowControlMsg),
    BarMarker(BarMarker),
    TimeSignature(TimeSignature),
    TimeSignatureDelayed(TimeSignature),
    MasterVolume(u16),
    MasterBalance(u16),
    TimeCodeCueing(TimeCodeCueingMsg),
    MachineControlCommand(MachineControlCommandMsg),
    MachineControlResponse(MachineControlResponseMsg),
    TuningNoteChange(TuningNoteChange),
}

impl UniversalRealTimeMsg {
    fn extend_midi(&self, v: &mut Vec<u8>) {
        match self {
            UniversalRealTimeMsg::TimeCodeFull(code) => {
                v.push(01);
                v.push(01);
                let [frame, seconds, minutes, codehour] = code.to_bytes();
                v.extend_from_slice(&[codehour, minutes, seconds, frame]);
            }
            UniversalRealTimeMsg::TimeCodeUserBits(user_bits) => {
                v.push(01);
                v.push(02);
                let [ub1, ub2, ub3, ub4, ub5, ub6, ub7, ub8, ub9] = user_bits.to_nibbles();
                v.extend_from_slice(&[ub1, ub2, ub3, ub4, ub5, ub6, ub7, ub8, ub9]);
            }
            UniversalRealTimeMsg::ShowControl(msg) => {
                v.push(02);
                msg.extend_midi(v);
            }
            UniversalRealTimeMsg::BarMarker(marker) => {
                v.push(03);
                v.push(01);
                marker.extend_midi(v);
            }
            UniversalRealTimeMsg::TimeSignature(signature) => {
                v.push(03);
                v.push(02);
                signature.extend_midi(v);
            }
            UniversalRealTimeMsg::TimeSignatureDelayed(signature) => {
                v.push(03);
                v.push(42);
                signature.extend_midi(v);
            }
            UniversalRealTimeMsg::MasterVolume(vol) => {
                v.push(04);
                v.push(01);
                let [msb, lsb] = to_u14(*vol);
                v.push(lsb);
                v.push(msb);
            }
            UniversalRealTimeMsg::MasterBalance(bal) => {
                v.push(04);
                v.push(02);
                let [msb, lsb] = to_u14(*bal);
                v.push(lsb);
                v.push(msb);
            }
            UniversalRealTimeMsg::TimeCodeCueing(msg) => {
                v.push(05);
                msg.extend_midi(v);
            }
            UniversalRealTimeMsg::MachineControlCommand(msg) => {
                v.push(06);
                msg.extend_midi(v);
            }
            UniversalRealTimeMsg::MachineControlResponse(msg) => {
                v.push(07);
                msg.extend_midi(v);
            }
            UniversalRealTimeMsg::TuningNoteChange(note_change) => {
                v.push(08);
                v.push(02);
                note_change.extend_midi(v);
            }
        }
    }

    fn from_midi(_m: &[u8]) -> Result<(Self, usize), &str> {
        Err("TODO: not implemented")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UniversalNonRealTimeMsg {
    SampleDump(SampleDumpMsg),
    TimeCode(TimeCodeMsg),
    IdentityRequest,
    IdentityReply(IdentityReply),
    FileDump(FileDumpMsg),
    // Tuning program number, 0-127
    TuningBulkDumpRequest(u8),
    TuningBulkDumpReply(u8, TuningBulkDumpReply),
    GeneralMidi(bool),
    EOF,
    Wait,
    Cancel,
    NAK(u8),
    ACK(u8),
}

impl UniversalNonRealTimeMsg {
    fn extend_midi(&self, v: &mut Vec<u8>) {
        match self {
            UniversalNonRealTimeMsg::SampleDump(msg) => {
                match msg {
                    SampleDumpMsg::Header => v.push(01),
                    SampleDumpMsg::Packet => v.push(02),
                    SampleDumpMsg::Request => v.push(03),
                    SampleDumpMsg::MultipleLoopPoints => {
                        v.push(05);
                        v.push(01);
                    }
                    SampleDumpMsg::LoopPointsRequest => {
                        v.push(05);
                        v.push(02);
                    }
                }
                msg.extend_midi(v);
            }
            UniversalNonRealTimeMsg::TimeCode(msg) => {
                v.push(04);
                msg.extend_midi(v);
            }
            UniversalNonRealTimeMsg::IdentityRequest => {
                v.push(06);
                v.push(01);
            }
            UniversalNonRealTimeMsg::IdentityReply(identity) => {
                v.push(06);
                v.push(02);
                identity.extend_midi(v);
            }
            UniversalNonRealTimeMsg::FileDump(msg) => {
                v.push(07);
                msg.extend_midi(v);
            }
            UniversalNonRealTimeMsg::TuningBulkDumpRequest(program_num) => {
                v.push(08);
                v.push(00);
                v.push(to_u7(*program_num));
            }
            UniversalNonRealTimeMsg::TuningBulkDumpReply(program_num, tuning) => {
                v.push(08);
                v.push(01);
                v.push(to_u7(*program_num));
                tuning.extend_midi(v);
            }
            UniversalNonRealTimeMsg::GeneralMidi(on) => {
                v.push(09);
                v.push(if *on { 01 } else { 02 });
            }
            UniversalNonRealTimeMsg::EOF => {
                v.push(0x7B);
                v.push(00);
            }
            UniversalNonRealTimeMsg::Wait => {
                v.push(0x7C);
                v.push(00);
            }
            UniversalNonRealTimeMsg::Cancel => {
                v.push(0x7D);
                v.push(00);
            }
            UniversalNonRealTimeMsg::NAK(packet_num) => {
                v.push(0x7E);
                v.push(to_u7(*packet_num));
            }
            UniversalNonRealTimeMsg::ACK(packet_num) => {
                v.push(0x7F);
                v.push(to_u7(*packet_num));
            }
        }
    }

    fn from_midi(_m: &[u8]) -> Result<(Self, usize), &str> {
        Err("TODO: not implemented")
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct IdentityReply {
    // TODO
}

impl IdentityReply {
    fn extend_midi(&self, v: &mut Vec<u8>) {
        //TODO
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn serialize_system_exclusive_msg() {
        assert_eq!(
            MidiMsg::SystemExclusive {
                msg: SystemExclusiveMsg::Commercial {
                    id: 1.into(),
                    data: vec![0xff, 0x77, 0x00]
                }
            }
            .to_midi(),
            vec![0xF0, 0x01, 0x7F, 0x77, 0x00, 0xF7]
        );

        assert_eq!(
            MidiMsg::SystemExclusive {
                msg: SystemExclusiveMsg::Commercial {
                    id: (1, 3).into(),
                    data: vec![0xff, 0x77, 0x00]
                }
            }
            .to_midi(),
            vec![0xF0, 0x00, 0x01, 0x03, 0x7F, 0x77, 0x00, 0xF7]
        );

        assert_eq!(
            MidiMsg::SystemExclusive {
                msg: SystemExclusiveMsg::NonCommercial {
                    data: vec![0xff, 0x77, 0x00]
                }
            }
            .to_midi(),
            vec![0xF0, 0x7D, 0x7F, 0x77, 0x00, 0xF7]
        );

        assert_eq!(
            MidiMsg::SystemExclusive {
                msg: SystemExclusiveMsg::UniversalNonRealTime {
                    device: DeviceID::AllCall,
                    msg: UniversalNonRealTimeMsg::EOF
                }
            }
            .to_midi(),
            vec![0xF0, 0x7E, 0x7F, 0x7B, 0x00, 0xF7]
        );

        assert_eq!(
            MidiMsg::SystemExclusive {
                msg: SystemExclusiveMsg::UniversalRealTime {
                    device: DeviceID::Device(3),
                    msg: UniversalRealTimeMsg::MasterVolume(1000)
                }
            }
            .to_midi(),
            vec![0xF0, 0x7F, 0x03, 0x04, 0x01, 0x68, 0x07, 0xF7]
        );
    }
}
