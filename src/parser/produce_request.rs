use super::*;
pub fn produce_request<'i, R: RecordBatchParser<I> + 'i, I>(
) -> impl Parser<I, Output = ProduceRequest<'i, R>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        nullable_string().expected("transactional_id"),
        be_i16()
            .and_then(|i| Acks::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message))
            .expected("acks"),
        be_i32().expected("timeout"),
        array(|| {
            (
                string().expected("topic"),
                array(|| {
                    (
                        be_i32().expected("partition"),
                        R::parser().expected("record_set"),
                    )
                        .map(|(partition, record_set)| Data {
                            partition,
                            record_set,
                        })
                        .expected("data")
                }),
            )
                .map(|(topic, data)| TopicData { topic, data })
                .expected("topic_data")
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
pub struct ProduceRequest<'i, R> {
    pub transactional_id: Option<&'i str>,
    pub acks: Acks,
    pub timeout: i32,
    pub topic_data: Vec<TopicData<'i, R>>,
}

impl<'i, R> crate::Encode for ProduceRequest<'i, R>
where
    R: Encode,
{
    fn encode_len(&self) -> usize {
        self.transactional_id.encode_len()
            + self.acks.encode_len()
            + self.timeout.encode_len()
            + self.topic_data.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.transactional_id.encode(writer);
        self.acks.encode(writer);
        self.timeout.encode(writer);
        self.topic_data.encode(writer);
    }
}

pub const VERSION: i16 = 7;

#[derive(Clone, Debug, PartialEq)]
pub struct Data<R> {
    pub partition: i32,
    pub record_set: Option<RecordBatch<R>>,
}

impl<R> crate::Encode for Data<R>
where
    R: Encode,
{
    fn encode_len(&self) -> usize {
        self.partition.encode_len() + self.record_set.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.partition.encode(writer);
        self.record_set.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TopicData<'i, R> {
    pub topic: &'i str,
    pub data: Vec<Data<R>>,
}

impl<'i, R> crate::Encode for TopicData<'i, R>
where
    R: Encode,
{
    fn encode_len(&self) -> usize {
        self.topic.encode_len() + self.data.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.topic.encode(writer);
        self.data.encode(writer);
    }
}
