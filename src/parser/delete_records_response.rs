use super::*;
pub fn delete_records_response<'i, I>() -> impl Parser<I, Output = DeleteRecordsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
            (
                string(),
                optional((be_i32(), be_i64(), be_i16()).map(
                    |(partition, low_watermark, error_code)| Partitions {
                        partition,
                        low_watermark,
                        error_code,
                    },
                )),
            )
                .map(|(topic, partitions)| Topics { topic, partitions }),
        ),
    )
        .map(|(throttle_time_ms, topics)| DeleteRecordsResponse {
            throttle_time_ms,
            topics,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeleteRecordsResponse<'i> {
    pub throttle_time_ms: i32,
    pub topics: Option<Topics<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub partition: i32,
    pub low_watermark: i64,
    pub error_code: i16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub topic: &'i str,
    pub partitions: Option<Partitions>,
}
