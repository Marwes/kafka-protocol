use std::{convert::TryFrom, io, mem, str, time::Duration};

use {
    bytes::Buf,
    combine::{
        error::{ParseError, StreamError},
        parser::{
            byte::num::{be_i16, be_i32, be_i64},
            range,
            token::{any, position, value},
        },
        stream::StreamErrorFor,
        Parser, RangeStream,
    },
    integer_encoding::VarInt,
    tokio::io::{AsyncRead, AsyncWrite},
};

use {api_key::ApiKey, client::Client, error::ErrorCode};

#[macro_use]
extern crate quick_error;

pub mod api_key;
pub mod client;
pub mod error;
pub mod parser;
pub mod producer;

pub type Result<T, E = Error> = std::result::Result<T, E>;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
            display("{}", err)
            from()
        }
        InvalidTimeout(dur: Duration) {
            display("Duration to large to be converted to a millisecond timeout: {:?}", dur)
            from()
        }
    }
}

fn be_i8<'i, I>() -> impl Parser<I, Output = i8>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    any().map(|b| b as i8)
}

fn varint<'i, I>() -> impl Parser<I, Output = i32>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    combine::parser::function::parser(move |input: &mut I| {
        let range = input.range();
        let (value, out) = i32::decode_var(range);
        combine::stream::uncons_range(input, out).into_result()?;
        Ok((value, combine::error::Commit::Commit(())))
    })
}

fn vararray<'i, I, P>(mut elem_parser: impl FnMut() -> P) -> impl Parser<I, Output = Vec<P::Output>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
    P: Parser<I>,
{
    varint().then_partial(move |&mut len| {
        if let Ok(len) = usize::try_from(len) {
            combine::parser::repeat::count_min_max(len, len, elem_parser()).left()
        } else {
            combine::parser::function::parser(|_| {
                Ok((Vec::new(), combine::error::Commit::Peek(())))
            })
            .right()
        }
    })
}

fn varstring<'i, I>() -> impl Parser<I, Output = &'i str>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    varbytes().and_then(|bs| str::from_utf8(bs).map_err(StreamErrorFor::<I>::other))
}

fn varbytes<'i, I>() -> impl Parser<I, Output = &'i [u8]>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    varint()
        .and_then(|len| usize::try_from(len).map_err(StreamErrorFor::<I>::other))
        .then_partial(|&mut len| range::take(len))
}

fn string<'i, I>() -> impl Parser<I, Output = &'i str>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    length_delimited(be_i16())
        .and_then(|result| result.map_err(StreamErrorFor::<I>::other))
        .and_then(|bs| str::from_utf8(bs).map_err(StreamErrorFor::<I>::other))
}

fn nullable_string<'i, I>() -> impl Parser<I, Output = Option<&'i str>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    length_delimited(be_i16())
        .map(|result| result.ok())
        .and_then(|bs| {
            bs.map(|bs| str::from_utf8(bs).map_err(StreamErrorFor::<I>::other))
                .transpose()
        })
}

fn bytes<'i, I>() -> impl Parser<I, Output = &'i [u8]>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    length_delimited(be_i32()).and_then(|result| result.map_err(StreamErrorFor::<I>::other))
}

fn nullable_bytes<'i, I>() -> impl Parser<I, Output = Option<&'i [u8]>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    length_delimited(be_i32()).map(|result| result.ok())
}

fn length_delimited<'i, I, P>(
    p: P,
) -> impl Parser<I, Output = Result<&'i [u8], <usize as TryFrom<P::Output>>::Error>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
    P: Parser<I>,
    usize: TryFrom<P::Output>,
    P::Output: Copy,
{
    p.then_partial(|&mut i| match usize::try_from(i) {
        Ok(i) => range::take(i).map(Ok).left(),
        Err(err) => {
            let mut opt = Some(err);
            combine::parser::function::parser(move |_| {
                Ok((Err(opt.take().unwrap()), combine::error::Commit::Peek(())))
            })
            .right()
        }
    })
}

fn array<'i, I, P>(mut elem_parser: impl FnMut() -> P) -> impl Parser<I, Output = Vec<P::Output>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
    P: Parser<I>,
{
    be_i32().then_partial(move |&mut len| {
        if let Ok(len) = usize::try_from(len) {
            combine::parser::repeat::count_min_max(len, len, elem_parser()).left()
        } else {
            combine::parser::function::parser(|_| {
                Ok((Vec::new(), combine::error::Commit::Peek(())))
            })
            .right()
        }
    })
}

pub trait Buffer: bytes::BufMut + std::ops::DerefMut<Target = [u8]> {}

impl<T> Buffer for T where T: bytes::BufMut + std::ops::DerefMut<Target = [u8]> {}

