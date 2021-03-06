use super::*;
pub mod request_header;
pub use self::request_header::{request_header, RequestHeader};
pub mod response_header;
pub use self::response_header::{response_header, ResponseHeader};
pub mod produce_request;
pub use self::produce_request::{produce_request, ProduceRequest};
pub mod produce_response;
pub use self::produce_response::{produce_response, ProduceResponse};
pub mod fetch_request;
pub use self::fetch_request::{fetch_request, FetchRequest};
pub mod fetch_response;
pub use self::fetch_response::{fetch_response, FetchResponse};
pub mod list_offsets_request;
pub use self::list_offsets_request::{list_offsets_request, ListOffsetsRequest};
pub mod list_offsets_response;
pub use self::list_offsets_response::{list_offsets_response, ListOffsetsResponse};
pub mod metadata_request;
pub use self::metadata_request::{metadata_request, MetadataRequest};
pub mod metadata_response;
pub use self::metadata_response::{metadata_response, MetadataResponse};
pub mod leader_and_isr_request;
pub use self::leader_and_isr_request::{leader_and_isr_request, LeaderAndIsrRequest};
pub mod leader_and_isr_response;
pub use self::leader_and_isr_response::{leader_and_isr_response, LeaderAndIsrResponse};
pub mod stop_replica_request;
pub use self::stop_replica_request::{stop_replica_request, StopReplicaRequest};
pub mod stop_replica_response;
pub use self::stop_replica_response::{stop_replica_response, StopReplicaResponse};
pub mod update_metadata_request;
pub use self::update_metadata_request::{update_metadata_request, UpdateMetadataRequest};
pub mod update_metadata_response;
pub use self::update_metadata_response::{update_metadata_response, UpdateMetadataResponse};
pub mod controlled_shutdown_request;
pub use self::controlled_shutdown_request::{
    controlled_shutdown_request, ControlledShutdownRequest,
};
pub mod controlled_shutdown_response;
pub use self::controlled_shutdown_response::{
    controlled_shutdown_response, ControlledShutdownResponse,
};
pub mod offset_commit_request;
pub use self::offset_commit_request::{offset_commit_request, OffsetCommitRequest};
pub mod offset_commit_response;
pub use self::offset_commit_response::{offset_commit_response, OffsetCommitResponse};
pub mod offset_fetch_request;
pub use self::offset_fetch_request::{offset_fetch_request, OffsetFetchRequest};
pub mod offset_fetch_response;
pub use self::offset_fetch_response::{offset_fetch_response, OffsetFetchResponse};
pub mod find_coordinator_request;
pub use self::find_coordinator_request::{find_coordinator_request, FindCoordinatorRequest};
pub mod find_coordinator_response;
pub use self::find_coordinator_response::{find_coordinator_response, FindCoordinatorResponse};
pub mod join_group_request;
pub use self::join_group_request::{join_group_request, JoinGroupRequest};
pub mod join_group_response;
pub use self::join_group_response::{join_group_response, JoinGroupResponse};
pub mod heartbeat_request;
pub use self::heartbeat_request::{heartbeat_request, HeartbeatRequest};
pub mod heartbeat_response;
pub use self::heartbeat_response::{heartbeat_response, HeartbeatResponse};
pub mod leave_group_request;
pub use self::leave_group_request::{leave_group_request, LeaveGroupRequest};
pub mod leave_group_response;
pub use self::leave_group_response::{leave_group_response, LeaveGroupResponse};
pub mod sync_group_request;
pub use self::sync_group_request::{sync_group_request, SyncGroupRequest};
pub mod sync_group_response;
pub use self::sync_group_response::{sync_group_response, SyncGroupResponse};
pub mod describe_groups_request;
pub use self::describe_groups_request::{describe_groups_request, DescribeGroupsRequest};
pub mod describe_groups_response;
pub use self::describe_groups_response::{describe_groups_response, DescribeGroupsResponse};
pub mod list_groups_request;
pub use self::list_groups_request::{list_groups_request, ListGroupsRequest};
pub mod list_groups_response;
pub use self::list_groups_response::{list_groups_response, ListGroupsResponse};
pub mod sasl_handshake_request;
pub use self::sasl_handshake_request::{sasl_handshake_request, SaslHandshakeRequest};
pub mod sasl_handshake_response;
pub use self::sasl_handshake_response::{sasl_handshake_response, SaslHandshakeResponse};
pub mod api_versions_request;
pub use self::api_versions_request::{api_versions_request, ApiVersionsRequest};
pub mod api_versions_response;
pub use self::api_versions_response::{api_versions_response, ApiVersionsResponse};
pub mod create_topics_request;
pub use self::create_topics_request::{create_topics_request, CreateTopicsRequest};
pub mod create_topics_response;
pub use self::create_topics_response::{create_topics_response, CreateTopicsResponse};
pub mod delete_topics_request;
pub use self::delete_topics_request::{delete_topics_request, DeleteTopicsRequest};
pub mod delete_topics_response;
pub use self::delete_topics_response::{delete_topics_response, DeleteTopicsResponse};
pub mod delete_records_request;
pub use self::delete_records_request::{delete_records_request, DeleteRecordsRequest};
pub mod delete_records_response;
pub use self::delete_records_response::{delete_records_response, DeleteRecordsResponse};
pub mod init_producer_id_request;
pub use self::init_producer_id_request::{init_producer_id_request, InitProducerIdRequest};
pub mod init_producer_id_response;
pub use self::init_producer_id_response::{init_producer_id_response, InitProducerIdResponse};
pub mod offset_for_leader_epoch_request;
pub use self::offset_for_leader_epoch_request::{
    offset_for_leader_epoch_request, OffsetForLeaderEpochRequest,
};
pub mod offset_for_leader_epoch_response;
pub use self::offset_for_leader_epoch_response::{
    offset_for_leader_epoch_response, OffsetForLeaderEpochResponse,
};
pub mod add_partitions_to_txn_request;
pub use self::add_partitions_to_txn_request::{
    add_partitions_to_txn_request, AddPartitionsToTxnRequest,
};
pub mod add_partitions_to_txn_response;
pub use self::add_partitions_to_txn_response::{
    add_partitions_to_txn_response, AddPartitionsToTxnResponse,
};
pub mod add_offsets_to_txn_request;
pub use self::add_offsets_to_txn_request::{add_offsets_to_txn_request, AddOffsetsToTxnRequest};
pub mod add_offsets_to_txn_response;
pub use self::add_offsets_to_txn_response::{add_offsets_to_txn_response, AddOffsetsToTxnResponse};
pub mod end_txn_request;
pub use self::end_txn_request::{end_txn_request, EndTxnRequest};
pub mod end_txn_response;
pub use self::end_txn_response::{end_txn_response, EndTxnResponse};
pub mod write_txn_markers_request;
pub use self::write_txn_markers_request::{write_txn_markers_request, WriteTxnMarkersRequest};
pub mod write_txn_markers_response;
pub use self::write_txn_markers_response::{write_txn_markers_response, WriteTxnMarkersResponse};
pub mod txn_offset_commit_request;
pub use self::txn_offset_commit_request::{txn_offset_commit_request, TxnOffsetCommitRequest};
pub mod txn_offset_commit_response;
pub use self::txn_offset_commit_response::{txn_offset_commit_response, TxnOffsetCommitResponse};
pub mod describe_acls_request;
pub use self::describe_acls_request::{describe_acls_request, DescribeAclsRequest};
pub mod describe_acls_response;
pub use self::describe_acls_response::{describe_acls_response, DescribeAclsResponse};
pub mod create_acls_request;
pub use self::create_acls_request::{create_acls_request, CreateAclsRequest};
pub mod create_acls_response;
pub use self::create_acls_response::{create_acls_response, CreateAclsResponse};
pub mod delete_acls_request;
pub use self::delete_acls_request::{delete_acls_request, DeleteAclsRequest};
pub mod delete_acls_response;
pub use self::delete_acls_response::{delete_acls_response, DeleteAclsResponse};
pub mod describe_configs_request;
pub use self::describe_configs_request::{describe_configs_request, DescribeConfigsRequest};
pub mod describe_configs_response;
pub use self::describe_configs_response::{describe_configs_response, DescribeConfigsResponse};
pub mod alter_configs_request;
pub use self::alter_configs_request::{alter_configs_request, AlterConfigsRequest};
pub mod alter_configs_response;
pub use self::alter_configs_response::{alter_configs_response, AlterConfigsResponse};
pub mod alter_replica_log_dirs_request;
pub use self::alter_replica_log_dirs_request::{
    alter_replica_log_dirs_request, AlterReplicaLogDirsRequest,
};
pub mod alter_replica_log_dirs_response;
pub use self::alter_replica_log_dirs_response::{
    alter_replica_log_dirs_response, AlterReplicaLogDirsResponse,
};
pub mod describe_log_dirs_request;
pub use self::describe_log_dirs_request::{describe_log_dirs_request, DescribeLogDirsRequest};
pub mod describe_log_dirs_response;
pub use self::describe_log_dirs_response::{describe_log_dirs_response, DescribeLogDirsResponse};
pub mod sasl_authenticate_request;
pub use self::sasl_authenticate_request::{sasl_authenticate_request, SaslAuthenticateRequest};
pub mod sasl_authenticate_response;
pub use self::sasl_authenticate_response::{sasl_authenticate_response, SaslAuthenticateResponse};
pub mod create_partitions_request;
pub use self::create_partitions_request::{create_partitions_request, CreatePartitionsRequest};
pub mod create_partitions_response;
pub use self::create_partitions_response::{create_partitions_response, CreatePartitionsResponse};
pub mod create_delegation_token_request;
pub use self::create_delegation_token_request::{
    create_delegation_token_request, CreateDelegationTokenRequest,
};
pub mod create_delegation_token_response;
pub use self::create_delegation_token_response::{
    create_delegation_token_response, CreateDelegationTokenResponse,
};
pub mod renew_delegation_token_request;
pub use self::renew_delegation_token_request::{
    renew_delegation_token_request, RenewDelegationTokenRequest,
};
pub mod renew_delegation_token_response;
pub use self::renew_delegation_token_response::{
    renew_delegation_token_response, RenewDelegationTokenResponse,
};
pub mod expire_delegation_token_request;
pub use self::expire_delegation_token_request::{
    expire_delegation_token_request, ExpireDelegationTokenRequest,
};
pub mod expire_delegation_token_response;
pub use self::expire_delegation_token_response::{
    expire_delegation_token_response, ExpireDelegationTokenResponse,
};
pub mod describe_delegation_token_request;
pub use self::describe_delegation_token_request::{
    describe_delegation_token_request, DescribeDelegationTokenRequest,
};
pub mod describe_delegation_token_response;
pub use self::describe_delegation_token_response::{
    describe_delegation_token_response, DescribeDelegationTokenResponse,
};
pub mod delete_groups_request;
pub use self::delete_groups_request::{delete_groups_request, DeleteGroupsRequest};
pub mod delete_groups_response;
pub use self::delete_groups_response::{delete_groups_response, DeleteGroupsResponse};
pub mod elect_preferred_leaders_request;
pub use self::elect_preferred_leaders_request::{
    elect_preferred_leaders_request, ElectPreferredLeadersRequest,
};
pub mod elect_preferred_leaders_response;
pub use self::elect_preferred_leaders_response::{
    elect_preferred_leaders_response, ElectPreferredLeadersResponse,
};
pub mod incremental_alter_configs_request;
pub use self::incremental_alter_configs_request::{
    incremental_alter_configs_request, IncrementalAlterConfigsRequest,
};
pub mod incremental_alter_configs_response;
pub use self::incremental_alter_configs_response::{
    incremental_alter_configs_response, IncrementalAlterConfigsResponse,
};
pub mod record_set;
pub use self::record_set::{record_set, RecordSet};
pub mod record;
pub use self::record::{record, Record};
pub mod protocol_metadata;
pub use self::protocol_metadata::{protocol_metadata, ProtocolMetadata};
pub mod member_assignment;
pub use self::member_assignment::{member_assignment, MemberAssignment};
impl<I> Client<I>
where
    I: AsyncRead + AsyncWrite + std::marker::Unpin,
{
    pub async fn add_offsets_to_txn<'i>(
        &'i mut self,
        request: AddOffsetsToTxnRequest<'_>,
    ) -> Result<AddOffsetsToTxnResponse> where {
        self.call(
            request,
            ApiKey::ADD_OFFSETS_TO_TXN,
            add_offsets_to_txn_request::VERSION,
            add_offsets_to_txn_response(),
        )
        .await
    }
    pub async fn add_partitions_to_txn<'i>(
        &'i mut self,
        request: AddPartitionsToTxnRequest<'_>,
    ) -> Result<AddPartitionsToTxnResponse<'i>> where {
        self.call(
            request,
            ApiKey::ADD_PARTITIONS_TO_TXN,
            add_partitions_to_txn_request::VERSION,
            add_partitions_to_txn_response(),
        )
        .await
    }
    pub async fn alter_configs<'i>(
        &'i mut self,
        request: AlterConfigsRequest<'_>,
    ) -> Result<AlterConfigsResponse<'i>> where {
        self.call(
            request,
            ApiKey::ALTER_CONFIGS,
            alter_configs_request::VERSION,
            alter_configs_response(),
        )
        .await
    }
    pub async fn alter_replica_log_dirs<'i>(
        &'i mut self,
        request: AlterReplicaLogDirsRequest<'_>,
    ) -> Result<AlterReplicaLogDirsResponse<'i>> where {
        self.call(
            request,
            ApiKey::ALTER_REPLICA_LOG_DIRS,
            alter_replica_log_dirs_request::VERSION,
            alter_replica_log_dirs_response(),
        )
        .await
    }
    pub async fn api_versions<'i>(
        &'i mut self,
        request: ApiVersionsRequest,
    ) -> Result<ApiVersionsResponse> where {
        self.call(
            request,
            ApiKey::API_VERSIONS,
            api_versions_request::VERSION,
            api_versions_response(),
        )
        .await
    }
    pub async fn controlled_shutdown<'i>(
        &'i mut self,
        request: ControlledShutdownRequest,
    ) -> Result<ControlledShutdownResponse<'i>> where {
        self.call(
            request,
            ApiKey::CONTROLLED_SHUTDOWN,
            controlled_shutdown_request::VERSION,
            controlled_shutdown_response(),
        )
        .await
    }
    pub async fn create_acls<'i>(
        &'i mut self,
        request: CreateAclsRequest<'_>,
    ) -> Result<CreateAclsResponse<'i>> where {
        self.call(
            request,
            ApiKey::CREATE_ACLS,
            create_acls_request::VERSION,
            create_acls_response(),
        )
        .await
    }
    pub async fn create_delegation_token<'i>(
        &'i mut self,
        request: CreateDelegationTokenRequest<'_>,
    ) -> Result<CreateDelegationTokenResponse<'i>> where {
        self.call(
            request,
            ApiKey::CREATE_DELEGATION_TOKEN,
            create_delegation_token_request::VERSION,
            create_delegation_token_response(),
        )
        .await
    }
    pub async fn create_partitions<'i>(
        &'i mut self,
        request: CreatePartitionsRequest<'_>,
    ) -> Result<CreatePartitionsResponse<'i>> where {
        self.call(
            request,
            ApiKey::CREATE_PARTITIONS,
            create_partitions_request::VERSION,
            create_partitions_response(),
        )
        .await
    }
    pub async fn create_topics<'i>(
        &'i mut self,
        request: CreateTopicsRequest<'_>,
    ) -> Result<CreateTopicsResponse<'i>> where {
        self.call(
            request,
            ApiKey::CREATE_TOPICS,
            create_topics_request::VERSION,
            create_topics_response(),
        )
        .await
    }
    pub async fn delete_acls<'i>(
        &'i mut self,
        request: DeleteAclsRequest<'_>,
    ) -> Result<DeleteAclsResponse<'i>> where {
        self.call(
            request,
            ApiKey::DELETE_ACLS,
            delete_acls_request::VERSION,
            delete_acls_response(),
        )
        .await
    }
    pub async fn delete_groups<'i>(
        &'i mut self,
        request: DeleteGroupsRequest<'_>,
    ) -> Result<DeleteGroupsResponse<'i>> where {
        self.call(
            request,
            ApiKey::DELETE_GROUPS,
            delete_groups_request::VERSION,
            delete_groups_response(),
        )
        .await
    }
    pub async fn delete_records<'i>(
        &'i mut self,
        request: DeleteRecordsRequest<'_>,
    ) -> Result<DeleteRecordsResponse<'i>> where {
        self.call(
            request,
            ApiKey::DELETE_RECORDS,
            delete_records_request::VERSION,
            delete_records_response(),
        )
        .await
    }
    pub async fn delete_topics<'i>(
        &'i mut self,
        request: DeleteTopicsRequest<'_>,
    ) -> Result<DeleteTopicsResponse<'i>> where {
        self.call(
            request,
            ApiKey::DELETE_TOPICS,
            delete_topics_request::VERSION,
            delete_topics_response(),
        )
        .await
    }
    pub async fn describe_acls<'i>(
        &'i mut self,
        request: DescribeAclsRequest<'_>,
    ) -> Result<DescribeAclsResponse<'i>> where {
        self.call(
            request,
            ApiKey::DESCRIBE_ACLS,
            describe_acls_request::VERSION,
            describe_acls_response(),
        )
        .await
    }
    pub async fn describe_configs<'i>(
        &'i mut self,
        request: DescribeConfigsRequest<'_>,
    ) -> Result<DescribeConfigsResponse<'i>> where {
        self.call(
            request,
            ApiKey::DESCRIBE_CONFIGS,
            describe_configs_request::VERSION,
            describe_configs_response(),
        )
        .await
    }
    pub async fn describe_delegation_token<'i>(
        &'i mut self,
        request: DescribeDelegationTokenRequest<'_>,
    ) -> Result<DescribeDelegationTokenResponse<'i>> where {
        self.call(
            request,
            ApiKey::DESCRIBE_DELEGATION_TOKEN,
            describe_delegation_token_request::VERSION,
            describe_delegation_token_response(),
        )
        .await
    }
    pub async fn describe_groups<'i>(
        &'i mut self,
        request: DescribeGroupsRequest<'_>,
    ) -> Result<DescribeGroupsResponse<'i>> where {
        self.call(
            request,
            ApiKey::DESCRIBE_GROUPS,
            describe_groups_request::VERSION,
            describe_groups_response(),
        )
        .await
    }
    pub async fn describe_log_dirs<'i>(
        &'i mut self,
        request: DescribeLogDirsRequest<'_>,
    ) -> Result<DescribeLogDirsResponse<'i>> where {
        self.call(
            request,
            ApiKey::DESCRIBE_LOG_DIRS,
            describe_log_dirs_request::VERSION,
            describe_log_dirs_response(),
        )
        .await
    }
    pub async fn elect_preferred_leaders<'i>(
        &'i mut self,
        request: ElectPreferredLeadersRequest<'_>,
    ) -> Result<ElectPreferredLeadersResponse<'i>> where {
        self.call(
            request,
            ApiKey::ELECT_PREFERRED_LEADERS,
            elect_preferred_leaders_request::VERSION,
            elect_preferred_leaders_response(),
        )
        .await
    }
    pub async fn end_txn<'i>(&'i mut self, request: EndTxnRequest<'_>) -> Result<EndTxnResponse> where
    {
        self.call(
            request,
            ApiKey::END_TXN,
            end_txn_request::VERSION,
            end_txn_response(),
        )
        .await
    }
    pub async fn expire_delegation_token<'i>(
        &'i mut self,
        request: ExpireDelegationTokenRequest<'_>,
    ) -> Result<ExpireDelegationTokenResponse> where {
        self.call(
            request,
            ApiKey::EXPIRE_DELEGATION_TOKEN,
            expire_delegation_token_request::VERSION,
            expire_delegation_token_response(),
        )
        .await
    }
    pub async fn fetch<'i, R>(
        &'i mut self,
        request: FetchRequest<'_>,
    ) -> Result<FetchResponse<'i, R>>
    where
        R: RecordBatchParser<combine::stream::easy::Stream<&'i [u8]>> + 'i,
    {
        self.call(
            request,
            ApiKey::FETCH,
            fetch_request::VERSION,
            fetch_response(),
        )
        .await
    }
    pub async fn find_coordinator<'i>(
        &'i mut self,
        request: FindCoordinatorRequest<'_>,
    ) -> Result<FindCoordinatorResponse<'i>> where {
        self.call(
            request,
            ApiKey::FIND_COORDINATOR,
            find_coordinator_request::VERSION,
            find_coordinator_response(),
        )
        .await
    }
    pub async fn heartbeat<'i>(
        &'i mut self,
        request: HeartbeatRequest<'_>,
    ) -> Result<HeartbeatResponse> where {
        self.call(
            request,
            ApiKey::HEARTBEAT,
            heartbeat_request::VERSION,
            heartbeat_response(),
        )
        .await
    }
    pub async fn incremental_alter_configs<'i>(
        &'i mut self,
        request: IncrementalAlterConfigsRequest<'_>,
    ) -> Result<IncrementalAlterConfigsResponse<'i>> where {
        self.call(
            request,
            ApiKey::INCREMENTAL_ALTER_CONFIGS,
            incremental_alter_configs_request::VERSION,
            incremental_alter_configs_response(),
        )
        .await
    }
    pub async fn init_producer_id<'i>(
        &'i mut self,
        request: InitProducerIdRequest<'_>,
    ) -> Result<InitProducerIdResponse> where {
        self.call(
            request,
            ApiKey::INIT_PRODUCER_ID,
            init_producer_id_request::VERSION,
            init_producer_id_response(),
        )
        .await
    }
    pub async fn join_group<'i>(
        &'i mut self,
        request: JoinGroupRequest<'_>,
    ) -> Result<JoinGroupResponse<'i>> where {
        self.call(
            request,
            ApiKey::JOIN_GROUP,
            join_group_request::VERSION,
            join_group_response(),
        )
        .await
    }
    pub async fn leader_and_isr<'i>(
        &'i mut self,
        request: LeaderAndIsrRequest<'_>,
    ) -> Result<LeaderAndIsrResponse<'i>> where {
        self.call(
            request,
            ApiKey::LEADER_AND_ISR,
            leader_and_isr_request::VERSION,
            leader_and_isr_response(),
        )
        .await
    }
    pub async fn leave_group<'i>(
        &'i mut self,
        request: LeaveGroupRequest<'_>,
    ) -> Result<LeaveGroupResponse> where {
        self.call(
            request,
            ApiKey::LEAVE_GROUP,
            leave_group_request::VERSION,
            leave_group_response(),
        )
        .await
    }
    pub async fn list_groups<'i>(
        &'i mut self,
        request: ListGroupsRequest,
    ) -> Result<ListGroupsResponse<'i>> where {
        self.call(
            request,
            ApiKey::LIST_GROUPS,
            list_groups_request::VERSION,
            list_groups_response(),
        )
        .await
    }
    pub async fn list_offsets<'i>(
        &'i mut self,
        request: ListOffsetsRequest<'_>,
    ) -> Result<ListOffsetsResponse<'i>> where {
        self.call(
            request,
            ApiKey::LIST_OFFSETS,
            list_offsets_request::VERSION,
            list_offsets_response(),
        )
        .await
    }
    pub async fn metadata<'i>(
        &'i mut self,
        request: MetadataRequest<'_>,
    ) -> Result<MetadataResponse<'i>> where {
        self.call(
            request,
            ApiKey::METADATA,
            metadata_request::VERSION,
            metadata_response(),
        )
        .await
    }
    pub async fn offset_commit<'i>(
        &'i mut self,
        request: OffsetCommitRequest<'_>,
    ) -> Result<OffsetCommitResponse<'i>> where {
        self.call(
            request,
            ApiKey::OFFSET_COMMIT,
            offset_commit_request::VERSION,
            offset_commit_response(),
        )
        .await
    }
    pub async fn offset_fetch<'i>(
        &'i mut self,
        request: OffsetFetchRequest<'_>,
    ) -> Result<OffsetFetchResponse<'i>> where {
        self.call(
            request,
            ApiKey::OFFSET_FETCH,
            offset_fetch_request::VERSION,
            offset_fetch_response(),
        )
        .await
    }
    pub async fn offset_for_leader_epoch<'i>(
        &'i mut self,
        request: OffsetForLeaderEpochRequest<'_>,
    ) -> Result<OffsetForLeaderEpochResponse<'i>> where {
        self.call(
            request,
            ApiKey::OFFSET_FOR_LEADER_EPOCH,
            offset_for_leader_epoch_request::VERSION,
            offset_for_leader_epoch_response(),
        )
        .await
    }
    pub async fn produce<'i, R>(
        &'i mut self,
        request: ProduceRequest<'_, R>,
    ) -> Result<ProduceResponse<'i>>
    where
        R: Encode,
    {
        self.call(
            request,
            ApiKey::PRODUCE,
            produce_request::VERSION,
            produce_response(),
        )
        .await
    }
    pub async fn renew_delegation_token<'i>(
        &'i mut self,
        request: RenewDelegationTokenRequest<'_>,
    ) -> Result<RenewDelegationTokenResponse> where {
        self.call(
            request,
            ApiKey::RENEW_DELEGATION_TOKEN,
            renew_delegation_token_request::VERSION,
            renew_delegation_token_response(),
        )
        .await
    }
    pub async fn sasl_authenticate<'i>(
        &'i mut self,
        request: SaslAuthenticateRequest<'_>,
    ) -> Result<SaslAuthenticateResponse<'i>> where {
        self.call(
            request,
            ApiKey::SASL_AUTHENTICATE,
            sasl_authenticate_request::VERSION,
            sasl_authenticate_response(),
        )
        .await
    }
    pub async fn sasl_handshake<'i>(
        &'i mut self,
        request: SaslHandshakeRequest<'_>,
    ) -> Result<SaslHandshakeResponse<'i>> where {
        self.call(
            request,
            ApiKey::SASL_HANDSHAKE,
            sasl_handshake_request::VERSION,
            sasl_handshake_response(),
        )
        .await
    }
    pub async fn stop_replica<'i>(
        &'i mut self,
        request: StopReplicaRequest<'_>,
    ) -> Result<StopReplicaResponse<'i>> where {
        self.call(
            request,
            ApiKey::STOP_REPLICA,
            stop_replica_request::VERSION,
            stop_replica_response(),
        )
        .await
    }
    pub async fn sync_group<'i>(
        &'i mut self,
        request: SyncGroupRequest<'_>,
    ) -> Result<SyncGroupResponse<'i>> where {
        self.call(
            request,
            ApiKey::SYNC_GROUP,
            sync_group_request::VERSION,
            sync_group_response(),
        )
        .await
    }
    pub async fn txn_offset_commit<'i>(
        &'i mut self,
        request: TxnOffsetCommitRequest<'_>,
    ) -> Result<TxnOffsetCommitResponse<'i>> where {
        self.call(
            request,
            ApiKey::TXN_OFFSET_COMMIT,
            txn_offset_commit_request::VERSION,
            txn_offset_commit_response(),
        )
        .await
    }
    pub async fn update_metadata<'i>(
        &'i mut self,
        request: UpdateMetadataRequest<'_>,
    ) -> Result<UpdateMetadataResponse> where {
        self.call(
            request,
            ApiKey::UPDATE_METADATA,
            update_metadata_request::VERSION,
            update_metadata_response(),
        )
        .await
    }
    pub async fn write_txn_markers<'i>(
        &'i mut self,
        request: WriteTxnMarkersRequest<'_>,
    ) -> Result<WriteTxnMarkersResponse<'i>> where {
        self.call(
            request,
            ApiKey::WRITE_TXN_MARKERS,
            write_txn_markers_request::VERSION,
            write_txn_markers_response(),
        )
        .await
    }
}
