use super::*;
pub fn offset_commit_request<'i, I>() -> impl Parser<I, Output = OffsetCommitRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        string().expected("group_id"),
        be_i32().expected("generation_id"),
        string().expected("member_id"),
        nullable_string().expected("group_instance_id"),
        array(|| {
            (
                string().expected("name"),
                array(|| {
                    (
                        be_i32().expected("partition_index"),
                        be_i64().expected("committed_offset"),
                        be_i32().expected("committed_leader_epoch"),
                        nullable_string().expected("committed_metadata"),
                    )
                        .map(
                            |(
                                partition_index,
                                committed_offset,
                                committed_leader_epoch,
                                committed_metadata,
                            )| {
                                Partitions {
                                    partition_index,
                                    committed_offset,
                                    committed_leader_epoch,
                                    committed_metadata,
                                }
                            },
                        )
                        .expected("partitions")
                }),
            )
                .map(|(name, partitions)| Topics { name, partitions })
                .expected("topics")
        }),
    )
        .map(
            |(group_id, generation_id, member_id, group_instance_id, topics)| OffsetCommitRequest {
                group_id,
                generation_id,
                member_id,
                group_instance_id,
                topics,
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct OffsetCommitRequest<'i> {
    pub group_id: &'i str,
    pub generation_id: i32,
    pub member_id: &'i str,
    pub group_instance_id: Option<&'i str>,
    pub topics: Vec<Topics<'i>>,
}

impl<'i> crate::Encode for OffsetCommitRequest<'i> {
    fn encode_len(&self) -> usize {
        self.group_id.encode_len()
            + self.generation_id.encode_len()
            + self.member_id.encode_len()
            + self.group_instance_id.encode_len()
            + self.topics.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.group_id.encode(writer);
        self.generation_id.encode(writer);
        self.member_id.encode(writer);
        self.group_instance_id.encode(writer);
        self.topics.encode(writer);
    }
}

pub const VERSION: i16 = 7;

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions<'i> {
    pub partition_index: i32,
    pub committed_offset: i64,
    pub committed_leader_epoch: i32,
    pub committed_metadata: Option<&'i str>,
}

impl<'i> crate::Encode for Partitions<'i> {
    fn encode_len(&self) -> usize {
        self.partition_index.encode_len()
            + self.committed_offset.encode_len()
            + self.committed_leader_epoch.encode_len()
            + self.committed_metadata.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.partition_index.encode(writer);
        self.committed_offset.encode(writer);
        self.committed_leader_epoch.encode(writer);
        self.committed_metadata.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub name: &'i str,
    pub partitions: Vec<Partitions<'i>>,
}

impl<'i> crate::Encode for Topics<'i> {
    fn encode_len(&self) -> usize {
        self.name.encode_len() + self.partitions.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.name.encode(writer);
        self.partitions.encode(writer);
    }
}
