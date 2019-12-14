use super::*;
fn produce_request<'i, I>() -> impl Parser<I, Output = ProduceRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        nullable_string(),
        be_i16(),
        be_i32(),
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
                nullable_bytes(),
            ).map(|(partition,record_set)| {Data{
            partition,record_set
            }})),
        ).map(|(topic,data)| {TopicData{
        topic,data
        }})),
    ).map(|(transactional_id,acks,timeout,topic_data)| {ProduceRequest{
    transactional_id,acks,timeout,topic_data
    }})
}

pub struct ProduceRequest<'i> {

    transactional_id: Option<&'i str>,
    acks: i16,
    timeout: i32,
    topic_data: Option<TopicData<'i>>,
}

pub struct Data<'i> {

    partition: i32,
    record_set: Option<&'i [u8]>,
}

pub struct TopicData<'i> {

    topic: &'i str,
    data: Option<Data<'i>>,
}
use super::*;
fn produce_response<'i, I>() -> impl Parser<I, Output = ProduceResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
                be_i16(),
                be_i64(),
                be_i64(),
                be_i64(),
            ).map(|(partition,error_code,base_offset,log_append_time,log_start_offset)| {PartitionResponses{
            partition,error_code,base_offset,log_append_time,log_start_offset
            }})),
        ).map(|(topic,partition_responses)| {Responses{
        topic,partition_responses
        }})),
        be_i32(),
    ).map(|(responses,throttle_time_ms)| {ProduceResponse{
    responses,throttle_time_ms
    }})
}

pub struct ProduceResponse<'i> {

    responses: Option<Responses<'i>>,
    throttle_time_ms: i32,
}

pub struct PartitionResponses<'i> {

    partition: i32,
    error_code: i16,
    base_offset: i64,
    log_append_time: i64,
    log_start_offset: i64,
}

pub struct Responses<'i> {

    topic: &'i str,
    partition_responses: Option<PartitionResponses<'i>>,
}
use super::*;
fn fetch_request<'i, I>() -> impl Parser<I, Output = FetchRequest<'i>>
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
            optional(
            (
                be_i32(),
                be_i32(),
                be_i64(),
                be_i64(),
                be_i32(),
            ).map(|(partition,current_leader_epoch,fetch_offset,log_start_offset,partition_max_bytes)| {Partitions{
            partition,current_leader_epoch,fetch_offset,log_start_offset,partition_max_bytes
            }})),
        ).map(|(topic,partitions)| {Topics{
        topic,partitions
        }})),
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
            ).map(|(INT32)| {Partitions{
            INT32
            }})),
        ).map(|(topic,partitions)| {ForgottenTopicsData{
        topic,partitions
        }})),
        string(),
    ).map(|(replica_id,max_wait_time,min_bytes,max_bytes,isolation_level,session_id,session_epoch,topics,forgotten_topics_data,rack_id)| {FetchRequest{
    replica_id,max_wait_time,min_bytes,max_bytes,isolation_level,session_id,session_epoch,topics,forgotten_topics_data,rack_id
    }})
}

pub struct FetchRequest<'i> {

    replica_id: i32,
    max_wait_time: i32,
    min_bytes: i32,
    max_bytes: i32,
    isolation_level: i8,
    session_id: i32,
    session_epoch: i32,
    topics: Option<Topics<'i>>,
    forgotten_topics_data: Option<ForgottenTopicsData<'i>>,
    rack_id: &'i str,
}

pub struct Partitions<'i> {

    partition: i32,
    current_leader_epoch: i32,
    fetch_offset: i64,
    log_start_offset: i64,
    partition_max_bytes: i32,
}

pub struct Topics<'i> {

    topic: &'i str,
    partitions: Option<Partitions<'i>>,
}

pub struct Partitions<'i> {

    INT32: i32,
}

pub struct ForgottenTopicsData<'i> {

    topic: &'i str,
    partitions: Option<Partitions<'i>>,
}
use super::*;
fn fetch_response<'i, I>() -> impl Parser<I, Output = FetchResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i16(),
        be_i32(),
        optional(
        (
            string(),
            optional(
            (
                    partition(),
                nullable_bytes(),
            ).map(|(partition_header,record_set)| {PartitionResponses{
            partition_header,record_set
            }})),
        ).map(|(topic,partition_responses)| {Responses{
        topic,partition_responses
        }})),
    ).map(|(throttle_time_ms,error_code,session_id,responses)| {FetchResponse{
    throttle_time_ms,error_code,session_id,responses
    }})
}

pub struct FetchResponse<'i> {

    throttle_time_ms: i32,
    error_code: i16,
    session_id: i32,
    responses: Option<Responses<'i>>,
}

pub struct PartitionResponses<'i> {

    partition_header: partition,
    record_set: Option<&'i [u8]>,
}

pub struct Responses<'i> {

    topic: &'i str,
    partition_responses: Option<PartitionResponses<'i>>,
}
use super::*;
fn list_offsets_request<'i, I>() -> impl Parser<I, Output = ListOffsetsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i8(),
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
                be_i32(),
                be_i64(),
            ).map(|(partition,current_leader_epoch,timestamp)| {Partitions{
            partition,current_leader_epoch,timestamp
            }})),
        ).map(|(topic,partitions)| {Topics{
        topic,partitions
        }})),
    ).map(|(replica_id,isolation_level,topics)| {ListOffsetsRequest{
    replica_id,isolation_level,topics
    }})
}

pub struct ListOffsetsRequest<'i> {

    replica_id: i32,
    isolation_level: i8,
    topics: Option<Topics<'i>>,
}

pub struct Partitions<'i> {

    partition: i32,
    current_leader_epoch: i32,
    timestamp: i64,
}

pub struct Topics<'i> {

    topic: &'i str,
    partitions: Option<Partitions<'i>>,
}
use super::*;
fn list_offsets_response<'i, I>() -> impl Parser<I, Output = ListOffsetsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
                be_i16(),
                be_i64(),
                be_i64(),
                be_i32(),
            ).map(|(partition,error_code,timestamp,offset,leader_epoch)| {PartitionResponses{
            partition,error_code,timestamp,offset,leader_epoch
            }})),
        ).map(|(topic,partition_responses)| {Responses{
        topic,partition_responses
        }})),
    ).map(|(throttle_time_ms,responses)| {ListOffsetsResponse{
    throttle_time_ms,responses
    }})
}

pub struct ListOffsetsResponse<'i> {

    throttle_time_ms: i32,
    responses: Option<Responses<'i>>,
}

pub struct PartitionResponses<'i> {

    partition: i32,
    error_code: i16,
    timestamp: i64,
    offset: i64,
    leader_epoch: i32,
}

pub struct Responses<'i> {

    topic: &'i str,
    partition_responses: Option<PartitionResponses<'i>>,
}
use super::*;
fn metadata_request<'i, I>() -> impl Parser<I, Output = MetadataRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional(
        (
            string(),
        ).map(|(name)| {Topics{
        name
        }})),
        any().map(|b| b != 0),
        any().map(|b| b != 0),
        any().map(|b| b != 0),
    ).map(|(topics,allow_auto_topic_creation,include_cluster_authorized_operations,include_topic_authorized_operations)| {MetadataRequest{
    topics,allow_auto_topic_creation,include_cluster_authorized_operations,include_topic_authorized_operations
    }})
}

pub struct MetadataRequest<'i> {

    topics: Option<Topics<'i>>,
    allow_auto_topic_creation: bool,
    include_cluster_authorized_operations: bool,
    include_topic_authorized_operations: bool,
}

pub struct Topics<'i> {

    name: &'i str,
}
use super::*;
fn metadata_response<'i, I>() -> impl Parser<I, Output = MetadataResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
        (
            be_i32(),
            string(),
            be_i32(),
            nullable_string(),
        ).map(|(node_id,host,port,rack)| {Brokers{
        node_id,host,port,rack
        }})),
        nullable_string(),
        be_i32(),
        optional(
        (
            be_i16(),
            string(),
            any().map(|b| b != 0),
            optional(
            (
                be_i16(),
                be_i32(),
                be_i32(),
                be_i32(),
                optional(
                (
                    be_i32(),
                ).map(|(INT32)| {ReplicaNodes{
                INT32
                }})),
                optional(
                (
                    be_i32(),
                ).map(|(INT32)| {IsrNodes{
                INT32
                }})),
                optional(
                (
                    be_i32(),
                ).map(|(INT32)| {OfflineReplicas{
                INT32
                }})),
            ).map(|(error_code,partition_index,leader_id,leader_epoch,replica_nodes,isr_nodes,offline_replicas)| {Partitions{
            error_code,partition_index,leader_id,leader_epoch,replica_nodes,isr_nodes,offline_replicas
            }})),
            be_i32(),
        ).map(|(error_code,name,is_internal,partitions,topic_authorized_operations)| {Topics{
        error_code,name,is_internal,partitions,topic_authorized_operations
        }})),
        be_i32(),
    ).map(|(throttle_time_ms,brokers,cluster_id,controller_id,topics,cluster_authorized_operations)| {MetadataResponse{
    throttle_time_ms,brokers,cluster_id,controller_id,topics,cluster_authorized_operations
    }})
}

pub struct MetadataResponse<'i> {

    throttle_time_ms: i32,
    brokers: Option<Brokers<'i>>,
    cluster_id: Option<&'i str>,
    controller_id: i32,
    topics: Option<Topics<'i>>,
    cluster_authorized_operations: i32,
}

