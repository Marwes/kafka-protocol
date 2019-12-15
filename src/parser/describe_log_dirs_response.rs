use super::*;
pub fn describe_log_dirs_response<'i, I>() -> impl Parser<I, Output = DescribeLogDirsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
            (
                be_i16(),
                string(),
                optional(
                    (
                        string(),
                        optional((be_i32(), be_i64(), be_i64(), any().map(|b| b != 0)).map(
                            |(partition, size, offset_lag, is_future)| Partitions {
                                partition,
                                size,
                                offset_lag,
                                is_future,
                            },
                        )),
                    )
                        .map(|(topic, partitions)| Topics { topic, partitions }),
                ),
            )
                .map(|(error_code, log_dir, topics)| LogDirs {
                    error_code,
                    log_dir,
                    topics,
                }),
        ),
    )
        .map(|(throttle_time_ms, log_dirs)| DescribeLogDirsResponse {
            throttle_time_ms,
            log_dirs,
        })
}

#[derive(Clone, Debug, PartialEq)]
pub struct DescribeLogDirsResponse<'i> {
    pub throttle_time_ms: i32,
    pub log_dirs: Option<LogDirs<'i>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub partition: i32,
    pub size: i64,
    pub offset_lag: i64,
    pub is_future: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub topic: &'i str,
    pub partitions: Option<Partitions>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LogDirs<'i> {
    pub error_code: i16,
    pub log_dir: &'i str,
    pub topics: Option<Topics<'i>>,
}
