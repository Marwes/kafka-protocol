use super::*;
pub fn write_txn_markers_response<'i, I>() -> impl Parser<I, Output = WriteTxnMarkersResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (many(
        (
            be_i64(),
            many(
                (
                    string(),
                    many(
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
    pub transaction_markers: Vec<TransactionMarkers<'i>>,
}

impl<'i> crate::Encode for WriteTxnMarkersResponse<'i> {
    fn encode_len(&self) -> usize {
        self.transaction_markers.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.transaction_markers.encode(writer);
    }
}

pub const VERSION: i16 = 0;

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub partition: i32,
    pub error_code: i16,
}

impl crate::Encode for Partitions {
    fn encode_len(&self) -> usize {
        self.partition.encode_len() + self.error_code.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.partition.encode(writer);
        self.error_code.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub topic: &'i str,
    pub partitions: Vec<Partitions>,
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

#[derive(Clone, Debug, PartialEq)]
pub struct TransactionMarkers<'i> {
    pub producer_id: i64,
    pub topics: Vec<Topics<'i>>,
}

impl<'i> crate::Encode for TransactionMarkers<'i> {
    fn encode_len(&self) -> usize {
        self.producer_id.encode_len() + self.topics.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.producer_id.encode(writer);
        self.topics.encode(writer);
    }
}
