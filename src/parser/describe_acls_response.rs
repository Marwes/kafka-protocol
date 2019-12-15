use super::*;
pub fn describe_acls_response<'i, I>() -> impl Parser<I, Output = DescribeAclsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i16(),
        nullable_string(),
        optional(
            (
                be_i8(),
                string(),
                be_i8(),
                optional((string(), string(), be_i8(), be_i8()).map(
                    |(principal, host, operation, permission_type)| Acls {
                        principal,
                        host,
                        operation,
                        permission_type,
                    },
                )),
            )
                .map(
                    |(resource_type, resource_name, resource_pattern_type, acls)| Resources {
                        resource_type,
                        resource_name,
                        resource_pattern_type,
                        acls,
                    },
                ),
        ),
    )
        .map(
            |(throttle_time_ms, error_code, error_message, resources)| DescribeAclsResponse {
                throttle_time_ms,
                error_code,
                error_message,
                resources,
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct DescribeAclsResponse<'i> {
    pub throttle_time_ms: i32,
    pub error_code: i16,
    pub error_message: Option<&'i str>,
    pub resources: Option<Resources<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Acls<'i> {
    pub principal: &'i str,
    pub host: &'i str,
    pub operation: i8,
    pub permission_type: i8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Resources<'i> {
    pub resource_type: i8,
    pub resource_name: &'i str,
    pub resource_pattern_type: i8,
    pub acls: Option<Acls<'i>>,
}
