use super::*;
pub fn offset_fetch_request<'i, I>() -> impl Parser<I, Output = OffsetFetchRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        string().expected("group_id"),
        array(|| {
            (
                string().expected("topic"),
                array(|| {
                    (be_i32().expected("partition"),)
                        .map(|(partition,)| Partitions { partition })
                        .expected("partitions")
                }),
            )
                .map(|(topic, partitions)| Topics { topic, partitions })
                .expected("topics")
        }),
    )
        .map(|(group_id, topics)| OffsetFetchRequest { group_id, topics })
}

#[derive(Clone, Debug, PartialEq)]
pub struct OffsetFetchRequest<'i> {
    pub group_id: &'i str,
    pub topics: Vec<Topics<'i>>,
}

impl<'i> crate::Encode for OffsetFetchRequest<'i> {
    fn encode_len(&self) -> usize {
        self.group_id.encode_len() + self.topics.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.group_id.encode(writer);
        self.topics.encode(writer);
    }
}

pub const VERSION: i16 = 5;

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub partition: i32,
}

impl crate::Encode for Partitions {
    fn encode_len(&self) -> usize {
        self.partition.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.partition.encode(writer);
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
