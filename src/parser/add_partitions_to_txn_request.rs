use super::*;
pub fn add_partitions_to_txn_request<'i, I>(
) -> impl Parser<I, Output = AddPartitionsToTxnRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        string(),
        be_i64(),
        be_i16(),
        optional(
            (string(), optional(be_i32())).map(|(topic, partitions)| Topics { topic, partitions }),
        ),
    )
        .map(|(transactional_id, producer_id, producer_epoch, topics)| {
            AddPartitionsToTxnRequest {
                transactional_id,
                producer_id,
                producer_epoch,
                topics,
            }
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct AddPartitionsToTxnRequest<'i> {
    pub transactional_id: &'i str,
    pub producer_id: i64,
    pub producer_epoch: i16,
    pub topics: Option<Topics<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub topic: &'i str,
    pub partitions: Option<i32>,
}
