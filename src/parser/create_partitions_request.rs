use super::*;
pub fn create_partitions_request<'i, I>() -> impl Parser<I, Output = CreatePartitionsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional(
            (
                string(),
                (be_i32(), optional(bytes()))
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
    pub topic_partitions: Option<TopicPartitions<'i>>,
    pub timeout: i32,
    pub validate_only: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NewPartitions<'i> {
    pub count: i32,
    pub assignment: Option<&'i [u8]>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TopicPartitions<'i> {
    pub topic: &'i str,
    pub new_partitions: NewPartitions<'i>,
}
