use super::*;
pub fn describe_delegation_token_response<'i, I>(
) -> impl Parser<I, Output = DescribeDelegationTokenResponse<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16()
            .and_then(|i| {
                ErrorCode::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message)
            })
            .expected("error_code"),
        array(|| {
            (
                (
                    string().expected("principal_type"),
                    string().expected("name"),
                )
                    .map(|(principal_type, name)| Owner {
                        principal_type,
                        name,
                    }),
                be_i64().expected("issue_timestamp"),
                be_i64().expected("expiry_timestamp"),
                be_i64().expected("max_timestamp"),
                string().expected("token_id"),
                bytes().expected("hmac"),
                array(|| {
                    (
                        string().expected("principal_type"),
                        string().expected("name"),
                    )
                        .map(|(principal_type, name)| Renewers {
                            principal_type,
                            name,
                        })
                        .expected("renewers")
                }),
            )
                .map(
                    |(
                        owner,
                        issue_timestamp,
                        expiry_timestamp,
                        max_timestamp,
                        token_id,
                        hmac,
                        renewers,
                    )| {
                        TokenDetails {
                            owner,
                            issue_timestamp,
                            expiry_timestamp,
                            max_timestamp,
                            token_id,
                            hmac,
                            renewers,
                        }
                    },
                )
                .expected("token_details")
        }),
        be_i32().expected("throttle_time_ms"),
    )
        .map(
            |(error_code, token_details, throttle_time_ms)| DescribeDelegationTokenResponse {
                error_code,
                token_details,
                throttle_time_ms,
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct DescribeDelegationTokenResponse<'i> {
    pub error_code: ErrorCode,
    pub token_details: Vec<TokenDetails<'i>>,
    pub throttle_time_ms: i32,
}

impl<'i> crate::Encode for DescribeDelegationTokenResponse<'i> {
    fn encode_len(&self) -> usize {
        self.error_code.encode_len()
            + self.token_details.encode_len()
            + self.throttle_time_ms.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.error_code.encode(writer);
        self.token_details.encode(writer);
        self.throttle_time_ms.encode(writer);
    }
}

pub const VERSION: i16 = 1;

#[derive(Clone, Debug, PartialEq)]
pub struct Owner<'i> {
    pub principal_type: &'i str,
    pub name: &'i str,
}

impl<'i> crate::Encode for Owner<'i> {
    fn encode_len(&self) -> usize {
        self.principal_type.encode_len() + self.name.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.principal_type.encode(writer);
        self.name.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Renewers<'i> {
    pub principal_type: &'i str,
    pub name: &'i str,
}

impl<'i> crate::Encode for Renewers<'i> {
    fn encode_len(&self) -> usize {
        self.principal_type.encode_len() + self.name.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.principal_type.encode(writer);
        self.name.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TokenDetails<'i> {
    pub owner: Owner<'i>,
    pub issue_timestamp: i64,
    pub expiry_timestamp: i64,
    pub max_timestamp: i64,
    pub token_id: &'i str,
    pub hmac: &'i [u8],
    pub renewers: Vec<Renewers<'i>>,
}

impl<'i> crate::Encode for TokenDetails<'i> {
    fn encode_len(&self) -> usize {
        self.owner.encode_len()
            + self.issue_timestamp.encode_len()
            + self.expiry_timestamp.encode_len()
            + self.max_timestamp.encode_len()
            + self.token_id.encode_len()
            + self.hmac.encode_len()
            + self.renewers.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.owner.encode(writer);
        self.issue_timestamp.encode(writer);
        self.expiry_timestamp.encode(writer);
        self.max_timestamp.encode(writer);
        self.token_id.encode(writer);
        self.hmac.encode(writer);
        self.renewers.encode(writer);
    }
}
