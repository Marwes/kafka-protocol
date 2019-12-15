use super::*;
pub fn create_partitions_request<'i, I>() -> impl Parser<I, Output = CreatePartitionsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        many(
            (
                string(),
                (be_i32(), many(bytes()))
                    .map(|(count, assignment)| NewPartitions { count, assignment }),
            )
                .map(|(topic, new_partitions)| TopicPartitions {
                    topic,
                    new_partitions,
                }),
        ),
        be_i32(),
        any().map(|b| b != 0),
    )
        .map(
            |(topic_partitions, timeout, validate_only)| CreatePartitionsRequest {
                topic_partitions,
                timeout,
                validate_only,
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreatePartitionsRequest<'i> {
    pub topic_partitions: Vec<TopicPartitions<'i>>,
    pub timeout: i32,
    pub validate_only: bool,
}

impl<'i> crate::Encode for CreatePartitionsRequest<'i> {
    fn encode_len(&self) -> usize {
        self.topic_partitions.encode_len()
            + self.timeout.encode_len()
            + self.validate_only.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.topic_partitions.encode(writer);
        self.timeout.encode(writer);
        self.validate_only.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NewPartitions<'i> {
    pub count: i32,
    pub assignment: Vec<&'i [u8]>,
}

impl<'i> crate::Encode for NewPartitions<'i> {
    fn encode_len(&self) -> usize {
        self.count.encode_len() + self.assignment.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.count.encode(writer);
        self.assignment.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TopicPartitions<'i> {
    pub topic: &'i str,
    pub new_partitions: NewPartitions<'i>,
}

impl<'i> crate::Encode for TopicPartitions<'i> {
    fn encode_len(&self) -> usize {
        self.topic.encode_len() + self.new_partitions.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.topic.encode(writer);
        self.new_partitions.encode(writer);
    }
}
