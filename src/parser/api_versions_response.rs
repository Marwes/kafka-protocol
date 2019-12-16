use super::*;
pub fn api_versions_response<'i, I>() -> impl Parser<I, Output = ApiVersionsResponse>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16(),
        array(|| {
            (be_i16(), be_i16(), be_i16()).map(|(api_key, min_version, max_version)| ApiVersions {
                api_key,
                min_version,
                max_version,
            })
        }),
        be_i32(),
    )
        .map(
            |(error_code, api_versions, throttle_time_ms)| ApiVersionsResponse {
                error_code,
                api_versions,
                throttle_time_ms,
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct ApiVersionsResponse {
    pub error_code: i16,
    pub api_versions: Vec<ApiVersions>,
    pub throttle_time_ms: i32,
}

impl crate::Encode for ApiVersionsResponse {
    fn encode_len(&self) -> usize {
        self.error_code.encode_len()
            + self.api_versions.encode_len()
            + self.throttle_time_ms.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.error_code.encode(writer);
        self.api_versions.encode(writer);
        self.throttle_time_ms.encode(writer);
    }
}

pub const VERSION: i16 = 2;

#[derive(Clone, Debug, PartialEq)]
pub struct ApiVersions {
    pub api_key: i16,
    pub min_version: i16,
    pub max_version: i16,
}

impl crate::Encode for ApiVersions {
    fn encode_len(&self) -> usize {
        self.api_key.encode_len() + self.min_version.encode_len() + self.max_version.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.api_key.encode(writer);
        self.min_version.encode(writer);
        self.max_version.encode(writer);
    }
}
