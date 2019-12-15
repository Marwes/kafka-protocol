use super::*;
pub fn delete_topics_request<'i, I>() -> impl Parser<I, Output = DeleteTopicsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (optional(string()), be_i32()).map(|(topic_names, timeout_ms)| DeleteTopicsRequest {
        topic_names,
        timeout_ms,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeleteTopicsRequest<'i> {
    pub topic_names: Option<&'i str>,
    pub timeout_ms: i32,
}