pub struct Brokers<'i> {

    node_id: i32,
    host: &'i str,
    port: i32,
    rack: Option<&'i str>,
}

pub struct ReplicaNodes<'i> {

    INT32: i32,
}

pub struct IsrNodes<'i> {

    INT32: i32,
}

pub struct OfflineReplicas<'i> {

    INT32: i32,
}

pub struct Partitions<'i> {

    error_code: i16,
    partition_index: i32,
    leader_id: i32,
    leader_epoch: i32,
    replica_nodes: Option<ReplicaNodes<'i>>,
    isr_nodes: Option<IsrNodes<'i>>,
    offline_replicas: Option<OfflineReplicas<'i>>,
}

pub struct Topics<'i> {

    error_code: i16,
    name: &'i str,
    is_internal: bool,
    partitions: Option<Partitions<'i>>,
    topic_authorized_operations: i32,
}
use super::*;
fn leader_and_isr_request<'i, I>() -> impl Parser<I, Output = LeaderAndIsrRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i32(),
        be_i64(),
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
                be_i32(),
                be_i32(),
                be_i32(),
                optional(
                (
                    be_i32(),
                ).map(|(INT32)| {Isr{
                INT32
                }})),
                be_i32(),
                optional(
                (
                    be_i32(),
                ).map(|(INT32)| {Replicas{
                INT32
                }})),
                any().map(|b| b != 0),
            ).map(|(partition,controller_epoch,leader,leader_epoch,isr,zk_version,replicas,is_new)| {PartitionStates{
            partition,controller_epoch,leader,leader_epoch,isr,zk_version,replicas,is_new
            }})),
        ).map(|(topic,partition_states)| {TopicStates{
        topic,partition_states
        }})),
        optional(
        (
            be_i32(),
            string(),
            be_i32(),
        ).map(|(id,host,port)| {LiveLeaders{
        id,host,port
        }})),
    ).map(|(controller_id,controller_epoch,broker_epoch,topic_states,live_leaders)| {LeaderAndIsrRequest{
    controller_id,controller_epoch,broker_epoch,topic_states,live_leaders
    }})
}

pub struct LeaderAndIsrRequest<'i> {

    controller_id: i32,
    controller_epoch: i32,
    broker_epoch: i64,
    topic_states: Option<TopicStates<'i>>,
    live_leaders: Option<LiveLeaders<'i>>,
}

pub struct Isr<'i> {

    INT32: i32,
}

pub struct Replicas<'i> {

    INT32: i32,
}

pub struct PartitionStates<'i> {

    partition: i32,
    controller_epoch: i32,
    leader: i32,
    leader_epoch: i32,
    isr: Option<Isr<'i>>,
    zk_version: i32,
    replicas: Option<Replicas<'i>>,
    is_new: bool,
}

pub struct TopicStates<'i> {

    topic: &'i str,
    partition_states: Option<PartitionStates<'i>>,
}

pub struct LiveLeaders<'i> {

    id: i32,
    host: &'i str,
    port: i32,
}
use super::*;
fn leader_and_isr_response<'i, I>() -> impl Parser<I, Output = LeaderAndIsrResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16(),
        optional(
        (
            string(),
            be_i32(),
            be_i16(),
        ).map(|(topic,partition,error_code)| {Partitions{
        topic,partition,error_code
        }})),
    ).map(|(error_code,partitions)| {LeaderAndIsrResponse{
    error_code,partitions
    }})
}

pub struct LeaderAndIsrResponse<'i> {

    error_code: i16,
    partitions: Option<Partitions<'i>>,
}

pub struct Partitions<'i> {

    topic: &'i str,
    partition: i32,
    error_code: i16,
}
use super::*;
fn stop_replica_request<'i, I>() -> impl Parser<I, Output = StopReplicaRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i32(),
        be_i64(),
        any().map(|b| b != 0),
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
            ).map(|(INT32)| {PartitionIds{
            INT32
            }})),
        ).map(|(topic,partition_ids)| {Partitions{
        topic,partition_ids
        }})),
    ).map(|(controller_id,controller_epoch,broker_epoch,delete_partitions,partitions)| {StopReplicaRequest{
    controller_id,controller_epoch,broker_epoch,delete_partitions,partitions
    }})
}

pub struct StopReplicaRequest<'i> {

    controller_id: i32,
    controller_epoch: i32,
    broker_epoch: i64,
    delete_partitions: bool,
    partitions: Option<Partitions<'i>>,
}

pub struct PartitionIds<'i> {

    INT32: i32,
}

pub struct Partitions<'i> {

    topic: &'i str,
    partition_ids: Option<PartitionIds<'i>>,
}
use super::*;
fn stop_replica_response<'i, I>() -> impl Parser<I, Output = StopReplicaResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16(),
        optional(
        (
            string(),
            be_i32(),
            be_i16(),
        ).map(|(topic,partition,error_code)| {Partitions{
        topic,partition,error_code
        }})),
    ).map(|(error_code,partitions)| {StopReplicaResponse{
    error_code,partitions
    }})
}

pub struct StopReplicaResponse<'i> {

    error_code: i16,
    partitions: Option<Partitions<'i>>,
}

pub struct Partitions<'i> {

    topic: &'i str,
    partition: i32,
    error_code: i16,
}
use super::*;
fn update_metadata_request<'i, I>() -> impl Parser<I, Output = UpdateMetadataRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i32(),
        be_i64(),
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
                be_i32(),
                be_i32(),
                be_i32(),
                optional(
                (
                    be_i32(),
                ).map(|(INT32)| {Isr{
                INT32
                }})),
                be_i32(),
                optional(
                (
                    be_i32(),
                ).map(|(INT32)| {Replicas{
                INT32
                }})),
                optional(
                (
                    be_i32(),
                ).map(|(INT32)| {OfflineReplicas{
                INT32
                }})),
            ).map(|(partition,controller_epoch,leader,leader_epoch,isr,zk_version,replicas,offline_replicas)| {PartitionStates{
            partition,controller_epoch,leader,leader_epoch,isr,zk_version,replicas,offline_replicas
            }})),
        ).map(|(topic,partition_states)| {TopicStates{
        topic,partition_states
        }})),
        optional(
        (
            be_i32(),
            optional(
            (
                be_i32(),
                string(),
                string(),
                be_i16(),
            ).map(|(port,host,listener_name,security_protocol_type)| {EndPoints{
            port,host,listener_name,security_protocol_type
            }})),
            nullable_string(),
        ).map(|(id,end_points,rack)| {LiveBrokers{
        id,end_points,rack
        }})),
    ).map(|(controller_id,controller_epoch,broker_epoch,topic_states,live_brokers)| {UpdateMetadataRequest{
    controller_id,controller_epoch,broker_epoch,topic_states,live_brokers
    }})
}

pub struct UpdateMetadataRequest<'i> {

    controller_id: i32,
    controller_epoch: i32,
    broker_epoch: i64,
    topic_states: Option<TopicStates<'i>>,
    live_brokers: Option<LiveBrokers<'i>>,
}

pub struct Isr<'i> {

    INT32: i32,
}

pub struct Replicas<'i> {

    INT32: i32,
}

pub struct OfflineReplicas<'i> {

    INT32: i32,
}

pub struct PartitionStates<'i> {

    partition: i32,
    controller_epoch: i32,
    leader: i32,
    leader_epoch: i32,
    isr: Option<Isr<'i>>,
    zk_version: i32,
    replicas: Option<Replicas<'i>>,
    offline_replicas: Option<OfflineReplicas<'i>>,
}

pub struct TopicStates<'i> {

    topic: &'i str,
    partition_states: Option<PartitionStates<'i>>,
}

pub struct EndPoints<'i> {

    port: i32,
    host: &'i str,
    listener_name: &'i str,
    security_protocol_type: i16,
}

pub struct LiveBrokers<'i> {

    id: i32,
    end_points: Option<EndPoints<'i>>,
    rack: Option<&'i str>,
}
use super::*;
fn update_metadata_response<'i, I>() -> impl Parser<I, Output = UpdateMetadataResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16(),
    ).map(|(error_code)| {UpdateMetadataResponse{
    error_code
    }})
}

pub struct UpdateMetadataResponse<'i> {

    error_code: i16,
}


use super::*;
fn controlled_shutdown_request<'i, I>() -> impl Parser<I, Output = ControlledShutdownRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i64(),
    ).map(|(broker_id,broker_epoch)| {ControlledShutdownRequest{
    broker_id,broker_epoch
    }})
}

pub struct ControlledShutdownRequest<'i> {

    broker_id: i32,
    broker_epoch: i64,
}


use super::*;
fn controlled_shutdown_response<'i, I>() -> impl Parser<I, Output = ControlledShutdownResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16(),
        optional(
        (
            string(),
            be_i32(),
        ).map(|(topic_name,partition_index)| {RemainingPartitions{
        topic_name,partition_index
        }})),
    ).map(|(error_code,remaining_partitions)| {ControlledShutdownResponse{
    error_code,remaining_partitions
    }})
}

pub struct ControlledShutdownResponse<'i> {

    error_code: i16,
    remaining_partitions: Option<RemainingPartitions<'i>>,
}

