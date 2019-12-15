use super::*;
pub fn describe_configs_request<'i, I>() -> impl Parser<I, Output = DescribeConfigsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional((be_i8(), string(), optional(string())).map(
            |(resource_type, resource_name, config_names)| Resources {
                resource_type,
                resource_name,
                config_names,
            },
        )),
        any().map(|b| b != 0),
    )
        .map(|(resources, include_synonyms)| DescribeConfigsRequest {
            resources,
            include_synonyms,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct DescribeConfigsRequest<'i> {
    pub resources: Option<Resources<'i>>,
    pub include_synonyms: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Resources<'i> {
    pub resource_type: i8,
    pub resource_name: &'i str,
    pub config_names: Option<&'i str>,
}
