use super::*;
pub fn incremental_alter_configs_request<'i, I>(
) -> impl Parser<I, Output = IncrementalAlterConfigsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional(
            (
                be_i8(),
                string(),
                optional((string(), be_i8(), nullable_string()).map(
                    |(name, config_operation, value)| Configs {
                        name,
                        config_operation,
                        value,
                    },
                )),
            )
                .map(|(resource_type, resource_name, configs)| Resources {
                    resource_type,
                    resource_name,
                    configs,
                }),
        ),
        any().map(|b| b != 0),
    )
        .map(
            |(resources, validate_only)| IncrementalAlterConfigsRequest {
                resources,
                validate_only,
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct IncrementalAlterConfigsRequest<'i> {
    pub resources: Option<Resources<'i>>,
    pub validate_only: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Configs<'i> {
    pub name: &'i str,
    pub config_operation: i8,
    pub value: Option<&'i str>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Resources<'i> {
    pub resource_type: i8,
    pub resource_name: &'i str,
    pub configs: Option<Configs<'i>>,
}