pub struct RemainingPartitions<'i> {

    topic_name: &'i str,
    partition_index: i32,
}
use super::*;
fn offset_commit_request<'i, I>() -> impl Parser<I, Output = OffsetCommitRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        string(),
        be_i32(),
        string(),
        nullable_string(),
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
                be_i64(),
                be_i32(),
                nullable_string(),
            ).map(|(partition_index,committed_offset,committed_leader_epoch,committed_metadata)| {Partitions{
            partition_index,committed_offset,committed_leader_epoch,committed_metadata
            }})),
        ).map(|(name,partitions)| {Topics{
        name,partitions
        }})),
    ).map(|(group_id,generation_id,member_id,group_instance_id,topics)| {OffsetCommitRequest{
    group_id,generation_id,member_id,group_instance_id,topics
    }})
}

pub struct OffsetCommitRequest<'i> {

    group_id: &'i str,
    generation_id: i32,
    member_id: &'i str,
    group_instance_id: Option<&'i str>,
    topics: Option<Topics<'i>>,
}

pub struct Partitions<'i> {

    partition_index: i32,
    committed_offset: i64,
    committed_leader_epoch: i32,
    committed_metadata: Option<&'i str>,
}

pub struct Topics<'i> {

    name: &'i str,
    partitions: Option<Partitions<'i>>,
}
use super::*;
fn offset_commit_response<'i, I>() -> impl Parser<I, Output = OffsetCommitResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
                be_i16(),
            ).map(|(partition_index,error_code)| {Partitions{
            partition_index,error_code
            }})),
        ).map(|(name,partitions)| {Topics{
        name,partitions
        }})),
    ).map(|(throttle_time_ms,topics)| {OffsetCommitResponse{
    throttle_time_ms,topics
    }})
}

pub struct OffsetCommitResponse<'i> {

    throttle_time_ms: i32,
    topics: Option<Topics<'i>>,
}

pub struct Partitions<'i> {

    partition_index: i32,
    error_code: i16,
}

pub struct Topics<'i> {

    name: &'i str,
    partitions: Option<Partitions<'i>>,
}
use super::*;
fn offset_fetch_request<'i, I>() -> impl Parser<I, Output = OffsetFetchRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        string(),
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
            ).map(|(partition)| {Partitions{
            partition
            }})),
        ).map(|(topic,partitions)| {Topics{
        topic,partitions
        }})),
    ).map(|(group_id,topics)| {OffsetFetchRequest{
    group_id,topics
    }})
}

pub struct OffsetFetchRequest<'i> {

    group_id: &'i str,
    topics: Option<Topics<'i>>,
}

pub struct Partitions<'i> {

    partition: i32,
}

pub struct Topics<'i> {

    topic: &'i str,
    partitions: Option<Partitions<'i>>,
}
use super::*;
fn offset_fetch_response<'i, I>() -> impl Parser<I, Output = OffsetFetchResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
                be_i64(),
                be_i32(),
                nullable_string(),
                be_i16(),
            ).map(|(partition,offset,leader_epoch,metadata,error_code)| {PartitionResponses{
            partition,offset,leader_epoch,metadata,error_code
            }})),
        ).map(|(topic,partition_responses)| {Responses{
        topic,partition_responses
        }})),
        be_i16(),
    ).map(|(throttle_time_ms,responses,error_code)| {OffsetFetchResponse{
    throttle_time_ms,responses,error_code
    }})
}

pub struct OffsetFetchResponse<'i> {

    throttle_time_ms: i32,
    responses: Option<Responses<'i>>,
    error_code: i16,
}

pub struct PartitionResponses<'i> {

    partition: i32,
    offset: i64,
    leader_epoch: i32,
    metadata: Option<&'i str>,
    error_code: i16,
}

pub struct Responses<'i> {

    topic: &'i str,
    partition_responses: Option<PartitionResponses<'i>>,
}
use super::*;
fn find_coordinator_request<'i, I>() -> impl Parser<I, Output = FindCoordinatorRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        string(),
        be_i8(),
    ).map(|(key,key_type)| {FindCoordinatorRequest{
    key,key_type
    }})
}

pub struct FindCoordinatorRequest<'i> {

    key: &'i str,
    key_type: i8,
}


use super::*;
fn find_coordinator_response<'i, I>() -> impl Parser<I, Output = FindCoordinatorResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i16(),
        nullable_string(),
        be_i32(),
        string(),
        be_i32(),
    ).map(|(throttle_time_ms,error_code,error_message,node_id,host,port)| {FindCoordinatorResponse{
    throttle_time_ms,error_code,error_message,node_id,host,port
    }})
}

pub struct FindCoordinatorResponse<'i> {

    throttle_time_ms: i32,
    error_code: i16,
    error_message: Option<&'i str>,
    node_id: i32,
    host: &'i str,
    port: i32,
}


use super::*;
fn join_group_request<'i, I>() -> impl Parser<I, Output = JoinGroupRequest<'i>>
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
        optional(
        (
            string(),
            bytes(),
        ).map(|(name,metadata)| {Protocols{
        name,metadata
        }})),
    ).map(|(group_id,session_timeout_ms,rebalance_timeout_ms,member_id,group_instance_id,protocol_type,protocols)| {JoinGroupRequest{
    group_id,session_timeout_ms,rebalance_timeout_ms,member_id,group_instance_id,protocol_type,protocols
    }})
}

pub struct JoinGroupRequest<'i> {

    group_id: &'i str,
    session_timeout_ms: i32,
    rebalance_timeout_ms: i32,
    member_id: &'i str,
    group_instance_id: Option<&'i str>,
    protocol_type: &'i str,
    protocols: Option<Protocols<'i>>,
}

pub struct Protocols<'i> {

    name: &'i str,
    metadata: &'i [u8],
}
use super::*;
fn join_group_response<'i, I>() -> impl Parser<I, Output = JoinGroupResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i16(),
        be_i32(),
        string(),
        string(),
        string(),
        optional(
        (
            string(),
            nullable_string(),
            bytes(),
        ).map(|(member_id,group_instance_id,metadata)| {Members{
        member_id,group_instance_id,metadata
        }})),
    ).map(|(throttle_time_ms,error_code,generation_id,protocol_name,leader,member_id,members)| {JoinGroupResponse{
    throttle_time_ms,error_code,generation_id,protocol_name,leader,member_id,members
    }})
}

pub struct JoinGroupResponse<'i> {

    throttle_time_ms: i32,
    error_code: i16,
    generation_id: i32,
    protocol_name: &'i str,
    leader: &'i str,
    member_id: &'i str,
    members: Option<Members<'i>>,
}

pub struct Members<'i> {

    member_id: &'i str,
    group_instance_id: Option<&'i str>,
    metadata: &'i [u8],
}
use super::*;
fn heartbeat_request<'i, I>() -> impl Parser<I, Output = HeartbeatRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        string(),
        be_i32(),
        string(),
        nullable_string(),
    ).map(|(group_id,generation_id,member_id,group_instance_id)| {HeartbeatRequest{
    group_id,generation_id,member_id,group_instance_id
    }})
}

pub struct HeartbeatRequest<'i> {

    group_id: &'i str,
    generation_id: i32,
    member_id: &'i str,
    group_instance_id: Option<&'i str>,
}


use super::*;
fn heartbeat_response<'i, I>() -> impl Parser<I, Output = HeartbeatResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i16(),
    ).map(|(throttle_time_ms,error_code)| {HeartbeatResponse{
    throttle_time_ms,error_code
    }})
}

pub struct HeartbeatResponse<'i> {

    throttle_time_ms: i32,
    error_code: i16,
}


use super::*;
fn leave_group_request<'i, I>() -> impl Parser<I, Output = LeaveGroupRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        string(),
        string(),
    ).map(|(group_id,member_id)| {LeaveGroupRequest{
    group_id,member_id
    }})
}

pub struct LeaveGroupRequest<'i> {

    group_id: &'i str,
    member_id: &'i str,
}


use super::*;
fn leave_group_response<'i, I>() -> impl Parser<I, Output = LeaveGroupResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i16(),
    ).map(|(throttle_time_ms,error_code)| {LeaveGroupResponse{
    throttle_time_ms,error_code
    }})
}

pub struct LeaveGroupResponse<'i> {

    throttle_time_ms: i32,
    error_code: i16,
}


use super::*;
fn sync_group_request<'i, I>() -> impl Parser<I, Output = SyncGroupRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        string(),
        be_i32(),
        string(),
        nullable_string(),
        optional(
        (
            string(),
            bytes(),
        ).map(|(member_id,assignment)| {Assignments{
        member_id,assignment
        }})),
    ).map(|(group_id,generation_id,member_id,group_instance_id,assignments)| {SyncGroupRequest{
    group_id,generation_id,member_id,group_instance_id,assignments
    }})
}

pub struct SyncGroupRequest<'i> {

    group_id: &'i str,
    generation_id: i32,
    member_id: &'i str,
    group_instance_id: Option<&'i str>,
    assignments: Option<Assignments<'i>>,
}

pub struct Assignments<'i> {

    member_id: &'i str,
    assignment: &'i [u8],
}
use super::*;
fn sync_group_response<'i, I>() -> impl Parser<I, Output = SyncGroupResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i16(),
        bytes(),
    ).map(|(throttle_time_ms,error_code,assignment)| {SyncGroupResponse{
    throttle_time_ms,error_code,assignment
    }})
}

pub struct SyncGroupResponse<'i> {

    throttle_time_ms: i32,
    error_code: i16,
    assignment: &'i [u8],
}


