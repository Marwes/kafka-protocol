use super::*;
pub fn produce_request<'i, I>() -> impl Parser<I, Output = ProduceRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        nullable_string(),
        be_i16(),
        be_i32(),
        array(|| {
            (
                string(),
                array(|| {
                    (be_i32(), nullable_bytes()).map(|(partition, record_set)| Data {
                        partition,
                        record_set,
                    })
                }),
            )
                .map(|(topic, data)| TopicData { topic, data })
        }),
    )
        .map(
            |(transactional_id, acks, timeout, topic_data)| ProduceRequest {
                transactional_id,
                acks,
                timeout,
                topic_data,
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProduceRequest<'i> {
    pub transactional_id: Option<&'i str>,
    pub acks: i16,
    pub timeout: i32,
    pub topic_data: Vec<TopicData<'i>>,
}

impl<'i> crate::Encode for ProduceRequest<'i> {
    fn encode_len(&self) -> usize {
        self.transactional_id.encode_len()
            + self.acks.encode_len()
            + self.timeout.encode_len()
            + self.topic_data.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.transactional_id.encode(writer);
        self.acks.encode(writer);
        self.timeout.encode(writer);
        self.topic_data.encode(writer);
    }
}

pub const VERSION: i16 = 7;

#[derive(Clone, Debug, PartialEq)]
pub struct Data<'i> {
    pub partition: i32,
    pub record_set: Option<&'i [u8]>,
}

impl<'i> crate::Encode for Data<'i> {
    fn encode_len(&self) -> usize {
        self.partition.encode_len() + self.record_set.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.partition.encode(writer);
        self.record_set.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TopicData<'i> {
    pub topic: &'i str,
    pub data: Vec<Data<'i>>,
}

impl<'i> crate::Encode for TopicData<'i> {
    fn encode_len(&self) -> usize {
        self.topic.encode_len() + self.data.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.topic.encode(writer);
        self.data.encode(writer);
    }
}
