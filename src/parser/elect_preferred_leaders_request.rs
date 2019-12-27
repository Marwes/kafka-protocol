use super::*;
pub fn elect_preferred_leaders_request<'i, I>(
) -> impl Parser<I, Output = ElectPreferredLeadersRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        array(|| {
            (
                string().expected("topic"),
                array(|| be_i32().expected("partition_id")),
            )
                .map(|(topic, partition_id)| TopicPartitions {
                    topic,
                    partition_id,
                })
                .expected("topic_partitions")
        }),
        be_i32().expected("timeout_ms"),
    )
        .map(
            |(topic_partitions, timeout_ms)| ElectPreferredLeadersRequest {
                topic_partitions,
                timeout_ms,
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct ElectPreferredLeadersRequest<'i> {
    pub topic_partitions: Vec<TopicPartitions<'i>>,
    pub timeout_ms: i32,
}

impl<'i> crate::Encode for ElectPreferredLeadersRequest<'i> {
    fn encode_len(&self) -> usize {
        self.topic_partitions.encode_len() + self.timeout_ms.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.topic_partitions.encode(writer);
        self.timeout_ms.encode(writer);
    }
}

pub const VERSION: i16 = 0;

#[derive(Clone, Debug, PartialEq)]
pub struct TopicPartitions<'i> {
    pub topic: &'i str,
    pub partition_id: Vec<i32>,
}

impl<'i> crate::Encode for TopicPartitions<'i> {
    fn encode_len(&self) -> usize {
        self.topic.encode_len() + self.partition_id.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.topic.encode(writer);
        self.partition_id.encode(writer);
    }
}
