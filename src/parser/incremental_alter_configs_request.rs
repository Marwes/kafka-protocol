use super::*;
pub fn incremental_alter_configs_request<'i, I>(
) -> impl Parser<I, Output = IncrementalAlterConfigsRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        array(|| {
            (
                be_i8().expected("resource_type"),
                string().expected("resource_name"),
                array(|| {
                    (
                        string().expected("name"),
                        be_i8().expected("config_operation"),
                        nullable_string().expected("value"),
                    )
                        .map(|(name, config_operation, value)| Configs {
                            name,
                            config_operation,
                            value,
                        })
                        .expected("configs")
                }),
            )
                .map(|(resource_type, resource_name, configs)| Resources {
                    resource_type,
                    resource_name,
                    configs,
                })
                .expected("resources")
        }),
        any().map(|b| b != 0).expected("validate_only"),
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
    pub resources: Vec<Resources<'i>>,
    pub validate_only: bool,
}

impl<'i> crate::Encode for IncrementalAlterConfigsRequest<'i> {
    fn encode_len(&self) -> usize {
        self.resources.encode_len() + self.validate_only.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.resources.encode(writer);
        self.validate_only.encode(writer);
    }
}

pub const VERSION: i16 = 0;

#[derive(Clone, Debug, PartialEq)]
pub struct Configs<'i> {
    pub name: &'i str,
    pub config_operation: i8,
    pub value: Option<&'i str>,
}

impl<'i> crate::Encode for Configs<'i> {
    fn encode_len(&self) -> usize {
        self.name.encode_len() + self.config_operation.encode_len() + self.value.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.name.encode(writer);
        self.config_operation.encode(writer);
        self.value.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Resources<'i> {
    pub resource_type: i8,
    pub resource_name: &'i str,
    pub configs: Vec<Configs<'i>>,
}

impl<'i> crate::Encode for Resources<'i> {
    fn encode_len(&self) -> usize {
        self.resource_type.encode_len()
            + self.resource_name.encode_len()
            + self.configs.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.resource_type.encode(writer);
        self.resource_name.encode(writer);
        self.configs.encode(writer);
    }
}
