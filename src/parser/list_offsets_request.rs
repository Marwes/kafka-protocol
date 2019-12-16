use super::*;
pub fn list_offsets_request<'i, I>() -> impl Parser<I, Output = ListOffsetsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i8(),
        array(|| {
            (
                string(),
                array(|| {
                    (be_i32(), be_i32(), be_i64()).map(
                        |(partition, current_leader_epoch, timestamp)| Partitions {
                            partition,
                            current_leader_epoch,
                            timestamp,
                        },
                    )
                }),
            )
                .map(|(topic, partitions)| Topics { topic, partitions })
        }),
    )
        .map(|(replica_id, isolation_level, topics)| ListOffsetsRequest {
            replica_id,
            isolation_level,
            topics,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListOffsetsRequest<'i> {
    pub replica_id: i32,
    pub isolation_level: i8,
    pub topics: Vec<Topics<'i>>,
}

impl<'i> crate::Encode for ListOffsetsRequest<'i> {
    fn encode_len(&self) -> usize {
        self.replica_id.encode_len() + self.isolation_level.encode_len() + self.topics.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.replica_id.encode(writer);
        self.isolation_level.encode(writer);
        self.topics.encode(writer);
    }
}

pub const VERSION: i16 = 5;

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub partition: i32,
    pub current_leader_epoch: i32,
    pub timestamp: i64,
}

impl crate::Encode for Partitions {
    fn encode_len(&self) -> usize {
        self.partition.encode_len()
            + self.current_leader_epoch.encode_len()
            + self.timestamp.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.partition.encode(writer);
        self.current_leader_epoch.encode(writer);
        self.timestamp.encode(writer);
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
