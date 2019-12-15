use super::*;
pub fn metadata_request<'i, I>() -> impl Parser<I, Output = MetadataRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional((string(),).map(|(name,)| Topics { name })),
        any().map(|b| b != 0),
        any().map(|b| b != 0),
        any().map(|b| b != 0),
    )
        .map(
            |(
                topics,
                allow_auto_topic_creation,
                include_cluster_authorized_operations,
                include_topic_authorized_operations,
            )| {
                MetadataRequest {
                    topics,
                    allow_auto_topic_creation,
                    include_cluster_authorized_operations,
                    include_topic_authorized_operations,
                }
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct MetadataRequest<'i> {
    pub topics: Option<Topics<'i>>,
    pub allow_auto_topic_creation: bool,
    pub include_cluster_authorized_operations: bool,
    pub include_topic_authorized_operations: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub name: &'i str,
}
