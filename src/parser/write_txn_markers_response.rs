use super::*;
pub fn write_txn_markers_response<'i, I>() -> impl Parser<I, Output = WriteTxnMarkersResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (optional(
        (
            be_i64(),
            optional(
                (
                    string(),
                    optional(
                        (be_i32(), be_i16()).map(|(partition, error_code)| Partitions {
                            partition,
                            error_code,
                        }),
                    ),
                )
                    .map(|(topic, partitions)| Topics { topic, partitions }),
            ),
        )
            .map(|(producer_id, topics)| TransactionMarkers {
                producer_id,
                topics,
            }),
    ),)
        .map(|(transaction_markers,)| WriteTxnMarkersResponse {
            transaction_markers,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct WriteTxnMarkersResponse<'i> {
    pub transaction_markers: Option<TransactionMarkers<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub partition: i32,
    pub error_code: i16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub topic: &'i str,
    pub partitions: Option<Partitions>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TransactionMarkers<'i> {
    pub producer_id: i64,
    pub topics: Option<Topics<'i>>,
}
