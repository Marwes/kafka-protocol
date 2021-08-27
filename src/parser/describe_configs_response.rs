use super::*;
pub fn describe_configs_response<'i, I>(
) -> impl Parser<I, Output = DescribeConfigsResponse<'i>> + 'i
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
                be_i8().expected("resource_type"),
                string().expected("resource_name"),
                array(|| {
                    (
                        string().expected("config_name"),
                        nullable_string().expected("config_value"),
                        any().map(|b| b != 0).expected("read_only"),
                        any().map(|b| b != 0).expected("is_sensitive"),
                        array(|| {
                            (
                                string().expected("config_name"),
                                nullable_string().expected("config_value"),
                                be_i8().expected("config_source"),
                            )
                                .map(
                                    |(config_name, config_value, config_source)| ConfigSynonyms {
                                        config_name,
                                        config_value,
                                        config_source,
                                    },
                                )
                                .expected("config_synonyms")
                        }),
                        any().map(|b| b != 0).expected("is_default"),
                        any().map(|b| b != 0).expected("is_sensitive"),
                    )
                        .map(
                            |(
                                config_name,
                                config_value,
                                read_only,
                                is_sensitive,
                                config_synonyms,
                                is_default,
                                is_sensitive,
                            )| {
                                ConfigEntries {
                                    config_name,
                                    config_value,
                                    read_only,
                                    is_sensitive,
                                    config_synonyms,
                                    is_default,
                                    is_sensitive,
                                }
                            },
                        )
                        .expected("config_entries")
                }),
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
                )
                .expected("resources")
        }),
    )
        .map(|(throttle_time_ms, resources)| DescribeConfigsResponse {
            throttle_time_ms,
            resources,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct DescribeConfigsResponse<'i> {
    pub throttle_time_ms: i32,
    pub resources: Vec<Resources<'i>>,
}

impl<'i> crate::Encode for DescribeConfigsResponse<'i> {
    fn encode_len(&self) -> usize {
        self.throttle_time_ms.encode_len() + self.resources.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.throttle_time_ms.encode(writer);
        self.resources.encode(writer);
    }
}

pub const VERSION: i16 = 0;

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigSynonyms<'i> {
    pub config_name: &'i str,
    pub config_value: Option<&'i str>,
    pub config_source: i8,
}

impl<'i> crate::Encode for ConfigSynonyms<'i> {
    fn encode_len(&self) -> usize {
        self.config_name.encode_len()
            + self.config_value.encode_len()
            + self.config_source.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.config_name.encode(writer);
        self.config_value.encode(writer);
        self.config_source.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigEntries<'i> {
    pub config_name: &'i str,
    pub config_value: Option<&'i str>,
    pub read_only: bool,
    pub is_sensitive: bool,
    pub config_synonyms: Vec<ConfigSynonyms<'i>>,
    pub is_default: bool,
    pub is_sensitive: bool,
}

impl<'i> crate::Encode for ConfigEntries<'i> {
    fn encode_len(&self) -> usize {
        self.config_name.encode_len()
            + self.config_value.encode_len()
            + self.read_only.encode_len()
            + self.is_sensitive.encode_len()
            + self.config_synonyms.encode_len()
            + self.is_default.encode_len()
            + self.is_sensitive.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.config_name.encode(writer);
        self.config_value.encode(writer);
        self.read_only.encode(writer);
        self.is_sensitive.encode(writer);
        self.config_synonyms.encode(writer);
        self.is_default.encode(writer);
        self.is_sensitive.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Resources<'i> {
    pub error_code: ErrorCode,
    pub error_message: Option<&'i str>,
    pub resource_type: i8,
    pub resource_name: &'i str,
    pub config_entries: Vec<ConfigEntries<'i>>,
}

impl<'i> crate::Encode for Resources<'i> {
    fn encode_len(&self) -> usize {
        self.error_code.encode_len()
            + self.error_message.encode_len()
            + self.resource_type.encode_len()
            + self.resource_name.encode_len()
            + self.config_entries.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.error_code.encode(writer);
        self.error_message.encode(writer);
        self.resource_type.encode(writer);
        self.resource_name.encode(writer);
        self.config_entries.encode(writer);
    }
}
