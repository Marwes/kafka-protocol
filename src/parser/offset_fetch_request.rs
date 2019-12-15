use super::*;
pub fn offset_fetch_request<'i, I>() -> impl Parser<I, Output = OffsetFetchRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        string(),
        optional(
            (
                string(),
                optional((be_i32(),).map(|(partition,)| Partitions { partition })),
            )
                .map(|(topic, partitions)| Topics { topic, partitions }),
        ),
    )
        .map(|(group_id, topics)| OffsetFetchRequest { group_id, topics })
}

#[derive(Clone, Debug, PartialEq)]
pub struct OffsetFetchRequest<'i> {
    pub group_id: &'i str,
    pub topics: Option<Topics<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub partition: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub topic: &'i str,
    pub partitions: Option<Partitions>,
}
