use super::*;
pub fn create_acls_request<'i, I>() -> impl Parser<I, Output = CreateAclsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (many(
        (
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
                    resource_type,
                    resource_name,
                    resource_pattern_type,
                    principal,
                    host,
                    operation,
                    permission_type,
                )| {
                    Creations {
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
    ),)
        .map(|(creations,)| CreateAclsRequest { creations })
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateAclsRequest<'i> {
    pub creations: Vec<Creations<'i>>,
}

impl<'i> crate::Encode for CreateAclsRequest<'i> {
    fn encode_len(&self) -> usize {
        self.creations.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.creations.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Creations<'i> {
    pub resource_type: i8,
    pub resource_name: &'i str,
    pub resource_pattern_type: i8,
    pub principal: &'i str,
    pub host: &'i str,
    pub operation: i8,
    pub permission_type: i8,
}

impl<'i> crate::Encode for Creations<'i> {
    fn encode_len(&self) -> usize {
        self.resource_type.encode_len()
            + self.resource_name.encode_len()
            + self.resource_pattern_type.encode_len()
            + self.principal.encode_len()
            + self.host.encode_len()
            + self.operation.encode_len()
            + self.permission_type.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.resource_type.encode(writer);
        self.resource_name.encode(writer);
        self.resource_pattern_type.encode(writer);
        self.principal.encode(writer);
        self.host.encode(writer);
        self.operation.encode(writer);
        self.permission_type.encode(writer);
    }
}
