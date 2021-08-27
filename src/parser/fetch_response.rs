use super::*;
pub fn fetch_response<'i, R: RecordBatchParser<I> + 'i, I>() -> impl Parser<I, Output = FetchResponse<'i, R>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16().and_then(|i| ErrorCode::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message)).expected("error_code"),
        be_i32().expected("session_id"),
        array(||
        (
            string().expected("topic"),
            array(||
            (
                
                (
                    be_i32().expected("partition"),
                    be_i16().and_then(|i| ErrorCode::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message)).expected("error_code"),
                    be_i64().expected("high_watermark"),
                    be_i64().expected("last_stable_offset"),
                    be_i64().expected("log_start_offset"),
                    array(||
                    (
                        be_i64().expected("producer_id"),
                        be_i64().expected("first_offset"),
                    ).map(|(producer_id,first_offset,)| {AbortedTransactions{
                    producer_id,first_offset
                    }}).expected("aborted_transactions"),),
                    be_i32().expected("preferred_read_replica"),
                ).map(|(partition,error_code,high_watermark,last_stable_offset,log_start_offset,aborted_transactions,preferred_read_replica,)| {PartitionHeader{
                partition,error_code,high_watermark,last_stable_offset,log_start_offset,aborted_transactions,preferred_read_replica
                }}),
                R::parser().expected("record_set"),
            ).map(|(partition_header,record_set,)| {PartitionResponses{
            partition_header,record_set
            }}).expected("partition_responses"),),
        ).map(|(topic,partition_responses,)| {Responses{
        topic,partition_responses
        }}).expected("responses"),),
        array(||
        (
            string().expected("topic"),
            array(||
            (
                
                (
                    be_i32().expected("partition"),
                    be_i16().and_then(|i| ErrorCode::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message)).expected("error_code"),
                    be_i64().expected("high_watermark"),
                    be_i64().expected("last_stable_offset"),
                    be_i64().expected("log_start_offset"),
                    array(||
                    (
                        be_i64().expected("producer_id"),
                        be_i64().expected("first_offset"),
                    ).map(|(producer_id,first_offset,)| {AbortedTransactions{
                    producer_id,first_offset
                    }}).expected("aborted_transactions"),),
                    be_i32().expected("preferred_read_replica"),
                ).map(|(partition,error_code,high_watermark,last_stable_offset,log_start_offset,aborted_transactions,preferred_read_replica,)| {PartitionHeader{
                partition,error_code,high_watermark,last_stable_offset,log_start_offset,aborted_transactions,preferred_read_replica
                }}),
                R::parser().expected("record_set"),
            ).map(|(partition_header,record_set,)| {PartitionResponses{
            partition_header,record_set
            }}).expected("partition_responses"),),
        ).map(|(topic,partition_responses,)| {Responses{
        topic,partition_responses
        }}).expected("responses"),),
    ).map(|(error_code,session_id,responses,responses,)| {FetchResponse{
    error_code,session_id,responses,responses
    }})
}

#[derive(Clone, Debug, PartialEq)]
pub struct FetchResponse<'i, R> {
    pub error_code: ErrorCode,
    pub session_id: i32,
    pub responses: Vec<Responses<'i, R>>,
    pub responses: Vec<Responses<'i, R>>,
}

impl<'i, R> crate::Encode for FetchResponse<'i, R> where R: Encode,{
    fn encode_len(&self) -> usize {
        self.error_code.encode_len() + self.session_id.encode_len() + self.responses.encode_len() + self.responses.encode_len()}
    fn encode(&self, writer: &mut impl Buffer) {
        self.error_code.encode(writer);
        self.session_id.encode(writer);
        self.responses.encode(writer);
        self.responses.encode(writer);}}

pub const VERSION: i16 = 0;





















#[derive(Clone, Debug, PartialEq)]
pub struct AbortedTransactions {
    pub producer_id: i64,
    pub first_offset: i64,
}

impl crate::Encode for AbortedTransactions where {
    fn encode_len(&self) -> usize {
        self.producer_id.encode_len() + self.first_offset.encode_len()}
    fn encode(&self, writer: &mut impl Buffer) {
        self.producer_id.encode(writer);
        self.first_offset.encode(writer);}}



#[derive(Clone, Debug, PartialEq)]
pub struct PartitionHeader {
    pub partition: i32,
    pub error_code: ErrorCode,
    pub high_watermark: i64,
    pub last_stable_offset: i64,
    pub log_start_offset: i64,
    pub aborted_transactions: Vec<AbortedTransactions>,
    pub preferred_read_replica: i32,
}

