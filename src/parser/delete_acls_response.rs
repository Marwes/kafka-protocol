use super::*;
pub fn delete_acls_response<'i, I>() -> impl Parser<I, Output = DeleteAclsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
            (
                be_i16(),
                nullable_string(),
                optional(
                    (
                        be_i16(),
                        nullable_string(),
                        be_i8(),
                        string(),
                        be_i8(),
                        string(),
                        string(),
                        be_i8(),
                        be_i8(),
                    )
                        .map(
                            |(
                                error_code,
                                error_message,
                                resource_type,
                                resource_name,
                                resource_pattern_type,
                                principal,
                                host,
                                operation,
                                permission_type,
                            )| {
                                MatchingAcls {
                                    error_code,
                                    error_message,
                                    resource_type,
                                    resource_name,
                                    resource_pattern_type,
                                    principal,
                                    host,
                                    operation,
                                    permission_type,
                                }
                            },
                        ),
                ),
            )
                .map(
                    |(error_code, error_message, matching_acls)| FilterResponses {
                        error_code,
                        error_message,
                        matching_acls,
                    },
                ),
        ),
    )
        .map(|(throttle_time_ms, filter_responses)| DeleteAclsResponse {
            throttle_time_ms,
            filter_responses,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeleteAclsResponse<'i> {
    pub throttle_time_ms: i32,
    pub filter_responses: Option<FilterResponses<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MatchingAcls<'i> {
    pub error_code: i16,
    pub error_message: Option<&'i str>,
    pub resource_type: i8,
    pub resource_name: &'i str,
    pub resource_pattern_type: i8,
    pub principal: &'i str,
    pub host: &'i str,
    pub operation: i8,
    pub permission_type: i8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FilterResponses<'i> {
    pub error_code: i16,
    pub error_message: Option<&'i str>,
    pub matching_acls: Option<MatchingAcls<'i>>,
}
