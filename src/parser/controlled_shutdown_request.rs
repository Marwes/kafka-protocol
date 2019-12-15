use super::*;
pub fn controlled_shutdown_request<'i, I>() -> impl Parser<I, Output = ControlledShutdownRequest>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (be_i32(), be_i64()).map(|(broker_id, broker_epoch)| ControlledShutdownRequest {
        broker_id,
        broker_epoch,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct ControlledShutdownRequest {
    pub broker_id: i32,
    pub broker_epoch: i64,
}
