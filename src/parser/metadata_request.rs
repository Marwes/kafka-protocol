use super::*;
pub fn metadata_request<'i, I>() -> impl Parser<I, Output = MetadataRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        array(||string().expected("topics").expected("topics"),),
        any().map(|b| b != 0).expected("allow_auto_topic_creation"),
        any().map(|b| b != 0).expected("include_cluster_authorized_operations"),
        any().map(|b| b != 0).expected("include_topic_authorized_operations"),
    ).map(|(topics,allow_auto_topic_creation,include_cluster_authorized_operations,include_topic_authorized_operations,)| {MetadataRequest{
    topics,allow_auto_topic_creation,include_cluster_authorized_operations,include_topic_authorized_operations
    }})
}

#[derive(Clone, Debug, PartialEq)]
pub struct MetadataRequest<'i> {
    pub topics: Vec<&'i str>,
    pub allow_auto_topic_creation: bool,
    pub include_cluster_authorized_operations: bool,
    pub include_topic_authorized_operations: bool,
}

impl<'i> crate::Encode for MetadataRequest<'i> where {
    fn encode_len(&self) -> usize {
        self.topics.encode_len() + self.allow_auto_topic_creation.encode_len() + self.include_cluster_authorized_operations.encode_len() + self.include_topic_authorized_operations.encode_len()}
    fn encode(&self, writer: &mut impl Buffer) {
        self.topics.encode(writer);
        self.allow_auto_topic_creation.encode(writer);
        self.include_cluster_authorized_operations.encode(writer);
        self.include_topic_authorized_operations.encode(writer);}}

pub const VERSION: i16 = 0;








