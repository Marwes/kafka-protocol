use super::*;
pub fn offset_for_leader_epoch_response<'i, I>(
) -> impl Parser<I, Output = OffsetForLeaderEpochResponse<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        array(|| {
            (
                string().expected("topic"),
                array(|| {
                    (
                        be_i16()
                            .and_then(|i| {
                                ErrorCode::try_from(i)
                                    .map_err(StreamErrorFor::<I>::unexpected_static_message)
                            })
                            .expected("error_code"),
                        be_i32().expected("partition"),
                        be_i32().expected("leader_epoch"),
                        be_i64().expected("end_offset"),
                    )
                        .map(
                            |(error_code, partition, leader_epoch, end_offset)| Partitions {
                                error_code,
                                partition,
                                leader_epoch,
                                end_offset,
                            },
                        )
                        .expected("partitions")
                }),
            )
                .map(|(topic, partitions)| Topics { topic, partitions })
                .expected("topics")
        }),
        array(|| {
            (
                string().expected("topic"),
                array(|| {
                    (
                        be_i16()
                            .and_then(|i| {
                                ErrorCode::try_from(i)
                                    .map_err(StreamErrorFor::<I>::unexpected_static_message)
                            })
                            .expected("error_code"),
                        be_i32().expected("partition"),
                        be_i32().expected("leader_epoch"),
                        be_i64().expected("end_offset"),
                    )
                        .map(
                            |(error_code, partition, leader_epoch, end_offset)| Partitions {
                                error_code,
                                partition,
                                leader_epoch,
                                end_offset,
                            },
                        )
                        .expected("partitions")
                }),
            )
                .map(|(topic, partitions)| Topics { topic, partitions })
                .expected("topics")
        }),
    )
        .map(|(topics, topics)| OffsetForLeaderEpochResponse { topics, topics })
}

#[derive(Clone, Debug, PartialEq)]
pub struct OffsetForLeaderEpochResponse<'i> {
    pub topics: Vec<Topics<'i>>,
    pub topics: Vec<Topics<'i>>,
}

impl<'i> crate::Encode for OffsetForLeaderEpochResponse<'i> {
    fn encode_len(&self) -> usize {
        self.topics.encode_len() + self.topics.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.topics.encode(writer);
        self.topics.encode(writer);
    }
}

pub const VERSION: i16 = 0;

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub error_code: ErrorCode,
    pub partition: i32,
    pub leader_epoch: i32,
    pub end_offset: i64,
}

impl crate::Encode for Partitions {
    fn encode_len(&self) -> usize {
        self.error_code.encode_len()
            + self.partition.encode_len()
            + self.leader_epoch.encode_len()
            + self.end_offset.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.error_code.encode(writer);
        self.partition.encode(writer);
        self.leader_epoch.encode(writer);
        self.end_offset.encode(writer);
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

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub error_code: ErrorCode,
    pub partition: i32,
    pub leader_epoch: i32,
    pub end_offset: i64,
}

impl crate::Encode for Partitions {
    fn encode_len(&self) -> usize {
        self.error_code.encode_len()
            + self.partition.encode_len()
            + self.leader_epoch.encode_len()
            + self.end_offset.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.error_code.encode(writer);
        self.partition.encode(writer);
        self.leader_epoch.encode(writer);
        self.end_offset.encode(writer);
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
