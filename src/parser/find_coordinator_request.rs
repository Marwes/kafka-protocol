use super::*;
pub fn find_coordinator_request<'i, I>() -> impl Parser<I, Output = FindCoordinatorRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (string().expected("key"), be_i8().expected("key_type"))
        .map(|(key, key_type)| FindCoordinatorRequest { key, key_type })
}

#[derive(Clone, Debug, PartialEq)]
pub struct FindCoordinatorRequest<'i> {
    pub key: &'i str,
    pub key_type: i8,
}

impl<'i> crate::Encode for FindCoordinatorRequest<'i> {
    fn encode_len(&self) -> usize {
        self.key.encode_len() + self.key_type.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.key.encode(writer);
        self.key_type.encode(writer);
    }
}

pub const VERSION: i16 = 2;
