use super::*;
pub fn find_coordinator_response<'i, I>(
) -> impl Parser<I, Output = FindCoordinatorResponse<'i>> + 'i
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
        nullable_string().expected("error_message"),
        be_i32().expected("node_id"),
        string().expected("host"),
        be_i32().expected("port"),
    )
        .map(
            |(throttle_time_ms, error_code, error_message, node_id, host, port)| {
                FindCoordinatorResponse {
                    throttle_time_ms,
                    error_code,
                    error_message,
                    node_id,
                    host,
                    port,
                }
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct FindCoordinatorResponse<'i> {
    pub throttle_time_ms: i32,
    pub error_code: ErrorCode,
    pub error_message: Option<&'i str>,
    pub node_id: i32,
    pub host: &'i str,
    pub port: i32,
}

impl<'i> crate::Encode for FindCoordinatorResponse<'i> {
    fn encode_len(&self) -> usize {
        self.throttle_time_ms.encode_len()
            + self.error_code.encode_len()
            + self.error_message.encode_len()
            + self.node_id.encode_len()
            + self.host.encode_len()
            + self.port.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.throttle_time_ms.encode(writer);
        self.error_code.encode(writer);
        self.error_message.encode(writer);
        self.node_id.encode(writer);
        self.host.encode(writer);
        self.port.encode(writer);
    }
}

pub const VERSION: i16 = 2;