use super::*;
fn describe_groups_request<'i, I>() -> impl Parser<I, Output = DescribeGroupsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional(
        (
            string(),
        ).map(|(STRING)| {Groups{
        STRING
        }})),
        any().map(|b| b != 0),
    ).map(|(groups,include_authorized_operations)| {DescribeGroupsRequest{
    groups,include_authorized_operations
    }})
}

pub struct DescribeGroupsRequest<'i> {

    groups: Option<Groups<'i>>,
    include_authorized_operations: bool,
}

pub struct Groups<'i> {

    STRING: &'i str,
}
use super::*;
fn describe_groups_response<'i, I>() -> impl Parser<I, Output = DescribeGroupsResponse<'i>>
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
            string(),
            string(),
            string(),
            optional(
            (
                string(),
                string(),
                string(),
                bytes(),
                bytes(),
            ).map(|(member_id,client_id,client_host,member_metadata,member_assignment)| {Members{
            member_id,client_id,client_host,member_metadata,member_assignment
            }})),
            be_i32(),
        ).map(|(error_code,group_id,group_state,protocol_type,protocol_data,members,authorized_operations)| {Groups{
        error_code,group_id,group_state,protocol_type,protocol_data,members,authorized_operations
        }})),
    ).map(|(throttle_time_ms,groups)| {DescribeGroupsResponse{
    throttle_time_ms,groups
    }})
}

pub struct DescribeGroupsResponse<'i> {

    throttle_time_ms: i32,
    groups: Option<Groups<'i>>,
}

pub struct Members<'i> {

    member_id: &'i str,
    client_id: &'i str,
    client_host: &'i str,
    member_metadata: &'i [u8],
    member_assignment: &'i [u8],
}

pub struct Groups<'i> {

    error_code: i16,
    group_id: &'i str,
    group_state: &'i str,
    protocol_type: &'i str,
    protocol_data: &'i str,
    members: Option<Members<'i>>,
    authorized_operations: i32,
}
use super::*;
fn list_groups_request<'i, I>() -> impl Parser<I, Output = ListGroupsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        
    ).map(|()| {ListGroupsRequest{
    
    }})
}

pub struct ListGroupsRequest<'i> {

    
}


use super::*;
fn list_groups_response<'i, I>() -> impl Parser<I, Output = ListGroupsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i16(),
        optional(
        (
            string(),
            string(),
        ).map(|(group_id,protocol_type)| {Groups{
        group_id,protocol_type
        }})),
    ).map(|(throttle_time_ms,error_code,groups)| {ListGroupsResponse{
    throttle_time_ms,error_code,groups
    }})
}

pub struct ListGroupsResponse<'i> {

    throttle_time_ms: i32,
    error_code: i16,
    groups: Option<Groups<'i>>,
}

pub struct Groups<'i> {

    group_id: &'i str,
    protocol_type: &'i str,
}
use super::*;
fn sasl_handshake_request<'i, I>() -> impl Parser<I, Output = SaslHandshakeRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        string(),
    ).map(|(mechanism)| {SaslHandshakeRequest{
    mechanism
    }})
}

pub struct SaslHandshakeRequest<'i> {

    mechanism: &'i str,
}


use super::*;
fn sasl_handshake_response<'i, I>() -> impl Parser<I, Output = SaslHandshakeResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16(),
        optional(
        (
            string(),
        ).map(|(STRING)| {Mechanisms{
        STRING
        }})),
    ).map(|(error_code,mechanisms)| {SaslHandshakeResponse{
    error_code,mechanisms
    }})
}

pub struct SaslHandshakeResponse<'i> {

    error_code: i16,
    mechanisms: Option<Mechanisms<'i>>,
}

pub struct Mechanisms<'i> {

    STRING: &'i str,
}
use super::*;
fn api_versions_request<'i, I>() -> impl Parser<I, Output = ApiVersionsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        
    ).map(|()| {ApiVersionsRequest{
    
    }})
}

pub struct ApiVersionsRequest<'i> {

    
}


use super::*;
fn api_versions_response<'i, I>() -> impl Parser<I, Output = ApiVersionsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16(),
        optional(
        (
            be_i16(),
            be_i16(),
            be_i16(),
        ).map(|(api_key,min_version,max_version)| {ApiVersions{
        api_key,min_version,max_version
        }})),
        be_i32(),
    ).map(|(error_code,api_versions,throttle_time_ms)| {ApiVersionsResponse{
    error_code,api_versions,throttle_time_ms
    }})
}

pub struct ApiVersionsResponse<'i> {

    error_code: i16,
    api_versions: Option<ApiVersions<'i>>,
    throttle_time_ms: i32,
}

pub struct ApiVersions<'i> {

    api_key: i16,
    min_version: i16,
    max_version: i16,
}
use super::*;
fn create_topics_request<'i, I>() -> impl Parser<I, Output = CreateTopicsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional(
        (
            string(),
            be_i32(),
            be_i16(),
            optional(
            (
                be_i32(),
                optional(
                (
                    be_i32(),
                ).map(|(INT32)| {BrokerIds{
                INT32
                }})),
            ).map(|(partition_index,broker_ids)| {Assignments{
            partition_index,broker_ids
            }})),
            optional(
            (
                string(),
                nullable_string(),
            ).map(|(name,value)| {Configs{
            name,value
            }})),
        ).map(|(name,num_partitions,replication_factor,assignments,configs)| {Topics{
        name,num_partitions,replication_factor,assignments,configs
        }})),
        be_i32(),
        any().map(|b| b != 0),
    ).map(|(topics,timeout_ms,validate_only)| {CreateTopicsRequest{
    topics,timeout_ms,validate_only
    }})
}

pub struct CreateTopicsRequest<'i> {

    topics: Option<Topics<'i>>,
    timeout_ms: i32,
    validate_only: bool,
}

pub struct BrokerIds<'i> {

    INT32: i32,
}

pub struct Assignments<'i> {

    partition_index: i32,
    broker_ids: Option<BrokerIds<'i>>,
}

pub struct Configs<'i> {

    name: &'i str,
    value: Option<&'i str>,
}

pub struct Topics<'i> {

    name: &'i str,
    num_partitions: i32,
    replication_factor: i16,
    assignments: Option<Assignments<'i>>,
    configs: Option<Configs<'i>>,
}
use super::*;
fn create_topics_response<'i, I>() -> impl Parser<I, Output = CreateTopicsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
        (
            string(),
            be_i16(),
            nullable_string(),
        ).map(|(name,error_code,error_message)| {Topics{
        name,error_code,error_message
        }})),
    ).map(|(throttle_time_ms,topics)| {CreateTopicsResponse{
    throttle_time_ms,topics
    }})
}

pub struct CreateTopicsResponse<'i> {

    throttle_time_ms: i32,
    topics: Option<Topics<'i>>,
}

pub struct Topics<'i> {

    name: &'i str,
    error_code: i16,
    error_message: Option<&'i str>,
}
use super::*;
fn delete_topics_request<'i, I>() -> impl Parser<I, Output = DeleteTopicsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional(
        (
            string(),
        ).map(|(STRING)| {TopicNames{
        STRING
        }})),
        be_i32(),
    ).map(|(topic_names,timeout_ms)| {DeleteTopicsRequest{
    topic_names,timeout_ms
    }})
}

pub struct DeleteTopicsRequest<'i> {

    topic_names: Option<TopicNames<'i>>,
    timeout_ms: i32,
}

pub struct TopicNames<'i> {

    STRING: &'i str,
}
use super::*;
fn delete_topics_response<'i, I>() -> impl Parser<I, Output = DeleteTopicsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
        (
            string(),
            be_i16(),
        ).map(|(name,error_code)| {Responses{
        name,error_code
        }})),
    ).map(|(throttle_time_ms,responses)| {DeleteTopicsResponse{
    throttle_time_ms,responses
    }})
}

pub struct DeleteTopicsResponse<'i> {

    throttle_time_ms: i32,
    responses: Option<Responses<'i>>,
}

pub struct Responses<'i> {

    name: &'i str,
    error_code: i16,
}
use super::*;
fn delete_records_request<'i, I>() -> impl Parser<I, Output = DeleteRecordsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
                be_i64(),
            ).map(|(partition,offset)| {Partitions{
            partition,offset
            }})),
        ).map(|(topic,partitions)| {Topics{
        topic,partitions
        }})),
        be_i32(),
    ).map(|(topics,timeout)| {DeleteRecordsRequest{
    topics,timeout
    }})
}

pub struct DeleteRecordsRequest<'i> {

    topics: Option<Topics<'i>>,
    timeout: i32,
}

pub struct Partitions<'i> {

    partition: i32,
    offset: i64,
}

pub struct Topics<'i> {

    topic: &'i str,
    partitions: Option<Partitions<'i>>,
}
use super::*;
fn delete_records_response<'i, I>() -> impl Parser<I, Output = DeleteRecordsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
                be_i64(),
                be_i16(),
            ).map(|(partition,low_watermark,error_code)| {Partitions{
            partition,low_watermark,error_code
            }})),
        ).map(|(topic,partitions)| {Topics{
        topic,partitions
        }})),
    ).map(|(throttle_time_ms,topics)| {DeleteRecordsResponse{
    throttle_time_ms,topics
    }})
}

pub struct DeleteRecordsResponse<'i> {

    throttle_time_ms: i32,
    topics: Option<Topics<'i>>,
}

