use super::*;
pub fn elect_preferred_leaders_response<'i, I>(
) -> impl Parser<I, Output = ElectPreferredLeadersResponse<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32().expected("throttle_time_ms"),
        array(|| {
            (
                string().expected("topic"),
                array(|| {
                    (
                        be_i32().expected("partition_id"),
                        be_i16()
                            .and_then(|i| {
                                ErrorCode::try_from(i)
                                    .map_err(StreamErrorFor::<I>::unexpected_static_message)
                            })
                            .expected("error_code"),
                        nullable_string().expected("error_message"),
                    )
                        .map(
                            |(partition_id, error_code, error_message)| PartitionResult {
                                partition_id,
                                error_code,
                                error_message,
                            },
                        )
                        .expected("partition_result")
                }),
            )
                .map(|(topic, partition_result)| ReplicaElectionResults {
                    topic,
                    partition_result,
                })
                .expected("replica_election_results")
        }),
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
    pub replica_election_results: Vec<ReplicaElectionResults<'i>>,
}

impl<'i> crate::Encode for ElectPreferredLeadersResponse<'i> {
    fn encode_len(&self) -> usize {
        self.throttle_time_ms.encode_len() + self.replica_election_results.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.throttle_time_ms.encode(writer);
        self.replica_election_results.encode(writer);
    }
}

pub const VERSION: i16 = 0;

#[derive(Clone, Debug, PartialEq)]
pub struct PartitionResult<'i> {
    pub partition_id: i32,
    pub error_code: ErrorCode,
    pub error_message: Option<&'i str>,
}

impl<'i> crate::Encode for PartitionResult<'i> {
    fn encode_len(&self) -> usize {
        self.partition_id.encode_len()
            + self.error_code.encode_len()
            + self.error_message.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.partition_id.encode(writer);
        self.error_code.encode(writer);
        self.error_message.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReplicaElectionResults<'i> {
    pub topic: &'i str,
    pub partition_result: Vec<PartitionResult<'i>>,
}

impl<'i> crate::Encode for ReplicaElectionResults<'i> {
    fn encode_len(&self) -> usize {
        self.topic.encode_len() + self.partition_result.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.topic.encode(writer);
        self.partition_result.encode(writer);
    }
}
