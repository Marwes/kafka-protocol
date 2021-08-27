use super::*;
pub fn stop_replica_request<'i, I>() -> impl Parser<I, Output = StopReplicaRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32().expected("controller_id"),
        be_i32().expected("controller_epoch"),
        any().map(|b| b != 0).expected("delete_partitions"),
        array(|| {
            (
                string().expected("topic"),
                array(|| be_i32().expected("partition_ids").expected("partition_ids")),
            )
                .map(|(topic, partition_ids)| Partitions {
                    topic,
                    partition_ids,
                })
                .expected("partitions")
        }),
        array(|| {
            (
                string().expected("topic"),
                array(|| be_i32().expected("partition_ids").expected("partition_ids")),
            )
                .map(|(topic, partition_ids)| Partitions {
                    topic,
                    partition_ids,
                })
                .expected("partitions")
        }),
    )
        .map(
            |(controller_id, controller_epoch, delete_partitions, partitions, partitions)| {
                StopReplicaRequest {
                    controller_id,
                    controller_epoch,
                    delete_partitions,
                    partitions,
                    partitions,
                }
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct StopReplicaRequest<'i> {
    pub controller_id: i32,
    pub controller_epoch: i32,
    pub delete_partitions: bool,
    pub partitions: Vec<Partitions<'i>>,
    pub partitions: Vec<Partitions<'i>>,
}

impl<'i> crate::Encode for StopReplicaRequest<'i> {
    fn encode_len(&self) -> usize {
        self.controller_id.encode_len()
            + self.controller_epoch.encode_len()
            + self.delete_partitions.encode_len()
            + self.partitions.encode_len()
            + self.partitions.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.controller_id.encode(writer);
        self.controller_epoch.encode(writer);
        self.delete_partitions.encode(writer);
        self.partitions.encode(writer);
        self.partitions.encode(writer);
    }
}

pub const VERSION: i16 = 0;

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions<'i> {
    pub topic: &'i str,
    pub partition_ids: Vec<i32>,
}

impl<'i> crate::Encode for Partitions<'i> {
    fn encode_len(&self) -> usize {
        self.topic.encode_len() + self.partition_ids.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.topic.encode(writer);
        self.partition_ids.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions<'i> {
    pub topic: &'i str,
    pub partition_ids: Vec<i32>,
}

impl<'i> crate::Encode for Partitions<'i> {
    fn encode_len(&self) -> usize {
        self.topic.encode_len() + self.partition_ids.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.topic.encode(writer);
        self.partition_ids.encode(writer);
    }
}
