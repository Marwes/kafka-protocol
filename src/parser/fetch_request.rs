use super::*;
pub fn fetch_request<'i, I>() -> impl Parser<I, Output = FetchRequest<'i>> + 'i
where
    I: RangeStream<Token = u8, Range = &'i [u8]> + 'i,
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
        array(|| {
            (
                string(),
                array(|| {
                    (be_i32(), be_i32(), be_i64(), be_i64(), be_i32()).map(
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
                    )
                }),
            )
                .map(|(topic, partitions)| Topics { topic, partitions })
        }),
        array(|| {
            (string(), array(|| be_i32()))
                .map(|(topic, partitions)| ForgottenTopicsData { topic, partitions })
        }),
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
    pub topics: Vec<Topics<'i>>,
    pub forgotten_topics_data: Vec<ForgottenTopicsData<'i>>,
    pub rack_id: &'i str,
}

impl<'i> crate::Encode for FetchRequest<'i> {
    fn encode_len(&self) -> usize {
        self.replica_id.encode_len()
            + self.max_wait_time.encode_len()
            + self.min_bytes.encode_len()
            + self.max_bytes.encode_len()
            + self.isolation_level.encode_len()
            + self.session_id.encode_len()
            + self.session_epoch.encode_len()
            + self.topics.encode_len()
            + self.forgotten_topics_data.encode_len()
            + self.rack_id.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.replica_id.encode(writer);
        self.max_wait_time.encode(writer);
        self.min_bytes.encode(writer);
        self.max_bytes.encode(writer);
        self.isolation_level.encode(writer);
        self.session_id.encode(writer);
        self.session_epoch.encode(writer);
        self.topics.encode(writer);
        self.forgotten_topics_data.encode(writer);
        self.rack_id.encode(writer);
    }
}

pub const VERSION: i16 = 11;

#[derive(Clone, Debug, PartialEq)]
pub struct Partitions {
    pub partition: i32,
    pub current_leader_epoch: i32,
    pub fetch_offset: i64,
    pub log_start_offset: i64,
    pub partition_max_bytes: i32,
}

impl crate::Encode for Partitions {
    fn encode_len(&self) -> usize {
        self.partition.encode_len()
            + self.current_leader_epoch.encode_len()
            + self.fetch_offset.encode_len()
            + self.log_start_offset.encode_len()
            + self.partition_max_bytes.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.partition.encode(writer);
        self.current_leader_epoch.encode(writer);
        self.fetch_offset.encode(writer);
        self.log_start_offset.encode(writer);
        self.partition_max_bytes.encode(writer);
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
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.topic.encode(writer);
        self.partitions.encode(writer);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgottenTopicsData<'i> {
    pub topic: &'i str,
    pub partitions: Vec<i32>,
}

impl<'i> crate::Encode for ForgottenTopicsData<'i> {
    fn encode_len(&self) -> usize {
        self.topic.encode_len() + self.partitions.encode_len()
    }
    fn encode(&self, writer: &mut impl bytes::BufMut) {
        self.topic.encode(writer);
        self.partitions.encode(writer);
    }
}
