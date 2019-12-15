use super::*;
pub fn delete_acls_request<'i, I>() -> impl Parser<I, Output = DeleteAclsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (optional(
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
                    Filters {
                        resource_type,
                        resource_name,
                        resource_pattern_type_filter,
                        principal,
                        host,
                        operation,
                        permission_type,
                    }
                },
            ),
    ),)
        .map(|(filters,)| DeleteAclsRequest { filters })
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeleteAclsRequest<'i> {
    pub filters: Option<Filters<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Filters<'i> {
    pub resource_type: i8,
    pub resource_name: Option<&'i str>,
    pub resource_pattern_type_filter: i8,
    pub principal: Option<&'i str>,
    pub host: Option<&'i str>,
    pub operation: i8,
    pub permission_type: i8,
}