impl crate::Encode for PartitionHeader where {
    fn encode_len(&self) -> usize {
        self.partition.encode_len() + self.error_code.encode_len() + self.high_watermark.encode_len() + self.last_stable_offset.encode_len() + self.log_start_offset.encode_len() + self.aborted_transactions.encode_len() + self.preferred_read_replica.encode_len()}
    fn encode(&self, writer: &mut impl Buffer) {
        self.partition.encode(writer);
        self.error_code.encode(writer);
        self.high_watermark.encode(writer);
        self.last_stable_offset.encode(writer);
        self.log_start_offset.encode(writer);
        self.aborted_transactions.encode(writer);
        self.preferred_read_replica.encode(writer);}}



#[derive(Clone, Debug, PartialEq)]
pub struct PartitionResponses<R> {
    pub partition_header: PartitionHeader,
    pub record_set: R,
}

impl<R> crate::Encode for PartitionResponses<R> where R: Encode,{
    fn encode_len(&self) -> usize {
        self.partition_header.encode_len() + self.record_set.encode_len()}
    fn encode(&self, writer: &mut impl Buffer) {
        self.partition_header.encode(writer);
        self.record_set.encode(writer);}}

#[derive(Clone, Debug, PartialEq)]
pub struct Responses<'i, R> {
    pub topic: &'i str,
    pub partition_responses: Vec<PartitionResponses<R>>,
}

impl<'i, R> crate::Encode for Responses<'i, R> where R: Encode,{
    fn encode_len(&self) -> usize {
        self.topic.encode_len() + self.partition_responses.encode_len()}
    fn encode(&self, writer: &mut impl Buffer) {
        self.topic.encode(writer);
        self.partition_responses.encode(writer);}}

















#[derive(Clone, Debug, PartialEq)]
pub struct AbortedTransactions {
    pub producer_id: i64,
    pub first_offset: i64,
}

impl crate::Encode for AbortedTransactions where {
    fn encode_len(&self) -> usize {
        self.producer_id.encode_len() + self.first_offset.encode_len()}
    fn encode(&self, writer: &mut impl Buffer) {
        self.producer_id.encode(writer);
        self.first_offset.encode(writer);}}



#[derive(Clone, Debug, PartialEq)]
pub struct PartitionHeader {
    pub partition: i32,
    pub error_code: ErrorCode,
    pub high_watermark: i64,
    pub last_stable_offset: i64,
    pub log_start_offset: i64,
    pub aborted_transactions: Vec<AbortedTransactions>,
    pub preferred_read_replica: i32,
}

impl crate::Encode for PartitionHeader where {
    fn encode_len(&self) -> usize {
        self.partition.encode_len() + self.error_code.encode_len() + self.high_watermark.encode_len() + self.last_stable_offset.encode_len() + self.log_start_offset.encode_len() + self.aborted_transactions.encode_len() + self.preferred_read_replica.encode_len()}
    fn encode(&self, writer: &mut impl Buffer) {
        self.partition.encode(writer);
        self.error_code.encode(writer);
        self.high_watermark.encode(writer);
        self.last_stable_offset.encode(writer);
        self.log_start_offset.encode(writer);
        self.aborted_transactions.encode(writer);
        self.preferred_read_replica.encode(writer);}}



#[derive(Clone, Debug, PartialEq)]
pub struct PartitionResponses<R> {
    pub partition_header: PartitionHeader,
    pub record_set: R,
}

impl<R> crate::Encode for PartitionResponses<R> where R: Encode,{
    fn encode_len(&self) -> usize {
        self.partition_header.encode_len() + self.record_set.encode_len()}
    fn encode(&self, writer: &mut impl Buffer) {
        self.partition_header.encode(writer);
        self.record_set.encode(writer);}}

#[derive(Clone, Debug, PartialEq)]
pub struct Responses<'i, R> {
    pub topic: &'i str,
    pub partition_responses: Vec<PartitionResponses<R>>,
}

impl<'i, R> crate::Encode for Responses<'i, R> where R: Encode,{
    fn encode_len(&self) -> usize {
        self.topic.encode_len() + self.partition_responses.encode_len()}
    fn encode(&self, writer: &mut impl Buffer) {
        self.topic.encode(writer);
        self.partition_responses.encode(writer);}}
