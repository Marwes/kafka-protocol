use super::*;
pub fn controlled_shutdown_request<'i, I>(
) -> impl Parser<I, Output = ControlledShutdownRequest> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32().expected("broker_id"),
        be_i64().expected("broker_epoch"),
    )
        .map(|(broker_id, broker_epoch)| ControlledShutdownRequest {
            broker_id,
            broker_epoch,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct ControlledShutdownRequest {
    pub broker_id: i32,
    pub broker_epoch: i64,
}

impl crate::Encode for ControlledShutdownRequest {
    fn encode_len(&self) -> usize {
        self.broker_id.encode_len() + self.broker_epoch.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.broker_id.encode(writer);
        self.broker_epoch.encode(writer);
    }
}

pub const VERSION: i16 = 2;
