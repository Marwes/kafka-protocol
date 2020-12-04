use super::*;
pub fn record<'i, I>() -> impl Parser<I, Output = Record<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        varint().expected("length"),
        be_i8().expected("attributes"),
        varint().expected("timestampDelta"),
        varint().expected("offsetDelta"),
        varbytes().expected("key"),
        varbytes().expected("value"),
        vararray(|| {
            (
                varstring().expected("headerKey"),
                varbytes().expected("Value"),
            )
                .map(|(header_key, value)| Header { header_key, value })
                .expected("Headers")
        }),
    )
        .map(
            |(length, attributes, timestamp_delta, offset_delta, key, value, headers)| Record {
                length,
                attributes,
                timestamp_delta,
                offset_delta,
                key,
                value,
                headers,
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct Record<'i> {
    pub length: i32,
    pub attributes: i8,
    pub timestamp_delta: i32,
    pub offset_delta: i32,
    pub key: &'i [u8],
    pub value: &'i [u8],
    pub headers: Vec<Header<'i>>,
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
