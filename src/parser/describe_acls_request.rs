use super::*;
pub fn describe_acls_request<'i, I>() -> impl Parser<I, Output = DescribeAclsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i8(),
        nullable_string(),
        be_i8(),
        nullable_string(),
        nullable_string(),
        be_i8(),
        be_i8(),
    )
        .map(
            |(
                resource_type,
                resource_name,
                resource_pattern_type_filter,
                principal,
                host,
                operation,
                permission_type,
            )| {
                DescribeAclsRequest {
                    resource_type,
                    resource_name,
                    resource_pattern_type_filter,
                    principal,
                    host,
                    operation,
                    permission_type,
                }
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct DescribeAclsRequest<'i> {
    pub resource_type: i8,
    pub resource_name: Option<&'i str>,
    pub resource_pattern_type_filter: i8,
    pub principal: Option<&'i str>,
    pub host: Option<&'i str>,
    pub operation: i8,
    pub permission_type: i8,
}

impl<'i> crate::Encode for DescribeAclsRequest<'i> {
    fn encode_len(&self) -> usize {
        self.resource_type.encode_len()
            + self.resource_name.encode_len()
            + self.resource_pattern_type_filter.encode_len()
            + self.principal.encode_len()
            + self.host.encode_len()
            + self.operation.encode_len()
            + self.permission_type.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.resource_type.encode(writer);
        self.resource_name.encode(writer);
        self.resource_pattern_type_filter.encode(writer);
        self.principal.encode(writer);
        self.host.encode(writer);
        self.operation.encode(writer);
        self.permission_type.encode(writer);
    }
}

pub const VERSION: i16 = 1;
