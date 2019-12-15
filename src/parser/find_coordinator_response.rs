use super::*;
pub fn find_coordinator_response<'i, I>() -> impl Parser<I, Output = FindCoordinatorResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i16(),
        nullable_string(),
        be_i32(),
        string(),
        be_i32(),
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
    pub error_code: i16,
    pub error_message: Option<&'i str>,
    pub node_id: i32,
    pub host: &'i str,
    pub port: i32,
}
