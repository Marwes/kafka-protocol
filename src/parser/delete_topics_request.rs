use super::*;
pub fn delete_topics_request<'i, I>() -> impl Parser<I, Output = DeleteTopicsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (many(string()), be_i32()).map(|(topic_names, timeout_ms)| DeleteTopicsRequest {
        topic_names,
        timeout_ms,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeleteTopicsRequest<'i> {
    pub topic_names: Vec<&'i str>,
    pub timeout_ms: i32,
}

impl<'i> crate::Encode for DeleteTopicsRequest<'i> {
    fn encode_len(&self) -> usize {
        self.topic_names.encode_len() + self.timeout_ms.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.topic_names.encode(writer);
        self.timeout_ms.encode(writer);
    }
}
