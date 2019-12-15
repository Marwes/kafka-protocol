use super::*;
pub fn api_versions_response<'i, I>() -> impl Parser<I, Output = ApiVersionsResponse>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16(),
        optional(
            (be_i16(), be_i16(), be_i16()).map(|(api_key, min_version, max_version)| ApiVersions {
                api_key,
                min_version,
                max_version,
            }),
        ),
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
    pub api_versions: Option<ApiVersions>,
    pub throttle_time_ms: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ApiVersions {
    pub api_key: i16,
    pub min_version: i16,
    pub max_version: i16,
}
