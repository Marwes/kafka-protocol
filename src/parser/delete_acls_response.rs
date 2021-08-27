use super::*;
pub fn delete_acls_response<'i, I>() -> impl Parser<I, Output = DeleteAclsResponse<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32().expected("throttle_time_ms"),
        array(|| {
            (
                be_i16()
                    .and_then(|i| {
                        ErrorCode::try_from(i)
                            .map_err(StreamErrorFor::<I>::unexpected_static_message)
                    })
                    .expected("error_code"),
                nullable_string().expected("error_message"),
                array(|| {
                    (
                        be_i16()
                            .and_then(|i| {
                                ErrorCode::try_from(i)
                                    .map_err(StreamErrorFor::<I>::unexpected_static_message)
                            })
                            .expected("error_code"),
                        nullable_string().expected("error_message"),
                        be_i8().expected("resource_type"),
                        string().expected("resource_name"),
                        string().expected("principal"),
                        string().expected("host"),
                        be_i8().expected("operation"),
                        be_i8().expected("permission_type"),
                        be_i8().expected("permission_type"),
                    )
                        .map(
                            |(
                                error_code,
                                error_message,
                                resource_type,
                                resource_name,
                                principal,
                                host,
                                operation,
                                permission_type,
                                permission_type,
                            )| {
                                MatchingAcls {
                                    error_code,
                                    error_message,
                                    resource_type,
                                    resource_name,
                                    principal,
                                    host,
                                    operation,
                                    permission_type,
                                    permission_type,
                                }
                            },
                        )
                        .expected("matching_acls")
                }),
            )
                .map(
                    |(error_code, error_message, matching_acls)| FilterResponses {
                        error_code,
                        error_message,
                        matching_acls,
                    },
                )
                .expected("filter_responses")
        }),
    )
        .map(|(throttle_time_ms, filter_responses)| DeleteAclsResponse {
            throttle_time_ms,
            filter_responses,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeleteAclsResponse<'i> {
    pub throttle_time_ms: i32,
    pub filter_responses: Vec<FilterResponses<'i>>,
}

impl<'i> crate::Encode for DeleteAclsResponse<'i> {
    fn encode_len(&self) -> usize {
        self.throttle_time_ms.encode_len() + self.filter_responses.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.throttle_time_ms.encode(writer);
        self.filter_responses.encode(writer);
    }
}

pub const VERSION: i16 = 0;

#[derive(Clone, Debug, PartialEq)]
pub struct MatchingAcls<'i> {
    pub error_code: ErrorCode,
    pub error_message: Option<&'i str>,
    pub resource_type: i8,
    pub resource_name: &'i str,
    pub principal: &'i str,
    pub host: &'i str,
    pub operation: i8,
    pub permission_type: i8,
    pub permission_type: i8,
}

impl<'i> crate::Encode for MatchingAcls<'i> {
    fn encode_len(&self) -> usize {
        self.error_code.encode_len()
            + self.error_message.encode_len()
            + self.resource_type.encode_len()
            + self.resource_name.encode_len()
            + self.principal.encode_len()
            + self.host.encode_len()
            + self.operation.encode_len()
            + self.permission_type.encode_len()
            + self.permission_type.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.error_code.encode(writer);
        self.error_message.encode(writer);
        self.resource_type.encode(writer);
        self.resource_name.encode(writer);
        self.principal.encode(writer);
        self.host.encode(writer);
        self.operation.encode(writer);
        self.permission_type.encode(writer);
        self.permission_type.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FilterResponses<'i> {
    pub error_code: ErrorCode,
    pub error_message: Option<&'i str>,
    pub matching_acls: Vec<MatchingAcls<'i>>,
}

impl<'i> crate::Encode for FilterResponses<'i> {
    fn encode_len(&self) -> usize {
        self.error_code.encode_len()
            + self.error_message.encode_len()
            + self.matching_acls.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.error_code.encode(writer);
        self.error_message.encode(writer);
        self.matching_acls.encode(writer);
    }
}
