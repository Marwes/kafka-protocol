use super::*;
pub fn add_partitions_to_txn_request<'i, I>(
) -> impl Parser<I, Output = AddPartitionsToTxnRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        string(),
        be_i64(),
        be_i16(),
        array(|| {
            (string(), array(|| be_i32())).map(|(topic, partitions)| Topics { topic, partitions })
        }),
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
    pub topics: Vec<Topics<'i>>,
}

impl<'i> crate::Encode for AddPartitionsToTxnRequest<'i> {
    fn encode_len(&self) -> usize {
        self.transactional_id.encode_len()
            + self.producer_id.encode_len()
            + self.producer_epoch.encode_len()
            + self.topics.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.transactional_id.encode(writer);
        self.producer_id.encode(writer);
        self.producer_epoch.encode(writer);
        self.topics.encode(writer);
    }
}

pub const VERSION: i16 = 1;

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub topic: &'i str,
    pub partitions: Vec<i32>,
}

impl<'i> crate::Encode for Topics<'i> {
    fn encode_len(&self) -> usize {
        self.topic.encode_len() + self.partitions.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.topic.encode(writer);
        self.partitions.encode(writer);
    }
}
