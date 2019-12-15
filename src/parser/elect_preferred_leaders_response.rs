use super::*;
pub fn elect_preferred_leaders_response<'i, I>(
) -> impl Parser<I, Output = ElectPreferredLeadersResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
            (
                string(),
                optional((be_i32(), be_i16(), nullable_string()).map(
                    |(partition_id, error_code, error_message)| PartitionResult {
                        partition_id,
                        error_code,
                        error_message,
                    },
                )),
            )
                .map(|(topic, partition_result)| ReplicaElectionResults {
                    topic,
                    partition_result,
                }),
        ),
    )
        .map(
            |(throttle_time_ms, replica_election_results)| ElectPreferredLeadersResponse {
                throttle_time_ms,
                replica_election_results,
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct ElectPreferredLeadersResponse<'i> {
    pub throttle_time_ms: i32,
    pub replica_election_results: Option<ReplicaElectionResults<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartitionResult<'i> {
    pub partition_id: i32,
    pub error_code: i16,
    pub error_message: Option<&'i str>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReplicaElectionResults<'i> {
    pub topic: &'i str,
    pub partition_result: Option<PartitionResult<'i>>,
}
