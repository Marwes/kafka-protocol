use super::*;
pub fn alter_configs_response<'i, I>() -> impl Parser<I, Output = AlterConfigsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional((be_i16(), nullable_string(), be_i8(), string()).map(
            |(error_code, error_message, resource_type, resource_name)| Resources {
                error_code,
                error_message,
                resource_type,
                resource_name,
            },
        )),
    )
        .map(|(throttle_time_ms, resources)| AlterConfigsResponse {
            throttle_time_ms,
            resources,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct AlterConfigsResponse<'i> {
    pub throttle_time_ms: i32,
    pub resources: Option<Resources<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Resources<'i> {
    pub error_code: i16,
    pub error_message: Option<&'i str>,
    pub resource_type: i8,
    pub resource_name: &'i str,
}
