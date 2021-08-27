use super::*;
pub fn add_partitions_to_txn_response<'i, I>(
) -> impl Parser<I, Output = AddPartitionsToTxnResponse<'i>> + 'i
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
                        be_i32().expected("partition"),
                        be_i16()
                            .and_then(|i| {
                                ErrorCode::try_from(i)
                                    .map_err(StreamErrorFor::<I>::unexpected_static_message)
                            })
                            .expected("error_code"),
                    )
                        .map(|(partition, error_code)| PartitionErrors {
                            partition,
                            error_code,
                        })
                        .expected("partition_errors")
                }),
            )
                .map(|(topic, partition_errors)| Errors {
                    topic,
                    partition_errors,
                })
                .expected("errors")
        }),
    )
        .map(|(throttle_time_ms, errors)| AddPartitionsToTxnResponse {
            throttle_time_ms,
            errors,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct AddPartitionsToTxnResponse<'i> {
    pub throttle_time_ms: i32,
    pub errors: Vec<Errors<'i>>,
}

impl<'i> crate::Encode for AddPartitionsToTxnResponse<'i> {
    fn encode_len(&self) -> usize {
        self.throttle_time_ms.encode_len() + self.errors.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.throttle_time_ms.encode(writer);
        self.errors.encode(writer);
    }
}

pub const VERSION: i16 = 0;

#[derive(Clone, Debug, PartialEq)]
pub struct PartitionErrors {
    pub partition: i32,
    pub error_code: ErrorCode,
}

impl crate::Encode for PartitionErrors {
    fn encode_len(&self) -> usize {
        self.partition.encode_len() + self.error_code.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.partition.encode(writer);
        self.error_code.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Errors<'i> {
    pub topic: &'i str,
    pub partition_errors: Vec<PartitionErrors>,
}

impl<'i> crate::Encode for Errors<'i> {
    fn encode_len(&self) -> usize {
        self.topic.encode_len() + self.partition_errors.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.topic.encode(writer);
        self.partition_errors.encode(writer);
    }
}
