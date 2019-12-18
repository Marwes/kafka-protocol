use super::*;
pub fn create_delegation_token_response<'i, I>(
) -> impl Parser<I, Output = CreateDelegationTokenResponse<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16().and_then(|i| {
            ErrorCode::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message)
        }),
        (string(), string()).map(|(principal_type, name)| Owner {
            principal_type,
            name,
        }),
        be_i64(),
        be_i64(),
        be_i64(),
        string(),
        bytes(),
        be_i32(),
    )
        .map(
            |(
                error_code,
                owner,
                issue_timestamp,
                expiry_timestamp,
                max_timestamp,
                token_id,
                hmac,
                throttle_time_ms,
            )| {
                CreateDelegationTokenResponse {
                    error_code,
                    owner,
                    issue_timestamp,
                    expiry_timestamp,
                    max_timestamp,
                    token_id,
                    hmac,
                    throttle_time_ms,
                }
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateDelegationTokenResponse<'i> {
    pub error_code: ErrorCode,
    pub owner: Owner<'i>,
    pub issue_timestamp: i64,
    pub expiry_timestamp: i64,
    pub max_timestamp: i64,
    pub token_id: &'i str,
    pub hmac: &'i [u8],
    pub throttle_time_ms: i32,
}

impl<'i> crate::Encode for CreateDelegationTokenResponse<'i> {
    fn encode_len(&self) -> usize {
        self.error_code.encode_len()
            + self.owner.encode_len()
            + self.issue_timestamp.encode_len()
            + self.expiry_timestamp.encode_len()
            + self.max_timestamp.encode_len()
            + self.token_id.encode_len()
            + self.hmac.encode_len()
            + self.throttle_time_ms.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.error_code.encode(writer);
        self.owner.encode(writer);
        self.issue_timestamp.encode(writer);
        self.expiry_timestamp.encode(writer);
        self.max_timestamp.encode(writer);
        self.token_id.encode(writer);
        self.hmac.encode(writer);
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
