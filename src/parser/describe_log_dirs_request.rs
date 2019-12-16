use super::*;
pub fn describe_log_dirs_request<'i, I>() -> impl Parser<I, Output = DescribeLogDirsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (array(|| {
        (string(), array(|| be_i32())).map(|(topic, partitions)| Topics { topic, partitions })
    }),)
        .map(|(topics,)| DescribeLogDirsRequest { topics })
}

#[derive(Clone, Debug, PartialEq)]
pub struct DescribeLogDirsRequest<'i> {
    pub topics: Vec<Topics<'i>>,
}

impl<'i> crate::Encode for DescribeLogDirsRequest<'i> {
    fn encode_len(&self) -> usize {
        self.topics.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.topics.encode(writer);
    }
}

pub const VERSION: i16 = 1;

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub topic: &'i str,
    pub partitions: Vec<i32>,
}

impl<'i> crate::Encode for Topics<'i> {
    fn encode_len(&self) -> usize {
        self.topic.encode_len() + self.partitions.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.topic.encode(writer);
        self.partitions.encode(writer);
    }
}
