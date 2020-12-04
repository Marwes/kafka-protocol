use super::*;
pub fn record_set<'i, R: RecordBatchParser<I> + 'i, I>(
) -> impl Parser<I, Output = RecordSet<R>> + 'i
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
        R::parser().expected("records"),
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
pub struct RecordSet<R> {
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
    pub records: R,
}

impl<R> crate::Encode for RecordSet<R>
where
    R: Encode,
{
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
