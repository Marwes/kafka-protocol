use super::*;
pub fn create_acls_request<'i, I>() -> impl Parser<I, Output = CreateAclsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (optional(
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
    pub creations: Option<Creations<'i>>,
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
