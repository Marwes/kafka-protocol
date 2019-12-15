use super::*;
pub fn delete_records_request<'i, I>() -> impl Parser<I, Output = DeleteRecordsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional(
            (
                string(),
                optional(
                    (be_i32(), be_i64())
                        .map(|(partition, offset)| Partitions { partition, offset }),
                ),
            )
                .map(|(topic, partitions)| Topics { topic, partitions }),
        ),
        be_i32(),
    )
        .map(|(topics, timeout)| DeleteRecordsRequest { topics, timeout })
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeleteRecordsRequest<'i> {
    pub topics: Option<Topics<'i>>,
    pub timeout: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub partition: i32,
    pub offset: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub topic: &'i str,
    pub partitions: Option<Partitions>,
}
