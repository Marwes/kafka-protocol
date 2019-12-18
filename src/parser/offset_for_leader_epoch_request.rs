use super::*;
pub fn offset_for_leader_epoch_request<'i, I>(
) -> impl Parser<I, Output = OffsetForLeaderEpochRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        array(|| {
            (
                string(),
                array(|| {
                    (be_i32(), be_i32(), be_i32()).map(
                        |(partition, current_leader_epoch, leader_epoch)| Partitions {
                            partition,
                            current_leader_epoch,
                            leader_epoch,
                        },
                    )
                }),
            )
                .map(|(topic, partitions)| Topics { topic, partitions })
        }),
    )
        .map(|(replica_id, topics)| OffsetForLeaderEpochRequest { replica_id, topics })
}

#[derive(Clone, Debug, PartialEq)]
pub struct OffsetForLeaderEpochRequest<'i> {
    pub replica_id: i32,
    pub topics: Vec<Topics<'i>>,
}

impl<'i> crate::Encode for OffsetForLeaderEpochRequest<'i> {
    fn encode_len(&self) -> usize {
        self.replica_id.encode_len() + self.topics.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.replica_id.encode(writer);
        self.topics.encode(writer);
    }
}

pub const VERSION: i16 = 3;

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub partition: i32,
    pub current_leader_epoch: i32,
    pub leader_epoch: i32,
}

impl crate::Encode for Partitions {
    fn encode_len(&self) -> usize {
        self.partition.encode_len()
            + self.current_leader_epoch.encode_len()
            + self.leader_epoch.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.partition.encode(writer);
        self.current_leader_epoch.encode(writer);
        self.leader_epoch.encode(writer);
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
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.topic.encode(writer);
        self.partitions.encode(writer);
    }
}
