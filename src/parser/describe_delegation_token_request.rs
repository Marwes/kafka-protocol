use super::*;
pub fn describe_delegation_token_request<'i, I>(
) -> impl Parser<I, Output = DescribeDelegationTokenRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (many((string(), string()).map(|(principal_type, name)| {
        Owners {
            principal_type,
            name,
        }
    })),)
        .map(|(owners,)| DescribeDelegationTokenRequest { owners })
}

#[derive(Clone, Debug, PartialEq)]
pub struct DescribeDelegationTokenRequest<'i> {
    pub owners: Vec<Owners<'i>>,
}

impl<'i> crate::Encode for DescribeDelegationTokenRequest<'i> {
    fn encode_len(&self) -> usize {
        self.owners.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.owners.encode(writer);
    }
}

pub const VERSION: i16 = 1;

#[derive(Clone, Debug, PartialEq)]
pub struct Owners<'i> {
    pub principal_type: &'i str,
    pub name: &'i str,
}

impl<'i> crate::Encode for Owners<'i> {
    fn encode_len(&self) -> usize {
        self.principal_type.encode_len() + self.name.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.principal_type.encode(writer);
        self.name.encode(writer);
    }
}
