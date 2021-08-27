use super::*;
pub fn join_group_request<'i, I>() -> impl Parser<I, Output = JoinGroupRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        string().expected("group_id"),
        be_i32().expected("session_timeout_ms"),
        string().expected("member_id"),
        nullable_string().expected("group_instance_id"),
        string().expected("protocol_type"),
        array(|| {
            (string().expected("name"), bytes().expected("metadata"))
                .map(|(name, metadata)| Protocols { name, metadata })
                .expected("protocols")
        }),
        array(|| {
            (string().expected("name"), bytes().expected("metadata"))
                .map(|(name, metadata)| Protocols { name, metadata })
                .expected("protocols")
        }),
    )
        .map(
            |(
                group_id,
                session_timeout_ms,
                member_id,
                group_instance_id,
                protocol_type,
                protocols,
                protocols,
            )| {
                JoinGroupRequest {
                    group_id,
                    session_timeout_ms,
                    member_id,
                    group_instance_id,
                    protocol_type,
                    protocols,
                    protocols,
                }
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct JoinGroupRequest<'i> {
    pub group_id: &'i str,
    pub session_timeout_ms: i32,
    pub member_id: &'i str,
    pub group_instance_id: Option<&'i str>,
    pub protocol_type: &'i str,
    pub protocols: Vec<Protocols<'i>>,
    pub protocols: Vec<Protocols<'i>>,
}

impl<'i> crate::Encode for JoinGroupRequest<'i> {
    fn encode_len(&self) -> usize {
        self.group_id.encode_len()
            + self.session_timeout_ms.encode_len()
            + self.member_id.encode_len()
            + self.group_instance_id.encode_len()
            + self.protocol_type.encode_len()
            + self.protocols.encode_len()
            + self.protocols.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.group_id.encode(writer);
        self.session_timeout_ms.encode(writer);
        self.member_id.encode(writer);
        self.group_instance_id.encode(writer);
        self.protocol_type.encode(writer);
        self.protocols.encode(writer);
        self.protocols.encode(writer);
    }
}

pub const VERSION: i16 = 0;

#[derive(Clone, Debug, PartialEq)]
pub struct Protocols<'i> {
    pub name: &'i str,
    pub metadata: &'i [u8],
}

impl<'i> crate::Encode for Protocols<'i> {
    fn encode_len(&self) -> usize {
        self.name.encode_len() + self.metadata.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.name.encode(writer);
        self.metadata.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Protocols<'i> {
    pub name: &'i str,
    pub metadata: &'i [u8],
}

impl<'i> crate::Encode for Protocols<'i> {
    fn encode_len(&self) -> usize {
        self.name.encode_len() + self.metadata.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.name.encode(writer);
        self.metadata.encode(writer);
    }
}
