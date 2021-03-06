use super::*;
pub fn describe_acls_response<'i, I>() -> impl Parser<I, Output = DescribeAclsResponse<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32().expected("throttle_time_ms"),
        be_i16()
            .and_then(|i| {
                ErrorCode::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message)
            })
            .expected("error_code"),
        nullable_string().expected("error_message"),
        array(|| {
            (
                be_i8().expected("resource_type"),
                string().expected("resource_name"),
                be_i8().expected("resource_pattern_type"),
                array(|| {
                    (
                        string().expected("principal"),
                        string().expected("host"),
                        be_i8().expected("operation"),
                        be_i8().expected("permission_type"),
                    )
                        .map(|(principal, host, operation, permission_type)| Acls {
                            principal,
                            host,
                            operation,
                            permission_type,
                        })
                        .expected("acls")
                }),
            )
                .map(
                    |(resource_type, resource_name, resource_pattern_type, acls)| Resources {
                        resource_type,
                        resource_name,
                        resource_pattern_type,
                        acls,
                    },
                )
                .expected("resources")
        }),
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
    pub error_code: ErrorCode,
    pub error_message: Option<&'i str>,
    pub resources: Vec<Resources<'i>>,
}

impl<'i> crate::Encode for DescribeAclsResponse<'i> {
    fn encode_len(&self) -> usize {
        self.throttle_time_ms.encode_len()
            + self.error_code.encode_len()
            + self.error_message.encode_len()
            + self.resources.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.throttle_time_ms.encode(writer);
        self.error_code.encode(writer);
        self.error_message.encode(writer);
        self.resources.encode(writer);
    }
}

pub const VERSION: i16 = 1;

#[derive(Clone, Debug, PartialEq)]
pub struct Acls<'i> {
    pub principal: &'i str,
    pub host: &'i str,
    pub operation: i8,
    pub permission_type: i8,
}

impl<'i> crate::Encode for Acls<'i> {
    fn encode_len(&self) -> usize {
        self.principal.encode_len()
            + self.host.encode_len()
            + self.operation.encode_len()
            + self.permission_type.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.principal.encode(writer);
        self.host.encode(writer);
        self.operation.encode(writer);
        self.permission_type.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Resources<'i> {
    pub resource_type: i8,
    pub resource_name: &'i str,
    pub resource_pattern_type: i8,
    pub acls: Vec<Acls<'i>>,
}

impl<'i> crate::Encode for Resources<'i> {
    fn encode_len(&self) -> usize {
        self.resource_type.encode_len()
            + self.resource_name.encode_len()
            + self.resource_pattern_type.encode_len()
            + self.acls.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.resource_type.encode(writer);
        self.resource_name.encode(writer);
        self.resource_pattern_type.encode(writer);
        self.acls.encode(writer);
    }
}