pub struct Partitions<'i> {

    partition: i32,
    low_watermark: i64,
    error_code: i16,
}

pub struct Topics<'i> {

    topic: &'i str,
    partitions: Option<Partitions<'i>>,
}
use super::*;
fn init_producer_id_request<'i, I>() -> impl Parser<I, Output = InitProducerIdRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        nullable_string(),
        be_i32(),
    ).map(|(transactional_id,transaction_timeout_ms)| {InitProducerIdRequest{
    transactional_id,transaction_timeout_ms
    }})
}

pub struct InitProducerIdRequest<'i> {

    transactional_id: Option<&'i str>,
    transaction_timeout_ms: i32,
}


use super::*;
fn init_producer_id_response<'i, I>() -> impl Parser<I, Output = InitProducerIdResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i16(),
        be_i64(),
        be_i16(),
    ).map(|(throttle_time_ms,error_code,producer_id,producer_epoch)| {InitProducerIdResponse{
    throttle_time_ms,error_code,producer_id,producer_epoch
    }})
}

pub struct InitProducerIdResponse<'i> {

    throttle_time_ms: i32,
    error_code: i16,
    producer_id: i64,
    producer_epoch: i16,
}


use super::*;
fn offset_for_leader_epoch_request<'i, I>() -> impl Parser<I, Output = OffsetForLeaderEpochRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
                be_i32(),
                be_i32(),
            ).map(|(partition,current_leader_epoch,leader_epoch)| {Partitions{
            partition,current_leader_epoch,leader_epoch
            }})),
        ).map(|(topic,partitions)| {Topics{
        topic,partitions
        }})),
    ).map(|(replica_id,topics)| {OffsetForLeaderEpochRequest{
    replica_id,topics
    }})
}

pub struct OffsetForLeaderEpochRequest<'i> {

    replica_id: i32,
    topics: Option<Topics<'i>>,
}

pub struct Partitions<'i> {

    partition: i32,
    current_leader_epoch: i32,
    leader_epoch: i32,
}

pub struct Topics<'i> {

    topic: &'i str,
    partitions: Option<Partitions<'i>>,
}
use super::*;
fn offset_for_leader_epoch_response<'i, I>() -> impl Parser<I, Output = OffsetForLeaderEpochResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
        (
            string(),
            optional(
            (
                be_i16(),
                be_i32(),
                be_i32(),
                be_i64(),
            ).map(|(error_code,partition,leader_epoch,end_offset)| {Partitions{
            error_code,partition,leader_epoch,end_offset
            }})),
        ).map(|(topic,partitions)| {Topics{
        topic,partitions
        }})),
    ).map(|(throttle_time_ms,topics)| {OffsetForLeaderEpochResponse{
    throttle_time_ms,topics
    }})
}

pub struct OffsetForLeaderEpochResponse<'i> {

    throttle_time_ms: i32,
    topics: Option<Topics<'i>>,
}

pub struct Partitions<'i> {

    error_code: i16,
    partition: i32,
    leader_epoch: i32,
    end_offset: i64,
}

pub struct Topics<'i> {

    topic: &'i str,
    partitions: Option<Partitions<'i>>,
}
use super::*;
fn add_partitions_to_txn_request<'i, I>() -> impl Parser<I, Output = AddPartitionsToTxnRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        string(),
        be_i64(),
        be_i16(),
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
            ).map(|(INT32)| {Partitions{
            INT32
            }})),
        ).map(|(topic,partitions)| {Topics{
        topic,partitions
        }})),
    ).map(|(transactional_id,producer_id,producer_epoch,topics)| {AddPartitionsToTxnRequest{
    transactional_id,producer_id,producer_epoch,topics
    }})
}

pub struct AddPartitionsToTxnRequest<'i> {

    transactional_id: &'i str,
    producer_id: i64,
    producer_epoch: i16,
    topics: Option<Topics<'i>>,
}

pub struct Partitions<'i> {

    INT32: i32,
}

pub struct Topics<'i> {

    topic: &'i str,
    partitions: Option<Partitions<'i>>,
}
use super::*;
fn add_partitions_to_txn_response<'i, I>() -> impl Parser<I, Output = AddPartitionsToTxnResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
                be_i16(),
            ).map(|(partition,error_code)| {PartitionErrors{
            partition,error_code
            }})),
        ).map(|(topic,partition_errors)| {Errors{
        topic,partition_errors
        }})),
    ).map(|(throttle_time_ms,errors)| {AddPartitionsToTxnResponse{
    throttle_time_ms,errors
    }})
}

pub struct AddPartitionsToTxnResponse<'i> {

    throttle_time_ms: i32,
    errors: Option<Errors<'i>>,
}

pub struct PartitionErrors<'i> {

    partition: i32,
    error_code: i16,
}

pub struct Errors<'i> {

    topic: &'i str,
    partition_errors: Option<PartitionErrors<'i>>,
}
use super::*;
fn add_offsets_to_txn_request<'i, I>() -> impl Parser<I, Output = AddOffsetsToTxnRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        string(),
        be_i64(),
        be_i16(),
        string(),
    ).map(|(transactional_id,producer_id,producer_epoch,group_id)| {AddOffsetsToTxnRequest{
    transactional_id,producer_id,producer_epoch,group_id
    }})
}

pub struct AddOffsetsToTxnRequest<'i> {

    transactional_id: &'i str,
    producer_id: i64,
    producer_epoch: i16,
    group_id: &'i str,
}


use super::*;
fn add_offsets_to_txn_response<'i, I>() -> impl Parser<I, Output = AddOffsetsToTxnResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i16(),
    ).map(|(throttle_time_ms,error_code)| {AddOffsetsToTxnResponse{
    throttle_time_ms,error_code
    }})
}

pub struct AddOffsetsToTxnResponse<'i> {

    throttle_time_ms: i32,
    error_code: i16,
}


use super::*;
fn end_txn_request<'i, I>() -> impl Parser<I, Output = EndTxnRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        string(),
        be_i64(),
        be_i16(),
        any().map(|b| b != 0),
    ).map(|(transactional_id,producer_id,producer_epoch,transaction_result)| {EndTxnRequest{
    transactional_id,producer_id,producer_epoch,transaction_result
    }})
}

pub struct EndTxnRequest<'i> {

    transactional_id: &'i str,
    producer_id: i64,
    producer_epoch: i16,
    transaction_result: bool,
}


use super::*;
fn end_txn_response<'i, I>() -> impl Parser<I, Output = EndTxnResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i16(),
    ).map(|(throttle_time_ms,error_code)| {EndTxnResponse{
    throttle_time_ms,error_code
    }})
}

pub struct EndTxnResponse<'i> {

    throttle_time_ms: i32,
    error_code: i16,
}


use super::*;
fn write_txn_markers_request<'i, I>() -> impl Parser<I, Output = WriteTxnMarkersRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional(
        (
            be_i64(),
            be_i16(),
            any().map(|b| b != 0),
            optional(
            (
                string(),
                optional(
                (
                    be_i32(),
                ).map(|(INT32)| {Partitions{
                INT32
                }})),
            ).map(|(topic,partitions)| {Topics{
            topic,partitions
            }})),
            be_i32(),
        ).map(|(producer_id,producer_epoch,transaction_result,topics,coordinator_epoch)| {TransactionMarkers{
        producer_id,producer_epoch,transaction_result,topics,coordinator_epoch
        }})),
    ).map(|(transaction_markers)| {WriteTxnMarkersRequest{
    transaction_markers
    }})
}

pub struct WriteTxnMarkersRequest<'i> {

    transaction_markers: Option<TransactionMarkers<'i>>,
}

pub struct Partitions<'i> {

    INT32: i32,
}

pub struct Topics<'i> {

    topic: &'i str,
    partitions: Option<Partitions<'i>>,
}

pub struct TransactionMarkers<'i> {

    producer_id: i64,
    producer_epoch: i16,
    transaction_result: bool,
    topics: Option<Topics<'i>>,
    coordinator_epoch: i32,
}
use super::*;
fn write_txn_markers_response<'i, I>() -> impl Parser<I, Output = WriteTxnMarkersResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional(
        (
            be_i64(),
            optional(
            (
                string(),
                optional(
                (
                    be_i32(),
                    be_i16(),
                ).map(|(partition,error_code)| {Partitions{
                partition,error_code
                }})),
            ).map(|(topic,partitions)| {Topics{
            topic,partitions
            }})),
        ).map(|(producer_id,topics)| {TransactionMarkers{
        producer_id,topics
        }})),
    ).map(|(transaction_markers)| {WriteTxnMarkersResponse{
    transaction_markers
    }})
}

pub struct WriteTxnMarkersResponse<'i> {

    transaction_markers: Option<TransactionMarkers<'i>>,
}

pub struct Partitions<'i> {

    partition: i32,
    error_code: i16,
}

pub struct Topics<'i> {

    topic: &'i str,
    partitions: Option<Partitions<'i>>,
}

pub struct TransactionMarkers<'i> {

    producer_id: i64,
    topics: Option<Topics<'i>>,
}
use super::*;
fn txn_offset_commit_request<'i, I>() -> impl Parser<I, Output = TxnOffsetCommitRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        string(),
        string(),
        be_i64(),
        be_i16(),
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
                be_i64(),
                be_i32(),
                nullable_string(),
            ).map(|(partition,offset,leader_epoch,metadata)| {Partitions{
            partition,offset,leader_epoch,metadata
            }})),
        ).map(|(topic,partitions)| {Topics{
        topic,partitions
        }})),
    ).map(|(transactional_id,group_id,producer_id,producer_epoch,topics)| {TxnOffsetCommitRequest{
    transactional_id,group_id,producer_id,producer_epoch,topics
    }})
}

