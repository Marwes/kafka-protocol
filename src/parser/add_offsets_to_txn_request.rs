use super::*;
pub fn add_offsets_to_txn_request<'i, I>(
) -> impl Parser<I, Output = AddOffsetsToTxnRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
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

impl<'i> crate::Encode for AddOffsetsToTxnRequest<'i> {
    fn encode_len(&self) -> usize {
        self.transactional_id.encode_len()
            + self.producer_id.encode_len()
            + self.producer_epoch.encode_len()
            + self.group_id.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.transactional_id.encode(writer);
        self.producer_id.encode(writer);
        self.producer_epoch.encode(writer);
        self.group_id.encode(writer);
    }
}

pub const VERSION: i16 = 1;
