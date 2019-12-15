use super::*;
pub fn describe_log_dirs_request<'i, I>() -> impl Parser<I, Output = DescribeLogDirsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (optional(
        (string(), optional(be_i32())).map(|(topic, partitions)| Topics { topic, partitions }),
    ),)
        .map(|(topics,)| DescribeLogDirsRequest { topics })
}

#[derive(Clone, Debug, PartialEq)]
pub struct DescribeLogDirsRequest<'i> {
    pub topics: Option<Topics<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub topic: &'i str,
    pub partitions: Option<i32>,
}
