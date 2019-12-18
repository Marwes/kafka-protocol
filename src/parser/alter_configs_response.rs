use super::*;
pub fn alter_configs_response<'i, I>() -> impl Parser<I, Output = AlterConfigsResponse<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        array(|| {
            (
                be_i16().and_then(|i| {
                    ErrorCode::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message)
                }),
                nullable_string(),
                be_i8(),
                string(),
            )
                .map(
                    |(error_code, error_message, resource_type, resource_name)| Resources {
                        error_code,
                        error_message,
                        resource_type,
                        resource_name,
                    },
                )
        }),
    )
        .map(|(throttle_time_ms, resources)| AlterConfigsResponse {
            throttle_time_ms,
            resources,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct AlterConfigsResponse<'i> {
    pub throttle_time_ms: i32,
    pub resources: Vec<Resources<'i>>,
}

impl<'i> crate::Encode for AlterConfigsResponse<'i> {
    fn encode_len(&self) -> usize {
        self.throttle_time_ms.encode_len() + self.resources.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.throttle_time_ms.encode(writer);
        self.resources.encode(writer);
    }
}

pub const VERSION: i16 = 1;

#[derive(Clone, Debug, PartialEq)]
pub struct Resources<'i> {
    pub error_code: ErrorCode,
    pub error_message: Option<&'i str>,
    pub resource_type: i8,
    pub resource_name: &'i str,
}

impl<'i> crate::Encode for Resources<'i> {
    fn encode_len(&self) -> usize {
        self.error_code.encode_len()
            + self.error_message.encode_len()
            + self.resource_type.encode_len()
            + self.resource_name.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.error_code.encode(writer);
        self.error_message.encode(writer);
        self.resource_type.encode(writer);
        self.resource_name.encode(writer);
    }
}
