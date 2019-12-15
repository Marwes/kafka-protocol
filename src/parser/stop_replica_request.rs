use super::*;
pub fn stop_replica_request<'i, I>() -> impl Parser<I, Output = StopReplicaRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i32(),
        be_i64(),
        any().map(|b| b != 0),
        optional(
            (string(), optional(be_i32())).map(|(topic, partition_ids)| Partitions {
                topic,
                partition_ids,
            }),
        ),
    )
        .map(
            |(controller_id, controller_epoch, broker_epoch, delete_partitions, partitions)| {
                StopReplicaRequest {
                    controller_id,
                    controller_epoch,
                    broker_epoch,
                    delete_partitions,
                    partitions,
                }
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct StopReplicaRequest<'i> {
    pub controller_id: i32,
    pub controller_epoch: i32,
    pub broker_epoch: i64,
    pub delete_partitions: bool,
    pub partitions: Option<Partitions<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions<'i> {
    pub topic: &'i str,
    pub partition_ids: Option<i32>,
}
