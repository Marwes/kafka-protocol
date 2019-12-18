use super::*;
pub fn describe_configs_request<'i, I>() -> impl Parser<I, Output = DescribeConfigsRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        array(|| {
            (be_i8(), string(), array(|| string())).map(
                |(resource_type, resource_name, config_names)| Resources {
                    resource_type,
                    resource_name,
                    config_names,
                },
            )
        }),
        any().map(|b| b != 0),
    )
        .map(|(resources, include_synonyms)| DescribeConfigsRequest {
            resources,
            include_synonyms,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct DescribeConfigsRequest<'i> {
    pub resources: Vec<Resources<'i>>,
    pub include_synonyms: bool,
}

impl<'i> crate::Encode for DescribeConfigsRequest<'i> {
    fn encode_len(&self) -> usize {
        self.resources.encode_len() + self.include_synonyms.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.resources.encode(writer);
        self.include_synonyms.encode(writer);
    }
}

pub const VERSION: i16 = 2;

#[derive(Clone, Debug, PartialEq)]
pub struct Resources<'i> {
    pub resource_type: i8,
    pub resource_name: &'i str,
    pub config_names: Vec<&'i str>,
}

impl<'i> crate::Encode for Resources<'i> {
    fn encode_len(&self) -> usize {
        self.resource_type.encode_len()
            + self.resource_name.encode_len()
            + self.config_names.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.resource_type.encode(writer);
        self.resource_name.encode(writer);
        self.config_names.encode(writer);
    }
}