pub trait Encode {
    fn encode_len(&self) -> usize;
    fn encode(&self, writer: &mut impl Buffer);
}

macro_rules! encode_impl {
    ($($ty: ty, $method: ident, )*) => {$(
        impl Encode for $ty {
            fn encode_len(&self) -> usize {
                mem::size_of::<Self>()
            }

            fn encode(&self, writer: &mut impl Buffer) {
                writer.$method(*self);
            }
        }
    )*};
}

encode_impl! {
    i8, put_i8,
    i16, put_i16,
    i32, put_i32,
    i64, put_i64,
}

impl<T> Encode for Vec<T>
where
    T: Encode,
{
    fn encode_len(&self) -> usize {
        mem::size_of::<i32>() + self.iter().map(|elem| elem.encode_len()).sum::<usize>()
    }

    fn encode(&self, writer: &mut impl Buffer) {
        let l = i32::try_from(self.len()).unwrap();
        l.encode(writer);
        for elem in self {
            elem.encode(writer);
        }
    }
}

impl<R> Encode for Option<RecordBatch<R>>
where
    R: Encode,
{
    fn encode_len(&self) -> usize {
        match self {
            Some(t) => 0i32.encode_len() + t.encode_len(),
            None => (-1i32).encode_len(),
        }
    }

    fn encode(&self, writer: &mut impl Buffer) {
        match self {
            Some(t) => {
                let len_reservation = reserve::<i32, _>(writer);
                t.encode(writer);

                len_reservation.fill_len(writer);
            }
            None => (-1i32).encode(writer),
        }
    }
}

impl Encode for Option<&'_ [u8]> {
    fn encode_len(&self) -> usize {
        match self {
            Some(t) => t.encode_len(),
            None => (-1i32).encode_len(),
        }
    }

    fn encode(&self, writer: &mut impl Buffer) {
        match self {
            Some(t) => t.encode(writer),
            None => (-1i32).encode(writer),
        }
    }
}

impl Encode for Option<&'_ str> {
    fn encode_len(&self) -> usize {
        match self {
            Some(t) => t.encode_len(),
            None => (-1i16).encode_len(),
        }
    }

    fn encode(&self, writer: &mut impl Buffer) {
        match self {
            Some(t) => t.encode(writer),
            None => (-1i16).encode(writer),
        }
    }
}

impl Encode for &'_ [u8] {
    fn encode_len(&self) -> usize {
        mem::size_of::<i32>() + self.len()
    }

    fn encode(&self, writer: &mut impl Buffer) {
        let l = i32::try_from(self.len()).unwrap();
        l.encode(writer);
        writer.put(*self);
    }
}

impl Encode for &'_ str {
    fn encode_len(&self) -> usize {
        mem::size_of::<i16>() + self.len()
    }

    fn encode(&self, writer: &mut impl Buffer) {
        let l = i16::try_from(self.len()).unwrap();
        l.encode(writer);
        writer.put(self.as_bytes());
    }
}

impl Encode for bool {
    fn encode_len(&self) -> usize {
        1
    }

    fn encode(&self, writer: &mut impl Buffer) {
        writer.put_u8(*self as u8);
    }
}

macro_rules! encode_as {
    ($($ty: ty as $prim: ty),* $(,)?) => { $(
        impl Encode for $ty {
            fn encode_len(&self) -> usize {
                mem::size_of::<$prim>()
            }

            fn encode(&self, writer: &mut impl Buffer) {
                (*self as $prim).encode(writer);
            }
        }

    )* }
}

encode_as! {
    ApiKey as i16,
    ErrorCode as i16,
    Acks as i16,
}

#[derive(Clone, PartialEq, Debug)]
pub struct RecordBatch<R> {
    base_offset: i64,
    // batch_length: i32,
    partition_leader_epoch: i32,
    // Computed magic: i8, // (current magic value is 2)
    // Computed crc: i32,
    // bit 0~2:
    //     0: no compression
    //     1: gzip
    //     2: snappy
    //     3: lz4
    //     4: zstd
    // bit 3: timestampType
    // bit 4: isTransactional (0 means not transactional)
    // bit 5: isControlBatch (0 means not a control batch)
    // bit 6~15: unused
    attributes: i16,
    last_offset_delta: i32,
    first_timestamp: i64,
    max_timestamp: i64,
    producer_id: i64,
    producer_epoch: i16,
    base_sequence: i32,
    records: R, // TODO Avoid a vec here when serializing
}

#[derive(Clone, PartialEq, Debug)]
pub struct Record<'i> {
    // Computed length: i32,          // varint
    attributes: i8,       // bit 0~7: unused
    timestamp_delta: i32, // varint
    offset_delta: i32,    // varint
    key: &'i [u8],
    value: &'i [u8],
    headers: Vec<RecordHeader<'i>>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct RecordHeader<'i> {
    key: &'i str,
    value: &'i [u8],
}

