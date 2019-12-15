use super::*;
pub fn create_partitions_response<'i, I>() -> impl Parser<I, Output = CreatePartitionsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional((string(), be_i16(), nullable_string()).map(
            |(topic, error_code, error_message)| TopicErrors {
                topic,
                error_code,
                error_message,
            },
        )),
    )
        .map(
            |(throttle_time_ms, topic_errors)| CreatePartitionsResponse {
                throttle_time_ms,
                topic_errors,
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreatePartitionsResponse<'i> {
    pub throttle_time_ms: i32,
    pub topic_errors: Option<TopicErrors<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TopicErrors<'i> {
    pub topic: &'i str,
    pub error_code: i16,
    pub error_message: Option<&'i str>,
}