pub struct TxnOffsetCommitRequest<'i> {

    transactional_id: &'i str,
    group_id: &'i str,
    producer_id: i64,
    producer_epoch: i16,
    topics: Option<Topics<'i>>,
}

pub struct Partitions<'i> {

    partition: i32,
    offset: i64,
    leader_epoch: i32,
    metadata: Option<&'i str>,
}

pub struct Topics<'i> {

    topic: &'i str,
    partitions: Option<Partitions<'i>>,
}
use super::*;
fn txn_offset_commit_response<'i, I>() -> impl Parser<I, Output = TxnOffsetCommitResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
                be_i16(),
            ).map(|(partition,error_code)| {Partitions{
            partition,error_code
            }})),
        ).map(|(topic,partitions)| {Topics{
        topic,partitions
        }})),
    ).map(|(throttle_time_ms,topics)| {TxnOffsetCommitResponse{
    throttle_time_ms,topics
    }})
}

pub struct TxnOffsetCommitResponse<'i> {

    throttle_time_ms: i32,
    topics: Option<Topics<'i>>,
}

pub struct Partitions<'i> {

    partition: i32,
    error_code: i16,
}

pub struct Topics<'i> {

    topic: &'i str,
    partitions: Option<Partitions<'i>>,
}
use super::*;
fn describe_acls_request<'i, I>() -> impl Parser<I, Output = DescribeAclsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i8(),
        nullable_string(),
        be_i8(),
        nullable_string(),
        nullable_string(),
        be_i8(),
        be_i8(),
    ).map(|(resource_type,resource_name,resource_pattern_type_filter,principal,host,operation,permission_type)| {DescribeAclsRequest{
    resource_type,resource_name,resource_pattern_type_filter,principal,host,operation,permission_type
    }})
}

pub struct DescribeAclsRequest<'i> {

    resource_type: i8,
    resource_name: Option<&'i str>,
    resource_pattern_type_filter: i8,
    principal: Option<&'i str>,
    host: Option<&'i str>,
    operation: i8,
    permission_type: i8,
}


use super::*;
fn describe_acls_response<'i, I>() -> impl Parser<I, Output = DescribeAclsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        be_i16(),
        nullable_string(),
        optional(
        (
            be_i8(),
            string(),
            be_i8(),
            optional(
            (
                string(),
                string(),
                be_i8(),
                be_i8(),
            ).map(|(principal,host,operation,permission_type)| {Acls{
            principal,host,operation,permission_type
            }})),
        ).map(|(resource_type,resource_name,resource_pattern_type,acls)| {Resources{
        resource_type,resource_name,resource_pattern_type,acls
        }})),
    ).map(|(throttle_time_ms,error_code,error_message,resources)| {DescribeAclsResponse{
    throttle_time_ms,error_code,error_message,resources
    }})
}

pub struct DescribeAclsResponse<'i> {

    throttle_time_ms: i32,
    error_code: i16,
    error_message: Option<&'i str>,
    resources: Option<Resources<'i>>,
}

pub struct Acls<'i> {

    principal: &'i str,
    host: &'i str,
    operation: i8,
    permission_type: i8,
}

pub struct Resources<'i> {

    resource_type: i8,
    resource_name: &'i str,
    resource_pattern_type: i8,
    acls: Option<Acls<'i>>,
}
use super::*;
fn create_acls_request<'i, I>() -> impl Parser<I, Output = CreateAclsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional(
        (
            be_i8(),
            string(),
            be_i8(),
            string(),
            string(),
            be_i8(),
            be_i8(),
        ).map(|(resource_type,resource_name,resource_pattern_type,principal,host,operation,permission_type)| {Creations{
        resource_type,resource_name,resource_pattern_type,principal,host,operation,permission_type
        }})),
    ).map(|(creations)| {CreateAclsRequest{
    creations
    }})
}

pub struct CreateAclsRequest<'i> {

    creations: Option<Creations<'i>>,
}

pub struct Creations<'i> {

    resource_type: i8,
    resource_name: &'i str,
    resource_pattern_type: i8,
    principal: &'i str,
    host: &'i str,
    operation: i8,
    permission_type: i8,
}
use super::*;
fn create_acls_response<'i, I>() -> impl Parser<I, Output = CreateAclsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
        (
            be_i16(),
            nullable_string(),
        ).map(|(error_code,error_message)| {CreationResponses{
        error_code,error_message
        }})),
    ).map(|(throttle_time_ms,creation_responses)| {CreateAclsResponse{
    throttle_time_ms,creation_responses
    }})
}

pub struct CreateAclsResponse<'i> {

    throttle_time_ms: i32,
    creation_responses: Option<CreationResponses<'i>>,
}

pub struct CreationResponses<'i> {

    error_code: i16,
    error_message: Option<&'i str>,
}
use super::*;
fn delete_acls_request<'i, I>() -> impl Parser<I, Output = DeleteAclsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional(
        (
            be_i8(),
            nullable_string(),
            be_i8(),
            nullable_string(),
            nullable_string(),
            be_i8(),
            be_i8(),
        ).map(|(resource_type,resource_name,resource_pattern_type_filter,principal,host,operation,permission_type)| {Filters{
        resource_type,resource_name,resource_pattern_type_filter,principal,host,operation,permission_type
        }})),
    ).map(|(filters)| {DeleteAclsRequest{
    filters
    }})
}

pub struct DeleteAclsRequest<'i> {

    filters: Option<Filters<'i>>,
}

pub struct Filters<'i> {

    resource_type: i8,
    resource_name: Option<&'i str>,
    resource_pattern_type_filter: i8,
    principal: Option<&'i str>,
    host: Option<&'i str>,
    operation: i8,
    permission_type: i8,
}
use super::*;
fn delete_acls_response<'i, I>() -> impl Parser<I, Output = DeleteAclsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
        (
            be_i16(),
            nullable_string(),
            optional(
            (
                be_i16(),
                nullable_string(),
                be_i8(),
                string(),
                be_i8(),
                string(),
                string(),
                be_i8(),
                be_i8(),
            ).map(|(error_code,error_message,resource_type,resource_name,resource_pattern_type,principal,host,operation,permission_type)| {MatchingAcls{
            error_code,error_message,resource_type,resource_name,resource_pattern_type,principal,host,operation,permission_type
            }})),
        ).map(|(error_code,error_message,matching_acls)| {FilterResponses{
        error_code,error_message,matching_acls
        }})),
    ).map(|(throttle_time_ms,filter_responses)| {DeleteAclsResponse{
    throttle_time_ms,filter_responses
    }})
}

pub struct DeleteAclsResponse<'i> {

    throttle_time_ms: i32,
    filter_responses: Option<FilterResponses<'i>>,
}

pub struct MatchingAcls<'i> {

    error_code: i16,
    error_message: Option<&'i str>,
    resource_type: i8,
    resource_name: &'i str,
    resource_pattern_type: i8,
    principal: &'i str,
    host: &'i str,
    operation: i8,
    permission_type: i8,
}

pub struct FilterResponses<'i> {

    error_code: i16,
    error_message: Option<&'i str>,
    matching_acls: Option<MatchingAcls<'i>>,
}
use super::*;
fn describe_configs_request<'i, I>() -> impl Parser<I, Output = DescribeConfigsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional(
        (
            be_i8(),
            string(),
            optional(
            (
                string(),
            ).map(|(STRING)| {ConfigNames{
            STRING
            }})),
        ).map(|(resource_type,resource_name,config_names)| {Resources{
        resource_type,resource_name,config_names
        }})),
        any().map(|b| b != 0),
    ).map(|(resources,include_synonyms)| {DescribeConfigsRequest{
    resources,include_synonyms
    }})
}

pub struct DescribeConfigsRequest<'i> {

    resources: Option<Resources<'i>>,
    include_synonyms: bool,
}

pub struct ConfigNames<'i> {

    STRING: &'i str,
}

pub struct Resources<'i> {

    resource_type: i8,
    resource_name: &'i str,
    config_names: Option<ConfigNames<'i>>,
}
use super::*;
fn describe_configs_response<'i, I>() -> impl Parser<I, Output = DescribeConfigsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
        (
            be_i16(),
            nullable_string(),
            be_i8(),
            string(),
            optional(
            (
                string(),
                nullable_string(),
                any().map(|b| b != 0),
                be_i8(),
                any().map(|b| b != 0),
                optional(
                (
                    string(),
                    nullable_string(),
                    be_i8(),
                ).map(|(config_name,config_value,config_source)| {ConfigSynonyms{
                config_name,config_value,config_source
                }})),
            ).map(|(config_name,config_value,read_only,config_source,is_sensitive,config_synonyms)| {ConfigEntries{
            config_name,config_value,read_only,config_source,is_sensitive,config_synonyms
            }})),
        ).map(|(error_code,error_message,resource_type,resource_name,config_entries)| {Resources{
        error_code,error_message,resource_type,resource_name,config_entries
        }})),
    ).map(|(throttle_time_ms,resources)| {DescribeConfigsResponse{
    throttle_time_ms,resources
    }})
}

