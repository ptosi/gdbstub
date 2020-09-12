use super::prelude::*;

#[derive(PartialEq, Eq, Debug)]
pub struct QStartNoAckMode;

impl<'a> ParseCommand<'a> for QStartNoAckMode {
    fn from_packet(buf: PacketBuf<'a>) -> Option<Self> {
        if !buf.into_body().is_empty() {
            return None;
        }
        Some(QStartNoAckMode)
    }
}
