use super::*;
pub fn member_assignment<'i, I>() -> impl Parser<I, Output = MemberAssignment<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16().expected("Version"),
        array(|| {
            (
                string().expected("Topic"),
                array(|| be_i32().expected("Partition").expected("Partition")),
            )
                .map(|(topic, partition)| Assignment { topic, partition })
                .expected("PartitionAssignment")
        }),
    )
        .map(|(version, partition_assignment)| MemberAssignment {
            version,
            partition_assignment,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct MemberAssignment<'i> {
    pub version: i16,
    pub partition_assignment: Vec<Assignment<'i>>,
}

impl<'i> crate::Encode for MemberAssignment<'i> {
    fn encode_len(&self) -> usize {
        self.version.encode_len() + self.partition_assignment.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.version.encode(writer);
        self.partition_assignment.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Assignment<'i> {
    pub topic: &'i str,
    pub partition: Vec<i32>,
}

impl<'i> crate::Encode for Assignment<'i> {
    fn encode_len(&self) -> usize {
        self.topic.encode_len() + self.partition.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.topic.encode(writer);
        self.partition.encode(writer);
    }
}