pub type OwnedRecordBatch<'i> = RecordBatch<Vec<Record<'i>>>;

impl<'i> OwnedRecordBatch<'i> {
    fn verify<I>(
        range: &[u8],
        record_set: crate::parser::RecordSet<'i>,
    ) -> Result<Self, StreamErrorFor<I>>
    where
        I: RangeStream,
    {
        let crate::parser::RecordSet {
            base_offset,
            batch_length,
            partition_leader_epoch,
            magic,
            crc,
            attributes,
            last_offset_delta,
            first_timestamp,
            max_timestamp,
            producer_id,
            producer_epoch,
            base_sequence,
            records,
        } = record_set;
        if magic != 2 {
            return Err(StreamErrorFor::<I>::message_static_message(
                "Record batch version 2",
            ));
        }
        let crc_range = &range[mem::size_of_val(&base_offset)
            + mem::size_of_val(&batch_length)
            + mem::size_of_val(&partition_leader_epoch)
            + mem::size_of_val(&magic)
            + mem::size_of_val(&crc)..];
        if crc != crc::crc32::checksum_castagnoli(crc_range) as i32 {
            return Err(StreamErrorFor::<I>::message_static_message(
                "Corrupted message",
            ));
        }
        Ok(RecordBatch {
            base_offset,
            partition_leader_epoch,
            attributes,
            last_offset_delta,
            first_timestamp,
            max_timestamp,
            producer_id,
            producer_epoch,
            base_sequence,
            records: records
                .record
                .into_iter()
                .map(|r| {
                    let crate::parser::record_set::Record {
                        length: _,
                        attributes,
                        timestamp_delta,
                        offset_delta,
                        key,
                        value,
                        headers,
                    } = r;
                    Record {
                        attributes,
                        timestamp_delta,
                        offset_delta,
                        key,
                        value,
                        headers: headers
                            .header
                            .into_iter()
                            .map(|header| RecordHeader {
                                key: header.header_key,
                                value: header.value,
                            })
                            .collect(),
                    }
                })
                .collect(),
        })
    }
}

fn record_batch<'i, I>() -> impl Parser<I, Output = Option<OwnedRecordBatch<'i>>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (position(), nullable_bytes())
        .flat_map(|(position, bytes)| match bytes {
            Some(bytes) if !bytes.is_empty() => {
                let attributes = (&bytes[mem::size_of::<i64>()
                    + mem::size_of::<i32>()
                    + mem::size_of::<i32>()
                    + mem::size_of::<i8>()
                    + mem::size_of::<i32>()..])
                    .get_i16();

                if (attributes & (1 << 4)) != 0 {
                    log::trace!("Control batch");
                }

                let (value, rest) = crate::parser::record_set().parse(bytes).map_err(|_| {
                    I::Error::from_error(
                        position,
                        StreamErrorFor::<I>::message_static_message("Unable to parse RecordSet"),
                    )
                })?;
                debug_assert!(rest.is_empty(), "{:#?} {:?}", value, rest);
                log::trace!("Parsed record_set: {:#?}", value);

                Ok(Some((bytes, value)))
            }
            Some(_) | None => Ok(None),
        })
        .and_then(|opt| match opt {
            Some((range, record_set)) => RecordBatch::verify::<I>(range, record_set).map(Some),
            None => Ok(None),
        })
}

fn encode_var_bytes_space(input: &[u8]) -> usize {
    i32::try_from(input.len()).unwrap().required_space() + input.len()
}

fn encode_var_bytes(input: &[u8], writer: &mut impl Buffer) {
    let len = i32::try_from(input.len()).unwrap();
    encode_var_i32(len, writer);
    writer.put(input);
}

fn encode_var_array<T>(input: &[T], writer: &mut impl Buffer)
where
    T: Encode,
{
    let len = i32::try_from(input.len()).unwrap();
    encode_var_i32(len, writer);
    for t in input {
        t.encode(writer);
    }
}

fn encode_var_i32(input: i32, writer: &mut impl Buffer) {
    let mut buf = [0; 5];
    let i = integer_encoding::VarInt::encode_var(input, &mut buf);
    writer.put(&buf[..i]);
}

struct Reservation<T>(usize, std::marker::PhantomData<T>);

fn reserve<T, B>(writer: &mut B) -> Reservation<T>
where
    T: Encode + Default,
    B: Buffer,
{
    let start = writer.len();
    T::default().encode(writer);
    Reservation(start, std::marker::PhantomData)
}

