use super::*;
pub fn list_offsets_request<'i, I>() -> impl Parser<I, Output = ListOffsetsRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32().expected("replica_id"),
        array(||
        (
            string().expected("topic"),
            array(||
            (
                be_i32().expected("partition"),
                be_i32().expected("current_leader_epoch"),
                be_i64().expected("timestamp"),
            ).map(|(partition,current_leader_epoch,timestamp,)| {Partitions{
            partition,current_leader_epoch,timestamp
            }}).expected("partitions"),),
        ).map(|(topic,partitions,)| {Topics{
        topic,partitions
        }}).expected("topics"),),
        array(||
        (
            string().expected("topic"),
            array(||
            (
                be_i32().expected("partition"),
                be_i32().expected("current_leader_epoch"),
                be_i64().expected("timestamp"),
            ).map(|(partition,current_leader_epoch,timestamp,)| {Partitions{
            partition,current_leader_epoch,timestamp
            }}).expected("partitions"),),
        ).map(|(topic,partitions,)| {Topics{
        topic,partitions
        }}).expected("topics"),),
    ).map(|(replica_id,topics,topics,)| {ListOffsetsRequest{
    replica_id,topics,topics
    }})
}

#[derive(Clone, Debug, PartialEq)]
pub struct ListOffsetsRequest<'i> {
    pub replica_id: i32,
    pub topics: Vec<Topics<'i>>,
    pub topics: Vec<Topics<'i>>,
}

impl<'i> crate::Encode for ListOffsetsRequest<'i> where {
    fn encode_len(&self) -> usize {
        self.replica_id.encode_len() + self.topics.encode_len() + self.topics.encode_len()}
    fn encode(&self, writer: &mut impl Buffer) {
        self.replica_id.encode(writer);
        self.topics.encode(writer);
        self.topics.encode(writer);}}

pub const VERSION: i16 = 0;











#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub partition: i32,
    pub current_leader_epoch: i32,
    pub timestamp: i64,
}

impl crate::Encode for Partitions where {
    fn encode_len(&self) -> usize {
        self.partition.encode_len() + self.current_leader_epoch.encode_len() + self.timestamp.encode_len()}
    fn encode(&self, writer: &mut impl Buffer) {
        self.partition.encode(writer);
        self.current_leader_epoch.encode(writer);
        self.timestamp.encode(writer);}}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub topic: &'i str,
    pub partitions: Vec<Partitions>,
}

impl<'i> crate::Encode for Topics<'i> where {
    fn encode_len(&self) -> usize {
        self.topic.encode_len() + self.partitions.encode_len()}
    fn encode(&self, writer: &mut impl Buffer) {
        self.topic.encode(writer);
        self.partitions.encode(writer);}}









#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub partition: i32,
    pub current_leader_epoch: i32,
    pub timestamp: i64,
}

impl crate::Encode for Partitions where {
    fn encode_len(&self) -> usize {
        self.partition.encode_len() + self.current_leader_epoch.encode_len() + self.timestamp.encode_len()}
    fn encode(&self, writer: &mut impl Buffer) {
        self.partition.encode(writer);
        self.current_leader_epoch.encode(writer);
        self.timestamp.encode(writer);}}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub topic: &'i str,
    pub partitions: Vec<Partitions>,
}

impl<'i> crate::Encode for Topics<'i> where {
    fn encode_len(&self) -> usize {
        self.topic.encode_len() + self.partitions.encode_len()}
    fn encode(&self, writer: &mut impl Buffer) {
        self.topic.encode(writer);
        self.partitions.encode(writer);}}
