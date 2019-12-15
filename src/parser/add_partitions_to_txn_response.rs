use super::*;
pub fn add_partitions_to_txn_response<'i, I>(
) -> impl Parser<I, Output = AddPartitionsToTxnResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
            (
                string(),
                optional(
                    (be_i32(), be_i16()).map(|(partition, error_code)| PartitionErrors {
                        partition,
                        error_code,
                    }),
                ),
            )
                .map(|(topic, partition_errors)| Errors {
                    topic,
                    partition_errors,
                }),
        ),
    )
        .map(|(throttle_time_ms, errors)| AddPartitionsToTxnResponse {
            throttle_time_ms,
            errors,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct AddPartitionsToTxnResponse<'i> {
    pub throttle_time_ms: i32,
    pub errors: Option<Errors<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PartitionErrors {
    pub partition: i32,
    pub error_code: i16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Errors<'i> {
    pub topic: &'i str,
    pub partition_errors: Option<PartitionErrors>,
}
