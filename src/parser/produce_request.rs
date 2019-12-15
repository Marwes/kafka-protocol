use super::*;
pub fn produce_request<'i, I>() -> impl Parser<I, Output = ProduceRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        nullable_string(),
        be_i16(),
        be_i32(),
        optional(
            (
                string(),
                optional(
                    (be_i32(), nullable_bytes()).map(|(partition, record_set)| Data {
                        partition,
                        record_set,
                    }),
                ),
            )
                .map(|(topic, data)| TopicData { topic, data }),
        ),
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
    pub topic_data: Option<TopicData<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Data<'i> {
    pub partition: i32,
    pub record_set: Option<&'i [u8]>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TopicData<'i> {
    pub topic: &'i str,
    pub data: Option<Data<'i>>,
}
