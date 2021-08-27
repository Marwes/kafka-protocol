use super::*;
pub fn delete_topics_request<'i, I>() -> impl Parser<I, Output = DeleteTopicsRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        array(|| string().expected("topic_names").expected("topic_names")),
        be_i32().expected("timeout_ms"),
    )
        .map(|(topic_names, timeout_ms)| DeleteTopicsRequest {
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
    fn encode(&self, writer: &mut impl Buffer) {
        self.topic_names.encode(writer);
        self.timeout_ms.encode(writer);
    }
}

pub const VERSION: i16 = 0;
