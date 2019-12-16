use super::*;
pub fn create_delegation_token_request<'i, I>(
) -> impl Parser<I, Output = CreateDelegationTokenRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        array(|| {
            (string(), string()).map(|(principal_type, name)| Renewers {
                principal_type,
                name,
            })
        }),
        be_i64(),
    )
        .map(|(renewers, max_life_time)| CreateDelegationTokenRequest {
            renewers,
            max_life_time,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateDelegationTokenRequest<'i> {
    pub renewers: Vec<Renewers<'i>>,
    pub max_life_time: i64,
}

impl<'i> crate::Encode for CreateDelegationTokenRequest<'i> {
    fn encode_len(&self) -> usize {
        self.renewers.encode_len() + self.max_life_time.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.renewers.encode(writer);
        self.max_life_time.encode(writer);
    }
}

pub const VERSION: i16 = 1;

#[derive(Clone, Debug, PartialEq)]
pub struct Renewers<'i> {
    pub principal_type: &'i str,
    pub name: &'i str,
}

impl<'i> crate::Encode for Renewers<'i> {
    fn encode_len(&self) -> usize {
        self.principal_type.encode_len() + self.name.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.principal_type.encode(writer);
        self.name.encode(writer);
    }
}
