#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct ApiKey(pub i16);
impl ApiKey {
    pub const PRODUCE: ApiKey = ApiKey(0);
    pub const FETCH: ApiKey = ApiKey(1);
    pub const LIST_OFFSETS: ApiKey = ApiKey(2);
    pub const METADATA: ApiKey = ApiKey(3);
    pub const LEADER_AND_ISR: ApiKey = ApiKey(4);
    pub const STOP_REPLICA: ApiKey = ApiKey(5);
    pub const UPDATE_METADATA: ApiKey = ApiKey(6);
    pub const CONTROLLED_SHUTDOWN: ApiKey = ApiKey(7);
    pub const OFFSET_COMMIT: ApiKey = ApiKey(8);
    pub const OFFSET_FETCH: ApiKey = ApiKey(9);
    pub const FIND_COORDINATOR: ApiKey = ApiKey(10);
    pub const JOIN_GROUP: ApiKey = ApiKey(11);
    pub const HEARTBEAT: ApiKey = ApiKey(12);
    pub const LEAVE_GROUP: ApiKey = ApiKey(13);
    pub const SYNC_GROUP: ApiKey = ApiKey(14);
    pub const DESCRIBE_GROUPS: ApiKey = ApiKey(15);
    pub const LIST_GROUPS: ApiKey = ApiKey(16);
    pub const SASL_HANDSHAKE: ApiKey = ApiKey(17);
    pub const API_VERSIONS: ApiKey = ApiKey(18);
    pub const CREATE_TOPICS: ApiKey = ApiKey(19);
    pub const DELETE_TOPICS: ApiKey = ApiKey(20);
    pub const DELETE_RECORDS: ApiKey = ApiKey(21);
    pub const INIT_PRODUCER_ID: ApiKey = ApiKey(22);
    pub const OFFSET_FOR_LEADER_EPOCH: ApiKey = ApiKey(23);
    pub const ADD_PARTITIONS_TO_TXN: ApiKey = ApiKey(24);
    pub const ADD_OFFSETS_TO_TXN: ApiKey = ApiKey(25);
    pub const END_TXN: ApiKey = ApiKey(26);
    pub const WRITE_TXN_MARKERS: ApiKey = ApiKey(27);
    pub const TXN_OFFSET_COMMIT: ApiKey = ApiKey(28);
    pub const DESCRIBE_ACLS: ApiKey = ApiKey(29);
    pub const CREATE_ACLS: ApiKey = ApiKey(30);
    pub const DELETE_ACLS: ApiKey = ApiKey(31);
    pub const DESCRIBE_CONFIGS: ApiKey = ApiKey(32);
    pub const ALTER_CONFIGS: ApiKey = ApiKey(33);
    pub const ALTER_REPLICA_LOG_DIRS: ApiKey = ApiKey(34);
    pub const DESCRIBE_LOG_DIRS: ApiKey = ApiKey(35);
    pub const SASL_AUTHENTICATE: ApiKey = ApiKey(36);
    pub const CREATE_PARTITIONS: ApiKey = ApiKey(37);
    pub const CREATE_DELEGATION_TOKEN: ApiKey = ApiKey(38);
    pub const RENEW_DELEGATION_TOKEN: ApiKey = ApiKey(39);
    pub const EXPIRE_DELEGATION_TOKEN: ApiKey = ApiKey(40);
    pub const DESCRIBE_DELEGATION_TOKEN: ApiKey = ApiKey(41);
    pub const DELETE_GROUPS: ApiKey = ApiKey(42);
    pub const ELECT_PREFERRED_LEADERS: ApiKey = ApiKey(43);
    pub const INCREMENTAL_ALTER_CONFIGS: ApiKey = ApiKey(44);
}
impl From<i16> for ApiKey {
    fn from(i: i16) -> Self {
        ApiKey(i)
    }
}
