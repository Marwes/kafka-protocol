use super::*;
pub fn alter_configs_request<'i, I>() -> impl Parser<I, Output = AlterConfigsRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        array(|| {
            (
                be_i8(),
                string(),
                array(|| {
                    (string(), nullable_string()).map(|(config_name, config_value)| ConfigEntries {
                        config_name,
                        config_value,
                    })
                }),
            )
                .map(|(resource_type, resource_name, config_entries)| Resources {
                    resource_type,
                    resource_name,
                    config_entries,
                })
        }),
        any().map(|b| b != 0),
    )
        .map(|(resources, validate_only)| AlterConfigsRequest {
            resources,
            validate_only,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct AlterConfigsRequest<'i> {
    pub resources: Vec<Resources<'i>>,
    pub validate_only: bool,
}

impl<'i> crate::Encode for AlterConfigsRequest<'i> {
    fn encode_len(&self) -> usize {
        self.resources.encode_len() + self.validate_only.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.resources.encode(writer);
        self.validate_only.encode(writer);
    }
}

pub const VERSION: i16 = 1;

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigEntries<'i> {
    pub config_name: &'i str,
    pub config_value: Option<&'i str>,
}

impl<'i> crate::Encode for ConfigEntries<'i> {
    fn encode_len(&self) -> usize {
        self.config_name.encode_len() + self.config_value.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.config_name.encode(writer);
        self.config_value.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Resources<'i> {
    pub resource_type: i8,
    pub resource_name: &'i str,
    pub config_entries: Vec<ConfigEntries<'i>>,
}

impl<'i> crate::Encode for Resources<'i> {
    fn encode_len(&self) -> usize {
        self.resource_type.encode_len()
            + self.resource_name.encode_len()
            + self.config_entries.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.resource_type.encode(writer);
        self.resource_name.encode(writer);
        self.config_entries.encode(writer);
    }
}