impl<T> Reservation<T>
where
    T: Encode + Sized,
{
    fn end(&self) -> usize {
        self.0 + mem::size_of::<T>()
    }

    fn fill_len(self, buf: &mut [u8]) -> usize {
        let length = buf.len() - self.end();
        self.fill(buf, &i32::try_from(length).unwrap().to_be_bytes());
        length
    }

    fn fill(self, buf: &mut [u8], value: &[u8]) {
        let end = self.0 + mem::size_of::<T>();
        buf[self.0..end].copy_from_slice(value)
    }
}

impl<R> Encode for RecordBatch<R>
where
    R: Encode,
{
    fn encode_len(&self) -> usize {
        self.base_offset.encode_len()
            + mem::size_of::<i32>() // self.batch_length.encode_len()
            + self.partition_leader_epoch.encode_len()
            + mem::size_of::<i8>() // self.magic.encode_len()
            + mem::size_of::<i32>() // self.crc.encode_len()
            + self.attributes.encode_len()
            + self.last_offset_delta.encode_len()
            + self.first_timestamp.encode_len()
            + self.max_timestamp.encode_len()
            + self.producer_id.encode_len()
            + self.producer_epoch.encode_len()
            + self.base_sequence.encode_len()
            + self.records.encode_len()
    }

    fn encode(&self, writer: &mut impl Buffer) {
        self.base_offset.encode(writer);
        let batch_length_reservation = reserve::<i32, _>(writer);
        self.partition_leader_epoch.encode(writer);
        2i8.encode(writer); // self.magic.encode(writer);
        let crc_reservation = reserve::<i32, _>(writer);
        self.attributes.encode(writer);
        self.last_offset_delta.encode(writer);
        self.first_timestamp.encode(writer);
        self.max_timestamp.encode(writer);
        self.producer_id.encode(writer);
        self.producer_epoch.encode(writer);
        self.base_sequence.encode(writer);
        self.records.encode(writer);

        let length = batch_length_reservation.fill_len(writer);

        let crc_slice = &writer[crc_reservation.end()..];
        debug_assert!(crc_slice.len() == length - 4 - 1 - 4);
        let crc = crc::crc32::checksum_castagnoli(crc_slice);
        crc_reservation.fill(writer, &crc.to_be_bytes());
    }
}

impl Encode for Record<'_> {
    fn encode_len(&self) -> usize {
        // self.length.required_space() TODO
        1 // self.attributes
            + self.timestamp_delta.required_space()
            + self.offset_delta.required_space()
            + encode_var_bytes_space(self.key)
            + encode_var_bytes_space(self.value)
            + i32::try_from(self.headers.len())
                .unwrap()
                .required_space()
            + self.headers.iter().map(|h| h.encode_len()).sum::<usize>()
    }

    fn encode(&self, writer: &mut impl Buffer) {
        let length = i32::try_from(self.encode_len()).unwrap();
        encode_var_i32(length, writer);
        writer.put_i8(self.attributes);
        encode_var_i32(self.timestamp_delta, writer);
        encode_var_i32(self.offset_delta, writer);
        encode_var_bytes(self.key, writer);
        encode_var_bytes(self.value, writer);

        encode_var_array(&self.headers, writer);
    }
}

impl Encode for RecordHeader<'_> {
    fn encode_len(&self) -> usize {
        encode_var_bytes_space(self.key.as_bytes()) + encode_var_bytes_space(self.value)
    }

    fn encode(&self, writer: &mut impl Buffer) {
        encode_var_bytes(self.key.as_bytes(), writer);
        encode_var_bytes(self.value, writer);
    }
}

pub const FETCH_EARLIEST_OFFSET: i64 = -2;
pub const FETCH_LATEST_OFFSET: i64 = -1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acks {
    OnlyLeader = -1,
    None = 0,
    Full = 1,
}

impl TryFrom<i16> for Acks {
    type Error = &'static str;
    fn try_from(i: i16) -> Result<Self, Self::Error> {
        Ok(match i {
            -1 => Acks::OnlyLeader,
            0 => Acks::None,
            1 => Acks::Full,
            _ => return Err("Invalid Acks"),
        })
    }
}

pub trait RecordBatchParser<Input>: Sized
where
    Input: combine::Stream,
{
    fn parser() -> combine::parser::combinator::FnOpaque<Input, Option<RecordBatch<Self>>>;
}

impl<'i, I> RecordBatchParser<I> for Vec<Record<'i>>
where
    I: combine::RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: combine::ParseError<I::Token, I::Range, I::Position>,
{
    fn parser() -> combine::parser::combinator::FnOpaque<I, Option<RecordBatch<Self>>> {
        combine::opaque!(combine::parser::combinator::no_partial(record_batch()),)
    }
}
