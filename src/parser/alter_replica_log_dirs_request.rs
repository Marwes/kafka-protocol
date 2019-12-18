use super::*;
pub fn alter_replica_log_dirs_request<'i, I>(
) -> impl Parser<I, Output = AlterReplicaLogDirsRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (array(|| {
        (
            string(),
            array(|| {
                (string(), array(|| be_i32()))
                    .map(|(topic, partitions)| Topics { topic, partitions })
            }),
        )
            .map(|(log_dir, topics)| LogDirs { log_dir, topics })
    }),)
        .map(|(log_dirs,)| AlterReplicaLogDirsRequest { log_dirs })
}

#[derive(Clone, Debug, PartialEq)]
pub struct AlterReplicaLogDirsRequest<'i> {
    pub log_dirs: Vec<LogDirs<'i>>,
}

impl<'i> crate::Encode for AlterReplicaLogDirsRequest<'i> {
    fn encode_len(&self) -> usize {
        self.log_dirs.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.log_dirs.encode(writer);
    }
}

pub const VERSION: i16 = 1;

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub topic: &'i str,
    pub partitions: Vec<i32>,
}

impl<'i> crate::Encode for Topics<'i> {
    fn encode_len(&self) -> usize {
        self.topic.encode_len() + self.partitions.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.topic.encode(writer);
        self.partitions.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LogDirs<'i> {
    pub log_dir: &'i str,
    pub topics: Vec<Topics<'i>>,
}

impl<'i> crate::Encode for LogDirs<'i> {
    fn encode_len(&self) -> usize {
        self.log_dir.encode_len() + self.topics.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.log_dir.encode(writer);
        self.topics.encode(writer);
    }
}
