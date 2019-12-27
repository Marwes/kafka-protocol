use super::*;
pub fn record_set<'i, I>() -> impl Parser<I, Output = RecordSet<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i64().expected("baseOffset"),
        be_i32().expected("batchLength"),
        be_i32().expected("partitionLeaderEpoch"),
        be_i8().expected("magic"),
        be_i32().expected("crc"),
        be_i16().expected("attributes"),
        be_i32().expected("lastOffsetDelta"),
        be_i64().expected("firstTimestamp"),
        be_i64().expected("maxTimestamp"),
        be_i64().expected("producerId"),
        be_i16().expected("producerEpoch"),
        be_i32().expected("baseSequence"),
        (array(|| {
            (
                varint().expected("length"),
                be_i8().expected("attributes"),
                varint().expected("timestampDelta"),
                varint().expected("offsetDelta"),
                varbytes().expected("key"),
                varbytes().expected("value"),
                (vararray(|| {
                    (
                        varstring().expected("headerKey"),
                        varbytes().expected("Value"),
                    )
                        .map(|(header_key, value)| Header { header_key, value })
                        .expected("Header")
                }),)
                    .map(|(header,)| Headers { header })
                    .expected("Headers"),
            )
                .map(
                    |(length, attributes, timestamp_delta, offset_delta, key, value, headers)| {
                        Record {
                            length,
                            attributes,
                            timestamp_delta,
                            offset_delta,
                            key,
                            value,
                            headers,
                        }
                    },
                )
                .expected("Record")
        }),)
            .map(|(record,)| Records { record })
            .expected("records"),
    )
        .map(
            |(
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
            )| {
                RecordSet {
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
                }
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct RecordSet<'i> {
    pub base_offset: i64,
    pub batch_length: i32,
    pub partition_leader_epoch: i32,
    pub magic: i8,
    pub crc: i32,
    pub attributes: i16,
    pub last_offset_delta: i32,
    pub first_timestamp: i64,
    pub max_timestamp: i64,
    pub producer_id: i64,
    pub producer_epoch: i16,
    pub base_sequence: i32,
    pub records: Records<'i>,
}

impl<'i> crate::Encode for RecordSet<'i> {
    fn encode_len(&self) -> usize {
        self.base_offset.encode_len()
            + self.batch_length.encode_len()
            + self.partition_leader_epoch.encode_len()
            + self.magic.encode_len()
            + self.crc.encode_len()
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
        self.batch_length.encode(writer);
        self.partition_leader_epoch.encode(writer);
        self.magic.encode(writer);
        self.crc.encode(writer);
        self.attributes.encode(writer);
        self.last_offset_delta.encode(writer);
        self.first_timestamp.encode(writer);
        self.max_timestamp.encode(writer);
        self.producer_id.encode(writer);
        self.producer_epoch.encode(writer);
        self.base_sequence.encode(writer);
        self.records.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Header<'i> {
    pub header_key: &'i str,
    pub value: &'i [u8],
}

impl<'i> crate::Encode for Header<'i> {
    fn encode_len(&self) -> usize {
        self.header_key.encode_len() + self.value.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.header_key.encode(writer);
        self.value.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Headers<'i> {
    pub header: Vec<Header<'i>>,
}

impl<'i> crate::Encode for Headers<'i> {
    fn encode_len(&self) -> usize {
        self.header.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.header.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Record<'i> {
    pub length: i32,
    pub attributes: i8,
    pub timestamp_delta: i32,
    pub offset_delta: i32,
    pub key: &'i [u8],
    pub value: &'i [u8],
    pub headers: Headers<'i>,
}

impl<'i> crate::Encode for Record<'i> {
    fn encode_len(&self) -> usize {
        self.length.encode_len()
            + self.attributes.encode_len()
            + self.timestamp_delta.encode_len()
            + self.offset_delta.encode_len()
            + self.key.encode_len()
            + self.value.encode_len()
            + self.headers.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.length.encode(writer);
        self.attributes.encode(writer);
        self.timestamp_delta.encode(writer);
        self.offset_delta.encode(writer);
        self.key.encode(writer);
        self.value.encode(writer);
        self.headers.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Records<'i> {
    pub record: Vec<Record<'i>>,
}

impl<'i> crate::Encode for Records<'i> {
    fn encode_len(&self) -> usize {
        self.record.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.record.encode(writer);
    }
}
