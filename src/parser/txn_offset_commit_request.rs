use super::*;
pub fn txn_offset_commit_request<'i, I>() -> impl Parser<I, Output = TxnOffsetCommitRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        string(),
        string(),
        be_i64(),
        be_i16(),
        many(
            (
                string(),
                many((be_i32(), be_i64(), be_i32(), nullable_string()).map(
                    |(partition, offset, leader_epoch, metadata)| Partitions {
                        partition,
                        offset,
                        leader_epoch,
                        metadata,
                    },
                )),
            )
                .map(|(topic, partitions)| Topics { topic, partitions }),
        ),
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
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.transactional_id.encode(writer);
        self.group_id.encode(writer);
        self.producer_id.encode(writer);
        self.producer_epoch.encode(writer);
        self.topics.encode(writer);
    }
}

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
    fn encode(&self, writer: &mut impl bytes::BufMut) {
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
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.topic.encode(writer);
        self.partitions.encode(writer);
    }
}
