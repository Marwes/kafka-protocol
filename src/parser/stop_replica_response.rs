use super::*;
pub fn stop_replica_response<'i, I>() -> impl Parser<I, Output = StopReplicaResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16(),
        optional(
            (string(), be_i32(), be_i16()).map(|(topic, partition, error_code)| Partitions {
                topic,
                partition,
                error_code,
            }),
        ),
    )
        .map(|(error_code, partitions)| StopReplicaResponse {
            error_code,
            partitions,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct StopReplicaResponse<'i> {
    pub error_code: i16,
    pub partitions: Option<Partitions<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions<'i> {
    pub topic: &'i str,
    pub partition: i32,
    pub error_code: i16,
}
