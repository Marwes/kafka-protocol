use std::convert::TryFrom;
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ErrorCode {
    /// The server experienced an unexpected error when processing the request.
    UnknownServerError = -1,
    None = 0,
    /// The requested offset is not within the range of offsets maintained by the server.
    OffsetOutOfRange = 1,
    /// This message has failed its CRC checksum, exceeds the valid size, has a null key for a compacted topic, or is otherwise corrupt.
    CorruptMessage = 2,
    /// This server does not host this topic-partition.
    UnknownTopicOrPartition = 3,
    /// The requested fetch size is invalid.
    InvalidFetchSize = 4,
    /// There is no leader for this topic-partition as we are in the middle of a leadership election.
    LeaderNotAvailable = 5,
    /// This server is not the leader for that topic-partition.
    NotLeaderForPartition = 6,
    /// The request timed out.
    RequestTimedOut = 7,
    /// The broker is not available.
    BrokerNotAvailable = 8,
    /// The replica is not available for the requested topic-partition.
    ReplicaNotAvailable = 9,
    /// The request included a message larger than the max message size the server will accept.
    MessageTooLarge = 10,
    /// The controller moved to another broker.
    StaleControllerEpoch = 11,
    /// The metadata field of the offset request was too large.
    OffsetMetadataTooLarge = 12,
    /// The server disconnected before a response was received.
    NetworkException = 13,
    /// The coordinator is loading and hence can't process requests.
    CoordinatorLoadInProgress = 14,
    /// The coordinator is not available.
    CoordinatorNotAvailable = 15,
    /// This is not the correct coordinator.
    NotCoordinator = 16,
    /// The request attempted to perform an operation on an invalid topic.
    InvalidTopicException = 17,
    /// The request included message batch larger than the configured segment size on the server.
    RecordListTooLarge = 18,
    /// Messages are rejected since there are fewer in-sync replicas than required.
    NotEnoughReplicas = 19,
    /// Messages are written to the log, but to fewer in-sync replicas than required.
    NotEnoughReplicasAfterAppend = 20,
    /// Produce request specified an invalid value for required acks.
    InvalidRequiredAcks = 21,
    /// Specified group generation id is not valid.
    IllegalGeneration = 22,
    /// The group member's supported protocols are incompatible with those of existing members or first group member tried to join with empty protocol type or empty protocol list.
    InconsistentGroupProtocol = 23,
    /// The configured groupId is invalid.
    InvalidGroupId = 24,
    /// The coordinator is not aware of this member.
    UnknownMemberId = 25,
    /// The session timeout is not within the range allowed by the broker (as configured by group.min.session.timeout.ms and group.max.session.timeout.ms).
    InvalidSessionTimeout = 26,
    /// The group is rebalancing, so a rejoin is needed.
    RebalanceInProgress = 27,
    /// The committing offset data size is not valid.
    InvalidCommitOffsetSize = 28,
    /// Not authorized to access topics: [Topic authorization failed.]
    TopicAuthorizationFailed = 29,
    /// Not authorized to access group: Group authorization failed.
    GroupAuthorizationFailed = 30,
    /// Cluster authorization failed.
    ClusterAuthorizationFailed = 31,
    /// The timestamp of the message is out of acceptable range.
    InvalidTimestamp = 32,
    /// The broker does not support the requested SASL mechanism.
    UnsupportedSaslMechanism = 33,
    /// Request is not valid given the current SASL state.
    IllegalSaslState = 34,
    /// The version of API is not supported.
    UnsupportedVersion = 35,
    /// Topic with this name already exists.
    TopicAlreadyExists = 36,
    /// Number of partitions is below 1.
    InvalidPartitions = 37,
    /// Replication factor is below 1 or larger than the number of available brokers.
    InvalidReplicationFactor = 38,
    /// Replica assignment is invalid.
    InvalidReplicaAssignment = 39,
    /// Configuration is invalid.
    InvalidConfig = 40,
    /// This is not the correct controller for this cluster.
    NotController = 41,
    /// This most likely occurs because of a request being malformed by the client library or the message was sent to an incompatible broker. See the broker logs for more details.
    InvalidRequest = 42,
    /// The message format version on the broker does not support the request.
    UnsupportedForMessageFormat = 43,
    /// Request parameters do not satisfy the configured policy.
    PolicyViolation = 44,
    /// The broker received an out of order sequence number.
    OutOfOrderSequenceNumber = 45,
    /// The broker received a duplicate sequence number.
    DuplicateSequenceNumber = 46,
    /// Producer attempted an operation with an old epoch. Either there is a newer producer with the same transactionalId, or the producer's transaction has been expired by the broker.
    InvalidProducerEpoch = 47,
    /// The producer attempted a transactional operation in an invalid state.
    InvalidTxnState = 48,
    /// The producer attempted to use a producer id which is not currently assigned to its transactional id.
    InvalidProducerIdMapping = 49,
    /// The transaction timeout is larger than the maximum value allowed by the broker (as configured by transaction.max.timeout.ms).
    InvalidTransactionTimeout = 50,
    /// The producer attempted to update a transaction while another concurrent operation on the same transaction was ongoing.
    ConcurrentTransactions = 51,
    /// Indicates that the transaction coordinator sending a WriteTxnMarker is no longer the current coordinator for a given producer.
    TransactionCoordinatorFenced = 52,
    /// Transactional Id authorization failed.
    TransactionalIdAuthorizationFailed = 53,
    /// Security features are disabled.
    SecurityDisabled = 54,
    /// The broker did not attempt to execute this operation. This may happen for batched RPCs where some operations in the batch failed, causing the broker to respond without trying the rest.
    OperationNotAttempted = 55,
    /// Disk error when trying to access log file on the disk.
    KafkaStorageError = 56,
    /// The user-specified log directory is not found in the broker config.
    LogDirNotFound = 57,
    /// SASL Authentication failed.
    SaslAuthenticationFailed = 58,
    /// This exception is raised by the broker if it could not locate the producer metadata associated with the producerId in question. This could happen if, for instance, the producer's records were deleted because their retention time had elapsed. Once the last records of the producerId are removed, the producer's metadata is removed from the broker, and future appends by the producer will return this exception.
    UnknownProducerId = 59,
    /// A partition reassignment is in progress.
    ReassignmentInProgress = 60,
    /// Delegation Token feature is not enabled.
    DelegationTokenAuthDisabled = 61,
    /// Delegation Token is not found on server.
    DelegationTokenNotFound = 62,
    /// Specified Principal is not valid Owner/Renewer.
    DelegationTokenOwnerMismatch = 63,
    /// Delegation Token requests are not allowed on PLAINTEXT/1-way SSL channels and on delegation token authenticated channels.
    DelegationTokenRequestNotAllowed = 64,
    /// Delegation Token authorization failed.
    DelegationTokenAuthorizationFailed = 65,
    /// Delegation Token is expired.
    DelegationTokenExpired = 66,
    /// Supplied principalType is not supported.
    InvalidPrincipalType = 67,
    /// The group is not empty.
    NonEmptyGroup = 68,
    /// The group id does not exist.
    GroupIdNotFound = 69,
    /// The fetch session ID was not found.
    FetchSessionIdNotFound = 70,
    /// The fetch session epoch is invalid.
    InvalidFetchSessionEpoch = 71,
    /// There is no listener on the leader broker that matches the listener on which metadata request was processed.
    ListenerNotFound = 72,
    /// Topic deletion is disabled.
    TopicDeletionDisabled = 73,
    /// The leader epoch in the request is older than the epoch on the broker
    FencedLeaderEpoch = 74,
    /// The leader epoch in the request is newer than the epoch on the broker
    UnknownLeaderEpoch = 75,
    /// The requesting client does not support the compression type of given partition.
    UnsupportedCompressionType = 76,
    /// Broker epoch has changed
    StaleBrokerEpoch = 77,
    /// The leader high watermark has not caught up from a recent leader election so the offsets cannot be guaranteed to be monotonically increasing
    OffsetNotAvailable = 78,
    /// The group member needs to have a valid member id before actually entering a consumer group
    MemberIdRequired = 79,
    /// The preferred leader was not available
    PreferredLeaderNotAvailable = 80,
    /// Consumer group The consumer group has reached its max size. already has the configured maximum number of members.
    GroupMaxSizeReached = 81,
    /// The broker rejected this static consumer since another consumer with the same group.instance.id has registered with a different member.id.
    FencedInstanceId = 82,
    /// Eligible topic partition leaders are not available.
    EligibleLeadersNotAvailable = 83,
    /// Leader election not needed for topic partition.
    ElectionNotNeeded = 84,
    /// No partition reassignment is in progress.
    NoReassignmentInProgress = 85,
    /// Deleting offsets of a topic is forbidden while the consumer group is actively subscribed to it.
    GroupSubscribedToTopic = 86,
    /// This record has failed the validation on broker and hence will be rejected.
    InvalidRecord = 87,
    /// There are unstable offsets that need to be cleared.
    UnstableOffsetCommit = 88,
}
impl TryFrom<i16> for ErrorCode {
    type Error = &'static str;
    fn try_from(i: i16) -> Result<Self, Self::Error> {
        Ok(match i {
            -1 => ErrorCode::UnknownServerError,
            0 => ErrorCode::None,
            1 => ErrorCode::OffsetOutOfRange,
            2 => ErrorCode::CorruptMessage,
            3 => ErrorCode::UnknownTopicOrPartition,
            4 => ErrorCode::InvalidFetchSize,
            5 => ErrorCode::LeaderNotAvailable,
            6 => ErrorCode::NotLeaderForPartition,
            7 => ErrorCode::RequestTimedOut,
            8 => ErrorCode::BrokerNotAvailable,
            9 => ErrorCode::ReplicaNotAvailable,
            10 => ErrorCode::MessageTooLarge,
            11 => ErrorCode::StaleControllerEpoch,
            12 => ErrorCode::OffsetMetadataTooLarge,
            13 => ErrorCode::NetworkException,
            14 => ErrorCode::CoordinatorLoadInProgress,
            15 => ErrorCode::CoordinatorNotAvailable,
            16 => ErrorCode::NotCoordinator,
            17 => ErrorCode::InvalidTopicException,
            18 => ErrorCode::RecordListTooLarge,
            19 => ErrorCode::NotEnoughReplicas,
            20 => ErrorCode::NotEnoughReplicasAfterAppend,
            21 => ErrorCode::InvalidRequiredAcks,
            22 => ErrorCode::IllegalGeneration,
            23 => ErrorCode::InconsistentGroupProtocol,
            24 => ErrorCode::InvalidGroupId,
            25 => ErrorCode::UnknownMemberId,
            26 => ErrorCode::InvalidSessionTimeout,
            27 => ErrorCode::RebalanceInProgress,
            28 => ErrorCode::InvalidCommitOffsetSize,
            29 => ErrorCode::TopicAuthorizationFailed,
            30 => ErrorCode::GroupAuthorizationFailed,
            31 => ErrorCode::ClusterAuthorizationFailed,
            32 => ErrorCode::InvalidTimestamp,
            33 => ErrorCode::UnsupportedSaslMechanism,
            34 => ErrorCode::IllegalSaslState,
            35 => ErrorCode::UnsupportedVersion,
            36 => ErrorCode::TopicAlreadyExists,
            37 => ErrorCode::InvalidPartitions,
            38 => ErrorCode::InvalidReplicationFactor,
            39 => ErrorCode::InvalidReplicaAssignment,
            40 => ErrorCode::InvalidConfig,
            41 => ErrorCode::NotController,
            42 => ErrorCode::InvalidRequest,
            43 => ErrorCode::UnsupportedForMessageFormat,
            44 => ErrorCode::PolicyViolation,
            45 => ErrorCode::OutOfOrderSequenceNumber,
            46 => ErrorCode::DuplicateSequenceNumber,
            47 => ErrorCode::InvalidProducerEpoch,
            48 => ErrorCode::InvalidTxnState,
            49 => ErrorCode::InvalidProducerIdMapping,
            50 => ErrorCode::InvalidTransactionTimeout,
            51 => ErrorCode::ConcurrentTransactions,
            52 => ErrorCode::TransactionCoordinatorFenced,
            53 => ErrorCode::TransactionalIdAuthorizationFailed,
            54 => ErrorCode::SecurityDisabled,
            55 => ErrorCode::OperationNotAttempted,
            56 => ErrorCode::KafkaStorageError,
            57 => ErrorCode::LogDirNotFound,
            58 => ErrorCode::SaslAuthenticationFailed,
            59 => ErrorCode::UnknownProducerId,
            60 => ErrorCode::ReassignmentInProgress,
            61 => ErrorCode::DelegationTokenAuthDisabled,
            62 => ErrorCode::DelegationTokenNotFound,
            63 => ErrorCode::DelegationTokenOwnerMismatch,
            64 => ErrorCode::DelegationTokenRequestNotAllowed,
            65 => ErrorCode::DelegationTokenAuthorizationFailed,
            66 => ErrorCode::DelegationTokenExpired,
            67 => ErrorCode::InvalidPrincipalType,
            68 => ErrorCode::NonEmptyGroup,
            69 => ErrorCode::GroupIdNotFound,
            70 => ErrorCode::FetchSessionIdNotFound,
            71 => ErrorCode::InvalidFetchSessionEpoch,
            72 => ErrorCode::ListenerNotFound,
            73 => ErrorCode::TopicDeletionDisabled,
            74 => ErrorCode::FencedLeaderEpoch,
            75 => ErrorCode::UnknownLeaderEpoch,
            76 => ErrorCode::UnsupportedCompressionType,
            77 => ErrorCode::StaleBrokerEpoch,
            78 => ErrorCode::OffsetNotAvailable,
            79 => ErrorCode::MemberIdRequired,
            80 => ErrorCode::PreferredLeaderNotAvailable,
            81 => ErrorCode::GroupMaxSizeReached,
            82 => ErrorCode::FencedInstanceId,
            83 => ErrorCode::EligibleLeadersNotAvailable,
            84 => ErrorCode::ElectionNotNeeded,
            85 => ErrorCode::NoReassignmentInProgress,
            86 => ErrorCode::GroupSubscribedToTopic,
            87 => ErrorCode::InvalidRecord,
            88 => ErrorCode::UnstableOffsetCommit,
            _ => return Err("Invalid ErrorCode"),
        })
    }
}
impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::UnknownServerError => f.write_str("The server experienced an unexpected error when processing the request."),
            ErrorCode::None => f.write_str("None"),
            ErrorCode::OffsetOutOfRange => f.write_str("The requested offset is not within the range of offsets maintained by the server."),
            ErrorCode::CorruptMessage => f.write_str("This message has failed its CRC checksum, exceeds the valid size, has a null key for a compacted topic, or is otherwise corrupt."),
            ErrorCode::UnknownTopicOrPartition => f.write_str("This server does not host this topic-partition."),
            ErrorCode::InvalidFetchSize => f.write_str("The requested fetch size is invalid."),
            ErrorCode::LeaderNotAvailable => f.write_str("There is no leader for this topic-partition as we are in the middle of a leadership election."),
            ErrorCode::NotLeaderForPartition => f.write_str("This server is not the leader for that topic-partition."),
            ErrorCode::RequestTimedOut => f.write_str("The request timed out."),
            ErrorCode::BrokerNotAvailable => f.write_str("The broker is not available."),
            ErrorCode::ReplicaNotAvailable => f.write_str("The replica is not available for the requested topic-partition."),
            ErrorCode::MessageTooLarge => f.write_str("The request included a message larger than the max message size the server will accept."),
            ErrorCode::StaleControllerEpoch => f.write_str("The controller moved to another broker."),
            ErrorCode::OffsetMetadataTooLarge => f.write_str("The metadata field of the offset request was too large."),
            ErrorCode::NetworkException => f.write_str("The server disconnected before a response was received."),
            ErrorCode::CoordinatorLoadInProgress => f.write_str("The coordinator is loading and hence can't process requests."),
            ErrorCode::CoordinatorNotAvailable => f.write_str("The coordinator is not available."),
            ErrorCode::NotCoordinator => f.write_str("This is not the correct coordinator."),
            ErrorCode::InvalidTopicException => f.write_str("The request attempted to perform an operation on an invalid topic."),
            ErrorCode::RecordListTooLarge => f.write_str("The request included message batch larger than the configured segment size on the server."),
            ErrorCode::NotEnoughReplicas => f.write_str("Messages are rejected since there are fewer in-sync replicas than required."),
            ErrorCode::NotEnoughReplicasAfterAppend => f.write_str("Messages are written to the log, but to fewer in-sync replicas than required."),
            ErrorCode::InvalidRequiredAcks => f.write_str("Produce request specified an invalid value for required acks."),
            ErrorCode::IllegalGeneration => f.write_str("Specified group generation id is not valid."),
            ErrorCode::InconsistentGroupProtocol => f.write_str("The group member's supported protocols are incompatible with those of existing members or first group member tried to join with empty protocol type or empty protocol list."),
            ErrorCode::InvalidGroupId => f.write_str("The configured groupId is invalid."),
            ErrorCode::UnknownMemberId => f.write_str("The coordinator is not aware of this member."),
            ErrorCode::InvalidSessionTimeout => f.write_str("The session timeout is not within the range allowed by the broker (as configured by group.min.session.timeout.ms and group.max.session.timeout.ms)."),
            ErrorCode::RebalanceInProgress => f.write_str("The group is rebalancing, so a rejoin is needed."),
            ErrorCode::InvalidCommitOffsetSize => f.write_str("The committing offset data size is not valid."),
            ErrorCode::TopicAuthorizationFailed => f.write_str("Not authorized to access topics: [Topic authorization failed.]"),
            ErrorCode::GroupAuthorizationFailed => f.write_str("Not authorized to access group: Group authorization failed."),
            ErrorCode::ClusterAuthorizationFailed => f.write_str("Cluster authorization failed."),
            ErrorCode::InvalidTimestamp => f.write_str("The timestamp of the message is out of acceptable range."),
            ErrorCode::UnsupportedSaslMechanism => f.write_str("The broker does not support the requested SASL mechanism."),
            ErrorCode::IllegalSaslState => f.write_str("Request is not valid given the current SASL state."),
            ErrorCode::UnsupportedVersion => f.write_str("The version of API is not supported."),
            ErrorCode::TopicAlreadyExists => f.write_str("Topic with this name already exists."),
            ErrorCode::InvalidPartitions => f.write_str("Number of partitions is below 1."),
            ErrorCode::InvalidReplicationFactor => f.write_str("Replication factor is below 1 or larger than the number of available brokers."),
            ErrorCode::InvalidReplicaAssignment => f.write_str("Replica assignment is invalid."),
            ErrorCode::InvalidConfig => f.write_str("Configuration is invalid."),
            ErrorCode::NotController => f.write_str("This is not the correct controller for this cluster."),
            ErrorCode::InvalidRequest => f.write_str("This most likely occurs because of a request being malformed by the client library or the message was sent to an incompatible broker. See the broker logs for more details."),
            ErrorCode::UnsupportedForMessageFormat => f.write_str("The message format version on the broker does not support the request."),
            ErrorCode::PolicyViolation => f.write_str("Request parameters do not satisfy the configured policy."),
            ErrorCode::OutOfOrderSequenceNumber => f.write_str("The broker received an out of order sequence number."),
            ErrorCode::DuplicateSequenceNumber => f.write_str("The broker received a duplicate sequence number."),
            ErrorCode::InvalidProducerEpoch => f.write_str("Producer attempted an operation with an old epoch. Either there is a newer producer with the same transactionalId, or the producer's transaction has been expired by the broker."),
            ErrorCode::InvalidTxnState => f.write_str("The producer attempted a transactional operation in an invalid state."),
            ErrorCode::InvalidProducerIdMapping => f.write_str("The producer attempted to use a producer id which is not currently assigned to its transactional id."),
            ErrorCode::InvalidTransactionTimeout => f.write_str("The transaction timeout is larger than the maximum value allowed by the broker (as configured by transaction.max.timeout.ms)."),
            ErrorCode::ConcurrentTransactions => f.write_str("The producer attempted to update a transaction while another concurrent operation on the same transaction was ongoing."),
            ErrorCode::TransactionCoordinatorFenced => f.write_str("Indicates that the transaction coordinator sending a WriteTxnMarker is no longer the current coordinator for a given producer."),
            ErrorCode::TransactionalIdAuthorizationFailed => f.write_str("Transactional Id authorization failed."),
            ErrorCode::SecurityDisabled => f.write_str("Security features are disabled."),
            ErrorCode::OperationNotAttempted => f.write_str("The broker did not attempt to execute this operation. This may happen for batched RPCs where some operations in the batch failed, causing the broker to respond without trying the rest."),
            ErrorCode::KafkaStorageError => f.write_str("Disk error when trying to access log file on the disk."),
            ErrorCode::LogDirNotFound => f.write_str("The user-specified log directory is not found in the broker config."),
            ErrorCode::SaslAuthenticationFailed => f.write_str("SASL Authentication failed."),
            ErrorCode::UnknownProducerId => f.write_str("This exception is raised by the broker if it could not locate the producer metadata associated with the producerId in question. This could happen if, for instance, the producer's records were deleted because their retention time had elapsed. Once the last records of the producerId are removed, the producer's metadata is removed from the broker, and future appends by the producer will return this exception."),
            ErrorCode::ReassignmentInProgress => f.write_str("A partition reassignment is in progress."),
            ErrorCode::DelegationTokenAuthDisabled => f.write_str("Delegation Token feature is not enabled."),
            ErrorCode::DelegationTokenNotFound => f.write_str("Delegation Token is not found on server."),
            ErrorCode::DelegationTokenOwnerMismatch => f.write_str("Specified Principal is not valid Owner/Renewer."),
            ErrorCode::DelegationTokenRequestNotAllowed => f.write_str("Delegation Token requests are not allowed on PLAINTEXT/1-way SSL channels and on delegation token authenticated channels."),
            ErrorCode::DelegationTokenAuthorizationFailed => f.write_str("Delegation Token authorization failed."),
            ErrorCode::DelegationTokenExpired => f.write_str("Delegation Token is expired."),
            ErrorCode::InvalidPrincipalType => f.write_str("Supplied principalType is not supported."),
            ErrorCode::NonEmptyGroup => f.write_str("The group is not empty."),
            ErrorCode::GroupIdNotFound => f.write_str("The group id does not exist."),
            ErrorCode::FetchSessionIdNotFound => f.write_str("The fetch session ID was not found."),
            ErrorCode::InvalidFetchSessionEpoch => f.write_str("The fetch session epoch is invalid."),
            ErrorCode::ListenerNotFound => f.write_str("There is no listener on the leader broker that matches the listener on which metadata request was processed."),
            ErrorCode::TopicDeletionDisabled => f.write_str("Topic deletion is disabled."),
            ErrorCode::FencedLeaderEpoch => f.write_str("The leader epoch in the request is older than the epoch on the broker"),
            ErrorCode::UnknownLeaderEpoch => f.write_str("The leader epoch in the request is newer than the epoch on the broker"),
            ErrorCode::UnsupportedCompressionType => f.write_str("The requesting client does not support the compression type of given partition."),
            ErrorCode::StaleBrokerEpoch => f.write_str("Broker epoch has changed"),
            ErrorCode::OffsetNotAvailable => f.write_str("The leader high watermark has not caught up from a recent leader election so the offsets cannot be guaranteed to be monotonically increasing"),
            ErrorCode::MemberIdRequired => f.write_str("The group member needs to have a valid member id before actually entering a consumer group"),
            ErrorCode::PreferredLeaderNotAvailable => f.write_str("The preferred leader was not available"),
            ErrorCode::GroupMaxSizeReached => f.write_str("Consumer group The consumer group has reached its max size. already has the configured maximum number of members."),
            ErrorCode::FencedInstanceId => f.write_str("The broker rejected this static consumer since another consumer with the same group.instance.id has registered with a different member.id."),
            ErrorCode::EligibleLeadersNotAvailable => f.write_str("Eligible topic partition leaders are not available."),
            ErrorCode::ElectionNotNeeded => f.write_str("Leader election not needed for topic partition."),
            ErrorCode::NoReassignmentInProgress => f.write_str("No partition reassignment is in progress."),
            ErrorCode::GroupSubscribedToTopic => f.write_str("Deleting offsets of a topic is forbidden while the consumer group is actively subscribed to it."),
            ErrorCode::InvalidRecord => f.write_str("This record has failed the validation on broker and hence will be rejected."),
            ErrorCode::UnstableOffsetCommit => f.write_str("There are unstable offsets that need to be cleared."),
        }
    }
}
