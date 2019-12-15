use super::*;
pub fn add_offsets_to_txn_request<'i, I>() -> impl Parser<I, Output = AddOffsetsToTxnRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (string(), be_i64(), be_i16(), string()).map(
        |(transactional_id, producer_id, producer_epoch, group_id)| AddOffsetsToTxnRequest {
            transactional_id,
            producer_id,
            producer_epoch,
            group_id,
        },
    )
}

#[derive(Clone, Debug, PartialEq)]
pub struct AddOffsetsToTxnRequest<'i> {
    pub transactional_id: &'i str,
    pub producer_id: i64,
    pub producer_epoch: i16,
    pub group_id: &'i str,
}
