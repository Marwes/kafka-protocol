use super::*;
pub fn incremental_alter_configs_response<'i, I>(
) -> impl Parser<I, Output = IncrementalAlterConfigsResponse<'i>> + 'i
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
            )
                .map(
                    |(error_code, error_message, resource_type, resource_name)| Responses {
                        error_code,
                        error_message,
                        resource_type,
                        resource_name,
                    },
                )
                .expected("responses")
        }),
    )
        .map(
            |(throttle_time_ms, responses)| IncrementalAlterConfigsResponse {
                throttle_time_ms,
                responses,
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct IncrementalAlterConfigsResponse<'i> {
    pub throttle_time_ms: i32,
    pub responses: Vec<Responses<'i>>,
}

impl<'i> crate::Encode for IncrementalAlterConfigsResponse<'i> {
    fn encode_len(&self) -> usize {
        self.throttle_time_ms.encode_len() + self.responses.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.throttle_time_ms.encode(writer);
        self.responses.encode(writer);
    }
}

pub const VERSION: i16 = 0;

#[derive(Clone, Debug, PartialEq)]
pub struct Responses<'i> {
    pub error_code: ErrorCode,
    pub error_message: Option<&'i str>,
    pub resource_type: i8,
    pub resource_name: &'i str,
}

impl<'i> crate::Encode for Responses<'i> {
    fn encode_len(&self) -> usize {
        self.error_code.encode_len()
            + self.error_message.encode_len()
            + self.resource_type.encode_len()
            + self.resource_name.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.error_code.encode(writer);
        self.error_message.encode(writer);
        self.resource_type.encode(writer);
        self.resource_name.encode(writer);
    }
}
