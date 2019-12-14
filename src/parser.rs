use super::*;
fn produce_request<'i, I>() -> impl Parser<I, Output = ProduceRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16(),
        be_i32(),
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
                nullable_bytes(),
            ).map(|(partition,record_set)| {Data{
            partition,record_set
            }}))
        ).map(|(topic,data)| {TopicData{
        topic,data
        }}))
    ).map(|(acks,timeout,topic_data)| {ProduceRequest{
    acks,timeout,topic_data
    }})
}

pub struct ProduceRequest<'i> {

    acks: i16,
    timeout: i32,
    topic_data: Option<TopicData<'i>>
}pub struct Data<'i> {

    partition: i32,
    record_set: Option<&'i [u8]>,
}
pub struct TopicData<'i> {

    topic: &'i str,
    data: Option<Data<'i>>
}
use super::*;
fn produce_request<'i, I>() -> impl Parser<I, Output = ProduceRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16(),
        be_i32(),
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
                nullable_bytes(),
            ).map(|(partition,record_set)| {Data{
            partition,record_set
            }}))
        ).map(|(topic,data)| {TopicData{
        topic,data
        }}))
    ).map(|(acks,timeout,topic_data)| {ProduceRequest{
    acks,timeout,topic_data
    }})
}

pub struct ProduceRequest<'i> {

    acks: i16,
    timeout: i32,
    topic_data: Option<TopicData<'i>>
}pub struct Data<'i> {

    partition: i32,
    record_set: Option<&'i [u8]>,
}
pub struct TopicData<'i> {

    topic: &'i str,
    data: Option<Data<'i>>
}
use super::*;
fn produce_request<'i, I>() -> impl Parser<I, Output = ProduceRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16(),
        be_i32(),
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
                nullable_bytes(),
            ).map(|(partition,record_set)| {Data{
            partition,record_set
            }}))
        ).map(|(topic,data)| {TopicData{
        topic,data
        }}))
    ).map(|(acks,timeout,topic_data)| {ProduceRequest{
    acks,timeout,topic_data
    }})
}

pub struct ProduceRequest<'i> {

    acks: i16,
    timeout: i32,
    topic_data: Option<TopicData<'i>>
}pub struct Data<'i> {

    partition: i32,
    record_set: Option<&'i [u8]>,
}
pub struct TopicData<'i> {

    topic: &'i str,
    data: Option<Data<'i>>
}
use super::*;
fn produce_request<'i, I>() -> impl Parser<I, Output = ProduceRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        nullable_string(),
        be_i16(),
        be_i32(),
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
                nullable_bytes(),
            ).map(|(partition,record_set)| {Data{
            partition,record_set
            }}))
        ).map(|(topic,data)| {TopicData{
        topic,data
        }}))
    ).map(|(transactional_id,acks,timeout,topic_data)| {ProduceRequest{
    transactional_id,acks,timeout,topic_data
    }})
}

pub struct ProduceRequest<'i> {

    transactional_id: Option<&'i str>,
    acks: i16,
    timeout: i32,
    topic_data: Option<TopicData<'i>>
}pub struct Data<'i> {

    partition: i32,
    record_set: Option<&'i [u8]>,
}
pub struct TopicData<'i> {

    topic: &'i str,
    data: Option<Data<'i>>
}
use super::*;
fn produce_request<'i, I>() -> impl Parser<I, Output = ProduceRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        nullable_string(),
        be_i16(),
        be_i32(),
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
                nullable_bytes(),
            ).map(|(partition,record_set)| {Data{
            partition,record_set
            }}))
        ).map(|(topic,data)| {TopicData{
        topic,data
        }}))
    ).map(|(transactional_id,acks,timeout,topic_data)| {ProduceRequest{
    transactional_id,acks,timeout,topic_data
    }})
}

pub struct ProduceRequest<'i> {

    transactional_id: Option<&'i str>,
    acks: i16,
    timeout: i32,
    topic_data: Option<TopicData<'i>>
}pub struct Data<'i> {

    partition: i32,
    record_set: Option<&'i [u8]>,
}
pub struct TopicData<'i> {

    topic: &'i str,
    data: Option<Data<'i>>
}
use super::*;
fn produce_request<'i, I>() -> impl Parser<I, Output = ProduceRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        nullable_string(),
        be_i16(),
        be_i32(),
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
                nullable_bytes(),
            ).map(|(partition,record_set)| {Data{
            partition,record_set
            }}))
        ).map(|(topic,data)| {TopicData{
        topic,data
        }}))
    ).map(|(transactional_id,acks,timeout,topic_data)| {ProduceRequest{
    transactional_id,acks,timeout,topic_data
    }})
}

pub struct ProduceRequest<'i> {

    transactional_id: Option<&'i str>,
    acks: i16,
    timeout: i32,
    topic_data: Option<TopicData<'i>>
}pub struct Data<'i> {

    partition: i32,
    record_set: Option<&'i [u8]>,
}
pub struct TopicData<'i> {

    topic: &'i str,
    data: Option<Data<'i>>
}
use super::*;
fn produce_request<'i, I>() -> impl Parser<I, Output = ProduceRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        nullable_string(),
        be_i16(),
        be_i32(),
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
                nullable_bytes(),
            ).map(|(partition,record_set)| {Data{
            partition,record_set
            }}))
        ).map(|(topic,data)| {TopicData{
        topic,data
        }}))
    ).map(|(transactional_id,acks,timeout,topic_data)| {ProduceRequest{
    transactional_id,acks,timeout,topic_data
    }})
}

pub struct ProduceRequest<'i> {

    transactional_id: Option<&'i str>,
    acks: i16,
    timeout: i32,
    topic_data: Option<TopicData<'i>>
}pub struct Data<'i> {

    partition: i32,
    record_set: Option<&'i [u8]>,
}
pub struct TopicData<'i> {

    topic: &'i str,
    data: Option<Data<'i>>
}
use super::*;
fn produce_request<'i, I>() -> impl Parser<I, Output = ProduceRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        nullable_string(),
        be_i16(),
        be_i32(),
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
                nullable_bytes(),
            ).map(|(partition,record_set)| {Data{
            partition,record_set
            }}))
        ).map(|(topic,data)| {TopicData{
        topic,data
        }}))
    ).map(|(transactional_id,acks,timeout,topic_data)| {ProduceRequest{
    transactional_id,acks,timeout,topic_data
    }})
}

pub struct ProduceRequest<'i> {

    transactional_id: Option<&'i str>,
    acks: i16,
    timeout: i32,
    topic_data: Option<TopicData<'i>>
}pub struct Data<'i> {

    partition: i32,
    record_set: Option<&'i [u8]>,
}
pub struct TopicData<'i> {

    topic: &'i str,
    data: Option<Data<'i>>
}
