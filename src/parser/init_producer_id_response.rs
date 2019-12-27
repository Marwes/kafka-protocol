use super::*;
pub fn init_producer_id_response<'i, I>() -> impl Parser<I, Output = InitProducerIdResponse> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32().expected("throttle_time_ms"),
        be_i16()
            .and_then(|i| {
                ErrorCode::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message)
            })
            .expected("error_code"),
        be_i64().expected("producer_id"),
        be_i16().expected("producer_epoch"),
    )
        .map(
            |(throttle_time_ms, error_code, producer_id, producer_epoch)| InitProducerIdResponse {
                throttle_time_ms,
                error_code,
                producer_id,
                producer_epoch,
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct InitProducerIdResponse {
    pub throttle_time_ms: i32,
    pub error_code: ErrorCode,
    pub producer_id: i64,
    pub producer_epoch: i16,
}

impl crate::Encode for InitProducerIdResponse {
    fn encode_len(&self) -> usize {
        self.throttle_time_ms.encode_len()
            + self.error_code.encode_len()
            + self.producer_id.encode_len()
            + self.producer_epoch.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.throttle_time_ms.encode(writer);
        self.error_code.encode(writer);
        self.producer_id.encode(writer);
        self.producer_epoch.encode(writer);
    }
}

pub const VERSION: i16 = 1;
