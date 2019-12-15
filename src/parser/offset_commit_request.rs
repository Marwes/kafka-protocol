use super::*;
pub fn offset_commit_request<'i, I>() -> impl Parser<I, Output = OffsetCommitRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        string(),
        be_i32(),
        string(),
        nullable_string(),
        many(
            (
                string(),
                many((be_i32(), be_i64(), be_i32(), nullable_string()).map(
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
                )),
            )
                .map(|(name, partitions)| Topics { name, partitions }),
        ),
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
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.group_id.encode(writer);
        self.generation_id.encode(writer);
        self.member_id.encode(writer);
        self.group_instance_id.encode(writer);
        self.topics.encode(writer);
    }
}

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
    fn encode(&self, writer: &mut impl bytes::BufMut) {
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
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.name.encode(writer);
        self.partitions.encode(writer);
    }
}