pub struct DescribeConfigsResponse<'i> {

    throttle_time_ms: i32,
    resources: Option<Resources<'i>>,
}

pub struct ConfigSynonyms<'i> {

    config_name: &'i str,
    config_value: Option<&'i str>,
    config_source: i8,
}

pub struct ConfigEntries<'i> {

    config_name: &'i str,
    config_value: Option<&'i str>,
    read_only: bool,
    config_source: i8,
    is_sensitive: bool,
    config_synonyms: Option<ConfigSynonyms<'i>>,
}

pub struct Resources<'i> {

    error_code: i16,
    error_message: Option<&'i str>,
    resource_type: i8,
    resource_name: &'i str,
    config_entries: Option<ConfigEntries<'i>>,
}
use super::*;
fn alter_configs_request<'i, I>() -> impl Parser<I, Output = AlterConfigsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional(
        (
            be_i8(),
            string(),
            optional(
            (
                string(),
                nullable_string(),
            ).map(|(config_name,config_value)| {ConfigEntries{
            config_name,config_value
            }})),
        ).map(|(resource_type,resource_name,config_entries)| {Resources{
        resource_type,resource_name,config_entries
        }})),
        any().map(|b| b != 0),
    ).map(|(resources,validate_only)| {AlterConfigsRequest{
    resources,validate_only
    }})
}

pub struct AlterConfigsRequest<'i> {

    resources: Option<Resources<'i>>,
    validate_only: bool,
}

pub struct ConfigEntries<'i> {

    config_name: &'i str,
    config_value: Option<&'i str>,
}

pub struct Resources<'i> {

    resource_type: i8,
    resource_name: &'i str,
    config_entries: Option<ConfigEntries<'i>>,
}
use super::*;
fn alter_configs_response<'i, I>() -> impl Parser<I, Output = AlterConfigsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
        (
            be_i16(),
            nullable_string(),
            be_i8(),
            string(),
        ).map(|(error_code,error_message,resource_type,resource_name)| {Resources{
        error_code,error_message,resource_type,resource_name
        }})),
    ).map(|(throttle_time_ms,resources)| {AlterConfigsResponse{
    throttle_time_ms,resources
    }})
}

pub struct AlterConfigsResponse<'i> {

    throttle_time_ms: i32,
    resources: Option<Resources<'i>>,
}

pub struct Resources<'i> {

    error_code: i16,
    error_message: Option<&'i str>,
    resource_type: i8,
    resource_name: &'i str,
}
use super::*;
fn alter_replica_log_dirs_request<'i, I>() -> impl Parser<I, Output = AlterReplicaLogDirsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional(
        (
            string(),
            optional(
            (
                string(),
                optional(
                (
                    be_i32(),
                ).map(|(INT32)| {Partitions{
                INT32
                }})),
            ).map(|(topic,partitions)| {Topics{
            topic,partitions
            }})),
        ).map(|(log_dir,topics)| {LogDirs{
        log_dir,topics
        }})),
    ).map(|(log_dirs)| {AlterReplicaLogDirsRequest{
    log_dirs
    }})
}

pub struct AlterReplicaLogDirsRequest<'i> {

    log_dirs: Option<LogDirs<'i>>,
}

pub struct Partitions<'i> {

    INT32: i32,
}

pub struct Topics<'i> {

    topic: &'i str,
    partitions: Option<Partitions<'i>>,
}

pub struct LogDirs<'i> {

    log_dir: &'i str,
    topics: Option<Topics<'i>>,
}
use super::*;
fn alter_replica_log_dirs_response<'i, I>() -> impl Parser<I, Output = AlterReplicaLogDirsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
                be_i16(),
            ).map(|(partition,error_code)| {Partitions{
            partition,error_code
            }})),
        ).map(|(topic,partitions)| {Topics{
        topic,partitions
        }})),
    ).map(|(throttle_time_ms,topics)| {AlterReplicaLogDirsResponse{
    throttle_time_ms,topics
    }})
}

pub struct AlterReplicaLogDirsResponse<'i> {

    throttle_time_ms: i32,
    topics: Option<Topics<'i>>,
}

pub struct Partitions<'i> {

    partition: i32,
    error_code: i16,
}

pub struct Topics<'i> {

    topic: &'i str,
    partitions: Option<Partitions<'i>>,
}
use super::*;
fn describe_log_dirs_request<'i, I>() -> impl Parser<I, Output = DescribeLogDirsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
            ).map(|(INT32)| {Partitions{
            INT32
            }})),
        ).map(|(topic,partitions)| {Topics{
        topic,partitions
        }})),
    ).map(|(topics)| {DescribeLogDirsRequest{
    topics
    }})
}

pub struct DescribeLogDirsRequest<'i> {

    topics: Option<Topics<'i>>,
}

pub struct Partitions<'i> {

    INT32: i32,
}

pub struct Topics<'i> {

    topic: &'i str,
    partitions: Option<Partitions<'i>>,
}
use super::*;
fn describe_log_dirs_response<'i, I>() -> impl Parser<I, Output = DescribeLogDirsResponse<'i>>
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
                optional(
                (
                    be_i32(),
                    be_i64(),
                    be_i64(),
                    any().map(|b| b != 0),
                ).map(|(partition,size,offset_lag,is_future)| {Partitions{
                partition,size,offset_lag,is_future
                }})),
            ).map(|(topic,partitions)| {Topics{
            topic,partitions
            }})),
        ).map(|(error_code,log_dir,topics)| {LogDirs{
        error_code,log_dir,topics
        }})),
    ).map(|(throttle_time_ms,log_dirs)| {DescribeLogDirsResponse{
    throttle_time_ms,log_dirs
    }})
}

pub struct DescribeLogDirsResponse<'i> {

    throttle_time_ms: i32,
    log_dirs: Option<LogDirs<'i>>,
}

pub struct Partitions<'i> {

    partition: i32,
    size: i64,
    offset_lag: i64,
    is_future: bool,
}

pub struct Topics<'i> {

    topic: &'i str,
    partitions: Option<Partitions<'i>>,
}

pub struct LogDirs<'i> {

    error_code: i16,
    log_dir: &'i str,
    topics: Option<Topics<'i>>,
}
use super::*;
fn sasl_authenticate_request<'i, I>() -> impl Parser<I, Output = SaslAuthenticateRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        bytes(),
    ).map(|(auth_bytes)| {SaslAuthenticateRequest{
    auth_bytes
    }})
}

pub struct SaslAuthenticateRequest<'i> {

    auth_bytes: &'i [u8],
}


use super::*;
fn sasl_authenticate_response<'i, I>() -> impl Parser<I, Output = SaslAuthenticateResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16(),
        nullable_string(),
        bytes(),
        be_i64(),
    ).map(|(error_code,error_message,auth_bytes,session_lifetime_ms)| {SaslAuthenticateResponse{
    error_code,error_message,auth_bytes,session_lifetime_ms
    }})
}

pub struct SaslAuthenticateResponse<'i> {

    error_code: i16,
    error_message: Option<&'i str>,
    auth_bytes: &'i [u8],
    session_lifetime_ms: i64,
}


use super::*;
fn create_partitions_request<'i, I>() -> impl Parser<I, Output = CreatePartitionsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional(
        (
            string(),
                count(),
        ).map(|(topic,new_partitions)| {TopicPartitions{
        topic,new_partitions
        }})),
            timeout(),
            validate_only(),
    ).map(|(topic_partitions,timeout,validate_only)| {CreatePartitionsRequest{
    topic_partitions,timeout,validate_only
    }})
}

pub struct CreatePartitionsRequest<'i> {

    topic_partitions: Option<TopicPartitions<'i>>,
    timeout: timeout,
    validate_only: validate_only,
}

pub struct TopicPartitions<'i> {

    topic: &'i str,
    new_partitions: count,
}
use super::*;
fn create_partitions_response<'i, I>() -> impl Parser<I, Output = CreatePartitionsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
        (
            string(),
            be_i16(),
            nullable_string(),
        ).map(|(topic,error_code,error_message)| {TopicErrors{
        topic,error_code,error_message
        }})),
    ).map(|(throttle_time_ms,topic_errors)| {CreatePartitionsResponse{
    throttle_time_ms,topic_errors
    }})
}

pub struct CreatePartitionsResponse<'i> {

    throttle_time_ms: i32,
    topic_errors: Option<TopicErrors<'i>>,
}

pub struct TopicErrors<'i> {

    topic: &'i str,
    error_code: i16,
    error_message: Option<&'i str>,
}
use super::*;
fn create_delegation_token_request<'i, I>() -> impl Parser<I, Output = CreateDelegationTokenRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional(
        (
            string(),
            string(),
        ).map(|(principal_type,name)| {Renewers{
        principal_type,name
        }})),
        be_i64(),
    ).map(|(renewers,max_life_time)| {CreateDelegationTokenRequest{
    renewers,max_life_time
    }})
}

pub struct CreateDelegationTokenRequest<'i> {

    renewers: Option<Renewers<'i>>,
    max_life_time: i64,
}

