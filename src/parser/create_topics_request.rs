use super::*;
pub fn create_topics_request<'i, I>() -> impl Parser<I, Output = CreateTopicsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional(
            (
                string(),
                be_i32(),
                be_i16(),
                optional(
                    (be_i32(), optional(be_i32())).map(|(partition_index, broker_ids)| {
                        Assignments {
                            partition_index,
                            broker_ids,
                        }
                    }),
                ),
                optional(
                    (string(), nullable_string()).map(|(name, value)| Configs { name, value }),
                ),
            )
                .map(
                    |(name, num_partitions, replication_factor, assignments, configs)| Topics {
                        name,
                        num_partitions,
                        replication_factor,
                        assignments,
                        configs,
                    },
                ),
        ),
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
    pub topics: Option<Topics<'i>>,
    pub timeout_ms: i32,
    pub validate_only: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Assignments {
    pub partition_index: i32,
    pub broker_ids: Option<i32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Configs<'i> {
    pub name: &'i str,
    pub value: Option<&'i str>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub name: &'i str,
    pub num_partitions: i32,
    pub replication_factor: i16,
    pub assignments: Option<Assignments>,
    pub configs: Option<Configs<'i>>,
}
