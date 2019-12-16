use super::*;
pub fn write_txn_markers_request<'i, I>() -> impl Parser<I, Output = WriteTxnMarkersRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (array(|| {
        (
            be_i64(),
            be_i16(),
            any().map(|b| b != 0),
            array(|| {
                (string(), array(|| be_i32()))
                    .map(|(topic, partitions)| Topics { topic, partitions })
            }),
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
            )
    }),)
        .map(|(transaction_markers,)| WriteTxnMarkersRequest {
            transaction_markers,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct WriteTxnMarkersRequest<'i> {
    pub transaction_markers: Vec<TransactionMarkers<'i>>,
}

impl<'i> crate::Encode for WriteTxnMarkersRequest<'i> {
    fn encode_len(&self) -> usize {
        self.transaction_markers.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.transaction_markers.encode(writer);
    }
}

pub const VERSION: i16 = 0;

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

#[derive(Clone, Debug, PartialEq)]
pub struct TransactionMarkers<'i> {
    pub producer_id: i64,
    pub producer_epoch: i16,
    pub transaction_result: bool,
    pub topics: Vec<Topics<'i>>,
    pub coordinator_epoch: i32,
}

impl<'i> crate::Encode for TransactionMarkers<'i> {
    fn encode_len(&self) -> usize {
        self.producer_id.encode_len()
            + self.producer_epoch.encode_len()
            + self.transaction_result.encode_len()
            + self.topics.encode_len()
            + self.coordinator_epoch.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.producer_id.encode(writer);
        self.producer_epoch.encode(writer);
        self.transaction_result.encode(writer);
        self.topics.encode(writer);
        self.coordinator_epoch.encode(writer);
    }
}
