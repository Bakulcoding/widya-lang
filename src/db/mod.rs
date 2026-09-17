// ============================================================================
// DB Module Library
// ============================================================================
// Database storage layer untuk Widya OS development
// ============================================================================

pub mod wal;
pub mod btree;
pub mod query;
pub mod sharding;
pub mod consensus;
pub mod replication;

pub use wal::{WriteAheadLog, LogRecord, LogRecordType, LogSequenceNumber, TransactionID, PageID};
pub use btree::{BPlusTree, BPlusTreeIterator, BPlusNode, NodeType, Key, Value, PageID};
pub use query::parser::{Tokenizer, Query, SelectQuery, InsertQuery, UpdateQuery, DeleteQuery, Column, ColumnDef, DataType, ColumnConstraint, Constraint, ConstraintKind, FromClause, JoinType, OrderByClause, OrderDirection, Assignment, Expr, Literal, UnaryOp, BinaryOp, Parser};
pub use query::optimizer::{QueryOptimizer, CostModel, TableStats, ColumnStats, PlanNode};
pub use query::execution::{ExecutionEngine, ExecutionResult, Value, Row};
pub use sharding::{ShardManager, ConsistentHashRing, VirtualNode, ShardMetadata, ShardConfig, ShardID, NodeID, HashKey, ClusterHealth, RouteInfo, NodeStatus};
pub use consensus::{RaftNode, RaftState, LogEntry, VoteRequest, VoteResponse, AppendEntriesRequest, AppendEntriesResponse, NodeConfig, ConsensusAlgorithm, DistributedCluster, ClusterConfig, Proposer, AcceptorState, Learner, ClusterHealth as ConsensusHealth, Term, LogIndex};
pub use replication::{ReplicationManager, ReplicationMode, ConflictResolution, ReplicaStatus, VectorClock, ConflictRecord, Resolution, ReplicationError, ReplicationMetrics, ClusterHealth as ReplicationClusterHealth};
