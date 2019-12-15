use super::*;
pub fn create_topics_response<'i, I>() -> impl Parser<I, Output = CreateTopicsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional((string(), be_i16(), nullable_string()).map(
            |(name, error_code, error_message)| Topics {
                name,
                error_code,
                error_message,
            },
        )),
    )
        .map(|(throttle_time_ms, topics)| CreateTopicsResponse {
            throttle_time_ms,
            topics,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateTopicsResponse<'i> {
    pub throttle_time_ms: i32,
    pub topics: Option<Topics<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub name: &'i str,
    pub error_code: i16,
    pub error_message: Option<&'i str>,
}
