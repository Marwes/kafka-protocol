use super::*;
pub fn end_txn_request<'i, I>() -> impl Parser<I, Output = EndTxnRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (string(), be_i64(), be_i16(), any().map(|b| b != 0)).map(
        |(transactional_id, producer_id, producer_epoch, transaction_result)| EndTxnRequest {
            transactional_id,
            producer_id,
            producer_epoch,
            transaction_result,
        },
    )
}

#[derive(Clone, Debug, PartialEq)]
pub struct EndTxnRequest<'i> {
    pub transactional_id: &'i str,
    pub producer_id: i64,
    pub producer_epoch: i16,
    pub transaction_result: bool,
}
