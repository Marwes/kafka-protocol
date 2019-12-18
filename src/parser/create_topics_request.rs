use super::*;
pub fn create_topics_request<'i, I>() -> impl Parser<I, Output = CreateTopicsRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        array(|| {
            (
                string(),
                be_i32(),
                be_i16(),
                array(|| {
                    (be_i32(), array(|| be_i32())).map(|(partition_index, broker_ids)| {
                        Assignments {
                            partition_index,
                            broker_ids,
                        }
                    })
                }),
                array(|| {
                    (string(), nullable_string()).map(|(name, value)| Configs { name, value })
                }),
            )
                .map(
                    |(name, num_partitions, replication_factor, assignments, configs)| Topics {
                        name,
                        num_partitions,
                        replication_factor,
                        assignments,
                        configs,
                    },
                )
        }),
        be_i32(),
        any().map(|b| b != 0),
    )
        .map(|(topics, timeout_ms, validate_only)| CreateTopicsRequest {
            topics,
            timeout_ms,
            validate_only,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateTopicsRequest<'i> {
    pub topics: Vec<Topics<'i>>,
    pub timeout_ms: i32,
    pub validate_only: bool,
}

impl<'i> crate::Encode for CreateTopicsRequest<'i> {
    fn encode_len(&self) -> usize {
        self.topics.encode_len() + self.timeout_ms.encode_len() + self.validate_only.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.topics.encode(writer);
        self.timeout_ms.encode(writer);
        self.validate_only.encode(writer);
    }
}

pub const VERSION: i16 = 3;

#[derive(Clone, Debug, PartialEq)]
pub struct Assignments {
    pub partition_index: i32,
    pub broker_ids: Vec<i32>,
}

impl crate::Encode for Assignments {
    fn encode_len(&self) -> usize {
        self.partition_index.encode_len() + self.broker_ids.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.partition_index.encode(writer);
        self.broker_ids.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Configs<'i> {
    pub name: &'i str,
    pub value: Option<&'i str>,
}

impl<'i> crate::Encode for Configs<'i> {
    fn encode_len(&self) -> usize {
        self.name.encode_len() + self.value.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.name.encode(writer);
        self.value.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub name: &'i str,
    pub num_partitions: i32,
    pub replication_factor: i16,
    pub assignments: Vec<Assignments>,
    pub configs: Vec<Configs<'i>>,
}

impl<'i> crate::Encode for Topics<'i> {
    fn encode_len(&self) -> usize {
        self.name.encode_len()
            + self.num_partitions.encode_len()
            + self.replication_factor.encode_len()
            + self.assignments.encode_len()
            + self.configs.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.name.encode(writer);
        self.num_partitions.encode(writer);
        self.replication_factor.encode(writer);
        self.assignments.encode(writer);
        self.configs.encode(writer);
    }
}
