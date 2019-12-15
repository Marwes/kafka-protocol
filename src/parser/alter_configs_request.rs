use super::*;
pub fn alter_configs_request<'i, I>() -> impl Parser<I, Output = AlterConfigsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional(
            (
                be_i8(),
                string(),
                optional(
                    (string(), nullable_string()).map(|(config_name, config_value)| {
                        ConfigEntries {
                            config_name,
                            config_value,
                        }
                    }),
                ),
            )
                .map(|(resource_type, resource_name, config_entries)| Resources {
                    resource_type,
                    resource_name,
                    config_entries,
                }),
        ),
        any().map(|b| b != 0),
    )
        .map(|(resources, validate_only)| AlterConfigsRequest {
            resources,
            validate_only,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct AlterConfigsRequest<'i> {
    pub resources: Option<Resources<'i>>,
    pub validate_only: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigEntries<'i> {
    pub config_name: &'i str,
    pub config_value: Option<&'i str>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Resources<'i> {
    pub resource_type: i8,
    pub resource_name: &'i str,
    pub config_entries: Option<ConfigEntries<'i>>,
}
