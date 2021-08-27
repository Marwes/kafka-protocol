use super::*;
pub fn delete_topics_response<'i, I>() -> impl Parser<I, Output = DeleteTopicsResponse<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        array(|| {
            (
                string().expected("name"),
                be_i16()
                    .and_then(|i| {
                        ErrorCode::try_from(i)
                            .map_err(StreamErrorFor::<I>::unexpected_static_message)
                    })
                    .expected("error_code"),
            )
                .map(|(name, error_code)| Responses { name, error_code })
                .expected("responses")
        }),
        array(|| {
            (
                string().expected("name"),
                be_i16()
                    .and_then(|i| {
                        ErrorCode::try_from(i)
                            .map_err(StreamErrorFor::<I>::unexpected_static_message)
                    })
                    .expected("error_code"),
            )
                .map(|(name, error_code)| Responses { name, error_code })
                .expected("responses")
        }),
    )
        .map(|(responses, responses)| DeleteTopicsResponse {
            responses,
            responses,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeleteTopicsResponse<'i> {
    pub responses: Vec<Responses<'i>>,
    pub responses: Vec<Responses<'i>>,
}

impl<'i> crate::Encode for DeleteTopicsResponse<'i> {
    fn encode_len(&self) -> usize {
        self.responses.encode_len() + self.responses.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.responses.encode(writer);
        self.responses.encode(writer);
    }
}

pub const VERSION: i16 = 0;

#[derive(Clone, Debug, PartialEq)]
pub struct Responses<'i> {
    pub name: &'i str,
    pub error_code: ErrorCode,
}

impl<'i> crate::Encode for Responses<'i> {
    fn encode_len(&self) -> usize {
        self.name.encode_len() + self.error_code.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.name.encode(writer);
        self.error_code.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Responses<'i> {
    pub name: &'i str,
    pub error_code: ErrorCode,
}

impl<'i> crate::Encode for Responses<'i> {
    fn encode_len(&self) -> usize {
        self.name.encode_len() + self.error_code.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.name.encode(writer);
        self.error_code.encode(writer);
    }
}
