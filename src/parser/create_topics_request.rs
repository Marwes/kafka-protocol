use super::*;
pub fn create_topics_request<'i, I>() -> impl Parser<I, Output = CreateTopicsRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        array(|| {
            (
                string().expected("name"),
                be_i32().expected("num_partitions"),
                be_i16().expected("replication_factor"),
                array(|| {
                    (
                        be_i32().expected("partition_index"),
                        array(|| be_i32().expected("broker_ids").expected("broker_ids")),
                    )
                        .map(|(partition_index, broker_ids)| Assignments {
                            partition_index,
                            broker_ids,
                        })
                        .expected("assignments")
                }),
                array(|| {
                    (
                        string().expected("name"),
                        nullable_string().expected("value"),
                    )
                        .map(|(name, value)| Configs { name, value })
                        .expected("configs")
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
                .expected("topics")
        }),
        be_i32().expected("timeout_ms"),
        any().map(|b| b != 0).expected("validate_only"),
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
    fn encode(&self, writer: &mut impl Buffer) {
        self.topics.encode(writer);
        self.timeout_ms.encode(writer);
        self.validate_only.encode(writer);
    }
}

pub const VERSION: i16 = 0;

#[derive(Clone, Debug, PartialEq)]
pub struct Assignments {
    pub partition_index: i32,
    pub broker_ids: Vec<i32>,
}

impl crate::Encode for Assignments {
    fn encode_len(&self) -> usize {
        self.partition_index.encode_len() + self.broker_ids.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
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
    fn encode(&self, writer: &mut impl Buffer) {
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
    fn encode(&self, writer: &mut impl Buffer) {
        self.name.encode(writer);
        self.num_partitions.encode(writer);
        self.replication_factor.encode(writer);
        self.assignments.encode(writer);
        self.configs.encode(writer);
    }
}
