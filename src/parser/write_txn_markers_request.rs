use super::*;
pub fn write_txn_markers_request<'i, I>() -> impl Parser<I, Output = WriteTxnMarkersRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (optional(
        (
            be_i64(),
            be_i16(),
            any().map(|b| b != 0),
            optional(
                (string(), optional(be_i32()))
                    .map(|(topic, partitions)| Topics { topic, partitions }),
            ),
            be_i32(),
        )
            .map(
                |(producer_id, producer_epoch, transaction_result, topics, coordinator_epoch)| {
                    TransactionMarkers {
                        producer_id,
                        producer_epoch,
                        transaction_result,
                        topics,
                        coordinator_epoch,
                    }
                },
            ),
    ),)
        .map(|(transaction_markers,)| WriteTxnMarkersRequest {
            transaction_markers,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct WriteTxnMarkersRequest<'i> {
    pub transaction_markers: Option<TransactionMarkers<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub topic: &'i str,
    pub partitions: Option<i32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TransactionMarkers<'i> {
    pub producer_id: i64,
    pub producer_epoch: i16,
    pub transaction_result: bool,
    pub topics: Option<Topics<'i>>,
    pub coordinator_epoch: i32,
}
