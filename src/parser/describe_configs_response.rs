use super::*;
pub fn describe_configs_response<'i, I>() -> impl Parser<I, Output = DescribeConfigsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        many(
            (
                be_i16(),
                nullable_string(),
                be_i8(),
                string(),
                many(
                    (
                        string(),
                        nullable_string(),
                        any().map(|b| b != 0),
                        be_i8(),
                        any().map(|b| b != 0),
                        many((string(), nullable_string(), be_i8()).map(
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
    pub resources: Vec<Resources<'i>>,
}

impl<'i> crate::Encode for DescribeConfigsResponse<'i> {
    fn encode_len(&self) -> usize {
        self.throttle_time_ms.encode_len() + self.resources.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.throttle_time_ms.encode(writer);
        self.resources.encode(writer);
    }
}

pub const VERSION: i16 = 2;

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
    fn encode(&self, writer: &mut impl bytes::BufMut) {
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
    pub config_source: i8,
    pub is_sensitive: bool,
    pub config_synonyms: Vec<ConfigSynonyms<'i>>,
}

impl<'i> crate::Encode for ConfigEntries<'i> {
    fn encode_len(&self) -> usize {
        self.config_name.encode_len()
            + self.config_value.encode_len()
            + self.read_only.encode_len()
            + self.config_source.encode_len()
            + self.is_sensitive.encode_len()
            + self.config_synonyms.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.config_name.encode(writer);
        self.config_value.encode(writer);
        self.read_only.encode(writer);
        self.config_source.encode(writer);
        self.is_sensitive.encode(writer);
        self.config_synonyms.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Resources<'i> {
    pub error_code: i16,
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
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.error_code.encode(writer);
        self.error_message.encode(writer);
        self.resource_type.encode(writer);
        self.resource_name.encode(writer);
        self.config_entries.encode(writer);
    }
}
