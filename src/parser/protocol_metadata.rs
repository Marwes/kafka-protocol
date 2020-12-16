use super::*;
pub fn protocol_metadata<'i, I>() -> impl Parser<I, Output = ProtocolMetadata<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16().expected("Version"),
        array(|| string().expected("Topic").expected("Subscription")),
        bytes().expected("UserData"),
    )
        .map(|(version, subscription, user_data)| ProtocolMetadata {
            version,
            subscription,
            user_data,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProtocolMetadata<'i> {
    pub version: i16,
    pub subscription: Vec<&'i str>,
    pub user_data: &'i [u8],
}

impl<'i> crate::Encode for ProtocolMetadata<'i> {
    fn encode_len(&self) -> usize {
        self.version.encode_len() + self.subscription.encode_len() + self.user_data.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.version.encode(writer);
        self.subscription.encode(writer);
        self.user_data.encode(writer);
    }
}
