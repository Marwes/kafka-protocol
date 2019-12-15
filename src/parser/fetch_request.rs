use super::*;
pub fn fetch_request<'i, I>() -> impl Parser<I, Output = FetchRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i32(),
        be_i32(),
        be_i32(),
        be_i8(),
        be_i32(),
        be_i32(),
        optional(
            (
                string(),
                optional((be_i32(), be_i32(), be_i64(), be_i64(), be_i32()).map(
                    |(
                        partition,
                        current_leader_epoch,
                        fetch_offset,
                        log_start_offset,
                        partition_max_bytes,
                    )| {
                        Partitions {
                            partition,
                            current_leader_epoch,
                            fetch_offset,
                            log_start_offset,
                            partition_max_bytes,
                        }
                    },
                )),
            )
                .map(|(topic, partitions)| Topics { topic, partitions }),
        ),
        optional(
            (string(), optional(be_i32()))
                .map(|(topic, partitions)| ForgottenTopicsData { topic, partitions }),
        ),
        string(),
    )
        .map(
            |(
                replica_id,
                max_wait_time,
                min_bytes,
                max_bytes,
                isolation_level,
                session_id,
                session_epoch,
                topics,
                forgotten_topics_data,
                rack_id,
            )| {
                FetchRequest {
                    replica_id,
                    max_wait_time,
                    min_bytes,
                    max_bytes,
                    isolation_level,
                    session_id,
                    session_epoch,
                    topics,
                    forgotten_topics_data,
                    rack_id,
                }
            },
        )
}

#[derive(Clone, Debug, PartialEq)]
pub struct FetchRequest<'i> {
    pub replica_id: i32,
    pub max_wait_time: i32,
    pub min_bytes: i32,
    pub max_bytes: i32,
    pub isolation_level: i8,
    pub session_id: i32,
    pub session_epoch: i32,
    pub topics: Option<Topics<'i>>,
    pub forgotten_topics_data: Option<ForgottenTopicsData<'i>>,
    pub rack_id: &'i str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub partition: i32,
    pub current_leader_epoch: i32,
    pub fetch_offset: i64,
    pub log_start_offset: i64,
    pub partition_max_bytes: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topics<'i> {
    pub topic: &'i str,
    pub partitions: Option<Partitions>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgottenTopicsData<'i> {
    pub topic: &'i str,
    pub partitions: Option<i32>,
}