pub struct Renewers<'i> {

    principal_type: &'i str,
    name: &'i str,
}
use super::*;
fn create_delegation_token_response<'i, I>() -> impl Parser<I, Output = CreateDelegationTokenResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16(),
            principal_type(),
        be_i64(),
        be_i64(),
        be_i64(),
        string(),
        bytes(),
        be_i32(),
    ).map(|(error_code,owner,issue_timestamp,expiry_timestamp,max_timestamp,token_id,hmac,throttle_time_ms)| {CreateDelegationTokenResponse{
    error_code,owner,issue_timestamp,expiry_timestamp,max_timestamp,token_id,hmac,throttle_time_ms
    }})
}

pub struct CreateDelegationTokenResponse<'i> {

    error_code: i16,
    owner: principal_type,
    issue_timestamp: i64,
    expiry_timestamp: i64,
    max_timestamp: i64,
    token_id: &'i str,
    hmac: &'i [u8],
    throttle_time_ms: i32,
}


use super::*;
fn renew_delegation_token_request<'i, I>() -> impl Parser<I, Output = RenewDelegationTokenRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        bytes(),
        be_i64(),
    ).map(|(hmac,renew_time_period)| {RenewDelegationTokenRequest{
    hmac,renew_time_period
    }})
}

pub struct RenewDelegationTokenRequest<'i> {

    hmac: &'i [u8],
    renew_time_period: i64,
}


use super::*;
fn renew_delegation_token_response<'i, I>() -> impl Parser<I, Output = RenewDelegationTokenResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16(),
        be_i64(),
        be_i32(),
    ).map(|(error_code,expiry_timestamp,throttle_time_ms)| {RenewDelegationTokenResponse{
    error_code,expiry_timestamp,throttle_time_ms
    }})
}

pub struct RenewDelegationTokenResponse<'i> {

    error_code: i16,
    expiry_timestamp: i64,
    throttle_time_ms: i32,
}


use super::*;
fn expire_delegation_token_request<'i, I>() -> impl Parser<I, Output = ExpireDelegationTokenRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        bytes(),
        be_i64(),
    ).map(|(hmac,expiry_time_period)| {ExpireDelegationTokenRequest{
    hmac,expiry_time_period
    }})
}

pub struct ExpireDelegationTokenRequest<'i> {

    hmac: &'i [u8],
    expiry_time_period: i64,
}


use super::*;
fn expire_delegation_token_response<'i, I>() -> impl Parser<I, Output = ExpireDelegationTokenResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16(),
        be_i64(),
        be_i32(),
    ).map(|(error_code,expiry_timestamp,throttle_time_ms)| {ExpireDelegationTokenResponse{
    error_code,expiry_timestamp,throttle_time_ms
    }})
}

pub struct ExpireDelegationTokenResponse<'i> {

    error_code: i16,
    expiry_timestamp: i64,
    throttle_time_ms: i32,
}


use super::*;
fn describe_delegation_token_request<'i, I>() -> impl Parser<I, Output = DescribeDelegationTokenRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional(
        (
            string(),
            string(),
        ).map(|(principal_type,name)| {Owners{
        principal_type,name
        }})),
    ).map(|(owners)| {DescribeDelegationTokenRequest{
    owners
    }})
}

pub struct DescribeDelegationTokenRequest<'i> {

    owners: Option<Owners<'i>>,
}

pub struct Owners<'i> {

    principal_type: &'i str,
    name: &'i str,
}
use super::*;
fn describe_delegation_token_response<'i, I>() -> impl Parser<I, Output = DescribeDelegationTokenResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i16(),
        optional(
        (
                principal_type(),
            be_i64(),
            be_i64(),
            be_i64(),
            string(),
            bytes(),
            optional(
            (
                string(),
                string(),
            ).map(|(principal_type,name)| {Renewers{
            principal_type,name
            }})),
        ).map(|(owner,issue_timestamp,expiry_timestamp,max_timestamp,token_id,hmac,renewers)| {TokenDetails{
        owner,issue_timestamp,expiry_timestamp,max_timestamp,token_id,hmac,renewers
        }})),
        be_i32(),
    ).map(|(error_code,token_details,throttle_time_ms)| {DescribeDelegationTokenResponse{
    error_code,token_details,throttle_time_ms
    }})
}

pub struct DescribeDelegationTokenResponse<'i> {

    error_code: i16,
    token_details: Option<TokenDetails<'i>>,
    throttle_time_ms: i32,
}

pub struct Renewers<'i> {

    principal_type: &'i str,
    name: &'i str,
}

pub struct TokenDetails<'i> {

    owner: principal_type,
    issue_timestamp: i64,
    expiry_timestamp: i64,
    max_timestamp: i64,
    token_id: &'i str,
    hmac: &'i [u8],
    renewers: Option<Renewers<'i>>,
}
use super::*;
fn delete_groups_request<'i, I>() -> impl Parser<I, Output = DeleteGroupsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional(
        (
            string(),
        ).map(|(STRING)| {Groups{
        STRING
        }})),
    ).map(|(groups)| {DeleteGroupsRequest{
    groups
    }})
}

pub struct DeleteGroupsRequest<'i> {

    groups: Option<Groups<'i>>,
}

pub struct Groups<'i> {

    STRING: &'i str,
}
use super::*;
fn delete_groups_response<'i, I>() -> impl Parser<I, Output = DeleteGroupsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
        (
            string(),
            be_i16(),
        ).map(|(group_id,error_code)| {GroupErrorCodes{
        group_id,error_code
        }})),
    ).map(|(throttle_time_ms,group_error_codes)| {DeleteGroupsResponse{
    throttle_time_ms,group_error_codes
    }})
}

pub struct DeleteGroupsResponse<'i> {

    throttle_time_ms: i32,
    group_error_codes: Option<GroupErrorCodes<'i>>,
}

pub struct GroupErrorCodes<'i> {

    group_id: &'i str,
    error_code: i16,
}
use super::*;
fn elect_preferred_leaders_request<'i, I>() -> impl Parser<I, Output = ElectPreferredLeadersRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
            ).map(|(INT32)| {PartitionId{
            INT32
            }})),
        ).map(|(topic,partition_id)| {TopicPartitions{
        topic,partition_id
        }})),
        be_i32(),
    ).map(|(topic_partitions,timeout_ms)| {ElectPreferredLeadersRequest{
    topic_partitions,timeout_ms
    }})
}

pub struct ElectPreferredLeadersRequest<'i> {

    topic_partitions: Option<TopicPartitions<'i>>,
    timeout_ms: i32,
}

pub struct PartitionId<'i> {

    INT32: i32,
}

pub struct TopicPartitions<'i> {

    topic: &'i str,
    partition_id: Option<PartitionId<'i>>,
}
use super::*;
fn elect_preferred_leaders_response<'i, I>() -> impl Parser<I, Output = ElectPreferredLeadersResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
        (
            string(),
            optional(
            (
                be_i32(),
                be_i16(),
                nullable_string(),
            ).map(|(partition_id,error_code,error_message)| {PartitionResult{
            partition_id,error_code,error_message
            }})),
        ).map(|(topic,partition_result)| {ReplicaElectionResults{
        topic,partition_result
        }})),
    ).map(|(throttle_time_ms,replica_election_results)| {ElectPreferredLeadersResponse{
    throttle_time_ms,replica_election_results
    }})
}

pub struct ElectPreferredLeadersResponse<'i> {

    throttle_time_ms: i32,
    replica_election_results: Option<ReplicaElectionResults<'i>>,
}

pub struct PartitionResult<'i> {

    partition_id: i32,
    error_code: i16,
    error_message: Option<&'i str>,
}

pub struct ReplicaElectionResults<'i> {

    topic: &'i str,
    partition_result: Option<PartitionResult<'i>>,
}
use super::*;
fn incremental_alter_configs_request<'i, I>() -> impl Parser<I, Output = IncrementalAlterConfigsRequest<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        optional(
        (
            be_i8(),
            string(),
            optional(
            (
                string(),
                be_i8(),
                nullable_string(),
            ).map(|(name,config_operation,value)| {Configs{
            name,config_operation,value
            }})),
        ).map(|(resource_type,resource_name,configs)| {Resources{
        resource_type,resource_name,configs
        }})),
        any().map(|b| b != 0),
    ).map(|(resources,validate_only)| {IncrementalAlterConfigsRequest{
    resources,validate_only
    }})
}

pub struct IncrementalAlterConfigsRequest<'i> {

    resources: Option<Resources<'i>>,
    validate_only: bool,
}

pub struct Configs<'i> {

    name: &'i str,
    config_operation: i8,
    value: Option<&'i str>,
}

pub struct Resources<'i> {

    resource_type: i8,
    resource_name: &'i str,
    configs: Option<Configs<'i>>,
}
use super::*;
fn incremental_alter_configs_response<'i, I>() -> impl Parser<I, Output = IncrementalAlterConfigsResponse<'i>>
where
    I: RangeStream<Token = u8, Range = &'i [u8]>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    (
        be_i32(),
        optional(
        (
            be_i16(),
            nullable_string(),
            be_i8(),
            string(),
        ).map(|(error_code,error_message,resource_type,resource_name)| {Responses{
        error_code,error_message,resource_type,resource_name
        }})),
    ).map(|(throttle_time_ms,responses)| {IncrementalAlterConfigsResponse{
    throttle_time_ms,responses
    }})
}

pub struct IncrementalAlterConfigsResponse<'i> {

    throttle_time_ms: i32,
    responses: Option<Responses<'i>>,
}

pub struct Responses<'i> {

    error_code: i16,
    error_message: Option<&'i str>,
    resource_type: i8,
    resource_name: &'i str,
}
