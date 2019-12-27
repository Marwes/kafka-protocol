use super::*;
pub fn txn_offset_commit_request<'i, I>() -> impl Parser<I, Output = TxnOffsetCommitRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        string().expected("transactional_id"),
        string().expected("group_id"),
        be_i64().expected("producer_id"),
        be_i16().expected("producer_epoch"),
        array(|| {
            (
                string().expected("topic"),
                array(|| {
                    (
                        be_i32().expected("partition"),
                        be_i64().expected("offset"),
                        be_i32().expected("leader_epoch"),
                        nullable_string().expected("metadata"),
                    )
                        .map(|(partition, offset, leader_epoch, metadata)| Partitions {
                            partition,
                            offset,
                            leader_epoch,
                            metadata,
                        })
                        .expected("partitions")
                }),
            )
                .map(|(topic, partitions)| Topics { topic, partitions })
                .expected("topics")
        }),
    )
        .map(
            |(transactional_id, group_id, producer_id, producer_epoch, topics)| {
                TxnOffsetCommitRequest {
                    transactional_id,
                    group_id,
                    producer_id,
                    producer_epoch,
                    topics,
                }
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct TxnOffsetCommitRequest<'i> {
    pub transactional_id: &'i str,
    pub group_id: &'i str,
    pub producer_id: i64,
    pub producer_epoch: i16,
    pub topics: Vec<Topics<'i>>,
}

impl<'i> crate::Encode for TxnOffsetCommitRequest<'i> {
    fn encode_len(&self) -> usize {
        self.transactional_id.encode_len()
            + self.group_id.encode_len()
            + self.producer_id.encode_len()
            + self.producer_epoch.encode_len()
            + self.topics.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.transactional_id.encode(writer);
        self.group_id.encode(writer);
        self.producer_id.encode(writer);
        self.producer_epoch.encode(writer);
        self.topics.encode(writer);
    }
}

pub const VERSION: i16 = 2;

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions<'i> {
    pub partition: i32,
    pub offset: i64,
    pub leader_epoch: i32,
    pub metadata: Option<&'i str>,
}

impl<'i> crate::Encode for Partitions<'i> {
    fn encode_len(&self) -> usize {
        self.partition.encode_len()
            + self.offset.encode_len()
            + self.leader_epoch.encode_len()
            + self.metadata.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.partition.encode(writer);
        self.offset.encode(writer);
        self.leader_epoch.encode(writer);
        self.metadata.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub topic: &'i str,
    pub partitions: Vec<Partitions<'i>>,
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
