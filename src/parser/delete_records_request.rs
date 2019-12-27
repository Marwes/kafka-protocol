use super::*;
pub fn delete_records_request<'i, I>() -> impl Parser<I, Output = DeleteRecordsRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        array(|| {
            (
                string().expected("topic"),
                array(|| {
                    (be_i32().expected("partition"), be_i64().expected("offset"))
                        .map(|(partition, offset)| Partitions { partition, offset })
                        .expected("partitions")
                }),
            )
                .map(|(topic, partitions)| Topics { topic, partitions })
                .expected("topics")
        }),
        be_i32().expected("timeout"),
    )
        .map(|(topics, timeout)| DeleteRecordsRequest { topics, timeout })
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeleteRecordsRequest<'i> {
    pub topics: Vec<Topics<'i>>,
    pub timeout: i32,
}

impl<'i> crate::Encode for DeleteRecordsRequest<'i> {
    fn encode_len(&self) -> usize {
        self.topics.encode_len() + self.timeout.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.topics.encode(writer);
        self.timeout.encode(writer);
    }
}

pub const VERSION: i16 = 1;

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub partition: i32,
    pub offset: i64,
}

impl crate::Encode for Partitions {
    fn encode_len(&self) -> usize {
        self.partition.encode_len() + self.offset.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.partition.encode(writer);
        self.offset.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub topic: &'i str,
    pub partitions: Vec<Partitions>,
}

impl<'i> crate::Encode for Topics<'i> {
    fn encode_len(&self) -> usize {
        self.topic.encode_len() + self.partitions.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.topic.encode(writer);
        self.partitions.encode(writer);
    }
}
