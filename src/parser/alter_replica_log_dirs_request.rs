use super::*;
pub fn alter_replica_log_dirs_request<'i, I>(
) -> impl Parser<I, Output = AlterReplicaLogDirsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (optional(
        (
            string(),
            optional(
                (string(), optional(be_i32()))
                    .map(|(topic, partitions)| Topics { topic, partitions }),
            ),
        )
            .map(|(log_dir, topics)| LogDirs { log_dir, topics }),
    ),)
        .map(|(log_dirs,)| AlterReplicaLogDirsRequest { log_dirs })
}

#[derive(Clone, Debug, PartialEq)]
pub struct AlterReplicaLogDirsRequest<'i> {
    pub log_dirs: Option<LogDirs<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub topic: &'i str,
    pub partitions: Option<i32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LogDirs<'i> {
    pub log_dir: &'i str,
    pub topics: Option<Topics<'i>>,
}
