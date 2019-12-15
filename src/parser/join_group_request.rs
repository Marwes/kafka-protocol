use super::*;
pub fn join_group_request<'i, I>() -> impl Parser<I, Output = JoinGroupRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        string(),
        be_i32(),
        be_i32(),
        string(),
        nullable_string(),
        string(),
        optional((string(), bytes()).map(|(name, metadata)| Protocols { name, metadata })),
    )
        .map(
            |(
                group_id,
                session_timeout_ms,
                rebalance_timeout_ms,
                member_id,
                group_instance_id,
                protocol_type,
                protocols,
            )| {
                JoinGroupRequest {
                    group_id,
                    session_timeout_ms,
                    rebalance_timeout_ms,
                    member_id,
                    group_instance_id,
                    protocol_type,
                    protocols,
                }
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct JoinGroupRequest<'i> {
    pub group_id: &'i str,
    pub session_timeout_ms: i32,
    pub rebalance_timeout_ms: i32,
    pub member_id: &'i str,
    pub group_instance_id: Option<&'i str>,
    pub protocol_type: &'i str,
    pub protocols: Option<Protocols<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Protocols<'i> {
    pub name: &'i str,
    pub metadata: &'i [u8],
}
