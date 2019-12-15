use super::*;
pub fn describe_configs_response<'i, I>() -> impl Parser<I, Output = DescribeConfigsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
            (
                be_i16(),
                nullable_string(),
                be_i8(),
                string(),
                optional(
                    (
                        string(),
                        nullable_string(),
                        any().map(|b| b != 0),
                        be_i8(),
                        any().map(|b| b != 0),
                        optional((string(), nullable_string(), be_i8()).map(
                            |(config_name, config_value, config_source)| ConfigSynonyms {
                                config_name,
                                config_value,
                                config_source,
                            },
                        )),
                    )
                        .map(
                            |(
                                config_name,
                                config_value,
                                read_only,
                                config_source,
                                is_sensitive,
                                config_synonyms,
                            )| {
                                ConfigEntries {
                                    config_name,
                                    config_value,
                                    read_only,
                                    config_source,
                                    is_sensitive,
                                    config_synonyms,
                                }
                            },
                        ),
                ),
            )
                .map(
                    |(error_code, error_message, resource_type, resource_name, config_entries)| {
                        Resources {
                            error_code,
                            error_message,
                            resource_type,
                            resource_name,
                            config_entries,
                        }
                    },
                ),
        ),
    )
        .map(|(throttle_time_ms, resources)| DescribeConfigsResponse {
            throttle_time_ms,
            resources,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct DescribeConfigsResponse<'i> {
    pub throttle_time_ms: i32,
    pub resources: Option<Resources<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigSynonyms<'i> {
    pub config_name: &'i str,
    pub config_value: Option<&'i str>,
    pub config_source: i8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigEntries<'i> {
    pub config_name: &'i str,
    pub config_value: Option<&'i str>,
    pub read_only: bool,
    pub config_source: i8,
    pub is_sensitive: bool,
    pub config_synonyms: Option<ConfigSynonyms<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Resources<'i> {
    pub error_code: i16,
    pub error_message: Option<&'i str>,
    pub resource_type: i8,
    pub resource_name: &'i str,
    pub config_entries: Option<ConfigEntries<'i>>,
}
