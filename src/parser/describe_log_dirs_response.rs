use super::*;
pub fn describe_log_dirs_response<'i, I>(
) -> impl Parser<I, Output = DescribeLogDirsResponse<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        array(|| {
            (
                be_i16().and_then(|i| {
                    ErrorCode::try_from(i).map_err(StreamErrorFor::<I>::unexpected_static_message)
                }),
                string(),
                array(|| {
                    (
                        string(),
                        array(|| {
                            (be_i32(), be_i64(), be_i64(), any().map(|b| b != 0)).map(
                                |(partition, size, offset_lag, is_future)| Partitions {
                                    partition,
                                    size,
                                    offset_lag,
                                    is_future,
                                },
                            )
                        }),
                    )
                        .map(|(topic, partitions)| Topics { topic, partitions })
                }),
            )
                .map(|(error_code, log_dir, topics)| LogDirs {
                    error_code,
                    log_dir,
                    topics,
                })
        }),
    )
        .map(|(throttle_time_ms, log_dirs)| DescribeLogDirsResponse {
            throttle_time_ms,
            log_dirs,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct DescribeLogDirsResponse<'i> {
    pub throttle_time_ms: i32,
    pub log_dirs: Vec<LogDirs<'i>>,
}

impl<'i> crate::Encode for DescribeLogDirsResponse<'i> {
    fn encode_len(&self) -> usize {
        self.throttle_time_ms.encode_len() + self.log_dirs.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.throttle_time_ms.encode(writer);
        self.log_dirs.encode(writer);
    }
}

pub const VERSION: i16 = 1;

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub partition: i32,
    pub size: i64,
    pub offset_lag: i64,
    pub is_future: bool,
}

impl crate::Encode for Partitions {
    fn encode_len(&self) -> usize {
        self.partition.encode_len()
            + self.size.encode_len()
            + self.offset_lag.encode_len()
            + self.is_future.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.partition.encode(writer);
        self.size.encode(writer);
        self.offset_lag.encode(writer);
        self.is_future.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub topic: &'i str,
    pub partitions: Vec<Partitions>,
}

impl<'i> crate::Encode for Topics<'i> {
    fn encode_len(&self) -> usize {
        self.topic.encode_len() + self.partitions.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.topic.encode(writer);
        self.partitions.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LogDirs<'i> {
    pub error_code: ErrorCode,
    pub log_dir: &'i str,
    pub topics: Vec<Topics<'i>>,
}

impl<'i> crate::Encode for LogDirs<'i> {
    fn encode_len(&self) -> usize {
        self.error_code.encode_len() + self.log_dir.encode_len() + self.topics.encode_len()
    }
    fn encode(&self, writer: &mut impl Buffer) {
        self.error_code.encode(writer);
        self.log_dir.encode(writer);
        self.topics.encode(writer);
    }
}
