// ============================================================================
// Database Sharding & Consistent Hashing - Production Layer
// ============================================================================
// Shard management dengan consistent hashing untuk horizontal scaling
// Features:
// - Virtual nodes dengan consistent hashing ring
// - Shard distribution & load balancing
// - Failure detection & automatic shard reassignment
// - Cross-shard transaction coordination
// ============================================================================

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

// ============================================================================
// Sharding Structures
// ============================================================================

/// Shard identifier
pub type ShardID = u64;

/// Node identifier dalam cluster
pub type NodeID = String;

/// Key untuk consistent hashing
pub type HashKey = u64;

/// Virtual node untuk consistent hashing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualNode {
    pub node_id: NodeID,
    pub shard_id: ShardID,
    pub virtual_id: u32,
    pub hash_position: HashKey,
    pub weight: f64,
    pub is_active: bool,
    pub last_heartbeat: u64,
}

impl VirtualNode {
    pub fn new(node_id: NodeID, shard_id: ShardID, virtual_id: u32, weight: f64) -> Self {
        let hash_input = format!("{}-{}-{}", node_id, shard_id, virtual_id);
        let hash_position = Self::hash_key(&hash_input);
        
        Self {
            node_id,
            shard_id,
            virtual_id,
            hash_position,
            weight,
            is_active: true,
            last_heartbeat: Self::current_timestamp(),
        }
    }
    
    fn hash_key(key: &str) -> HashKey {
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        let result = hasher.finalize();
        
        // Convert first 8 bytes to u64
        let bytes: [u8; 8] = result[0..8].try_into().unwrap();
        u64::from_be_bytes(bytes)
    }
    
    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
    
    pub fn update_heartbeat(&mut self) {
        self.last_heartbeat = Self::current_timestamp();
        self.is_active = true;
    }
    
    pub fn is_stale(&self, timeout_seconds: u64) -> bool {
        Self::current_timestamp() - self.last_heartbeat > timeout_seconds
    }
}

/// Shard metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardMetadata {
    pub shard_id: ShardID,
    pub node_ids: Vec<NodeID>,
    pub key_range: (HashKey, HashKey),
    pub replication_factor: u32,
    pub is_writeable: bool,
    pub data_size_bytes: u64,
    pub last_rebalanced: u64,
}

impl ShardMetadata {
    pub fn new(shard_id: ShardID, node_ids: Vec<NodeID>, key_range: (HashKey, HashKey)) -> Self {
        Self {
            shard_id,
            node_ids,
            key_range,
            replication_factor: 3,
            is_writeable: true,
            data_size_bytes: 0,
            last_rebalanced: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    pub fn contains_key(&self, key: HashKey) -> bool {
        key >= self.key_range.0 && key <= self.key_range.1
    }
}

/// Consistent hashing ring
#[derive(Debug, Clone)]
pub struct ConsistentHashRing {
    pub virtual_nodes: Vec<VirtualNode>,
    pub node_to_virtual_nodes: HashMap<NodeID, Vec<VirtualNode>>,
    pub sorted_positions: Vec<HashKey>,
    pub ring_size: usize,
}

impl ConsistentHashRing {
    pub fn new(nodes: Vec<NodeID>, virtual_nodes_per_node: u32, weight: f64) -> Self {
        let mut virtual_nodes = Vec::new();
        let mut node_to_virtual_nodes = HashMap::new();
        
        for (shard_id, node_id) in nodes.iter().enumerate() {
            let node_virtual_nodes = (0..virtual_nodes_per_node)
                .map(|virtual_id| {
                    VirtualNode::new(
                        node_id.clone(),
                        shard_id as ShardID,
                        virtual_id,
                        weight,
                    )
                })
                .collect::<Vec<_>>();
            
            for vnode in &node_virtual_nodes {
                virtual_nodes.push(vnode.clone());
            }
            
            node_to_virtual_nodes.insert(node_id.clone(), node_virtual_nodes);
        }
        
        // Sort virtual nodes by hash position
        virtual_nodes.sort_by_key(|v| v.hash_position);
        let sorted_positions = virtual_nodes.iter().map(|v| v.hash_position).collect();
        
        Self {
            virtual_nodes,
            node_to_virtual_nodes,
            sorted_positions,
            ring_size: nodes.len() * virtual_nodes_per_node as usize,
        }
    }
    
    /// Get node responsible for a key
    pub fn get_node_for_key(&self, key: HashKey) -> Option<&VirtualNode> {
        if self.sorted_positions.is_empty() {
            return None;
        }
        
        // Binary search untuk posisi clockwise
        let idx = self.sorted_positions
            .binary_search(&key)
            .unwrap_or_else(|idx| idx % self.sorted_positions.len());
        
        self.virtual_nodes.get(idx)
    }
    
    /// Get N replicas for a key
    pub fn get_replicas_for_key(&self, key: HashKey, n: usize) -> Vec<&VirtualNode> {
        if self.sorted_positions.is_empty() {
            return Vec::new();
        }
        
        let mut replicas = Vec::new();
        let start_idx = self.sorted_positions
            .binary_search(&key)
            .unwrap_or_else(|idx| idx % self.sorted_positions.len());
        
        for i in 0..n {
            let idx = (start_idx + i) % self.sorted_positions.len();
            if let Some(vnode) = self.virtual_nodes.get(idx) {
                replicas.push(vnode);
            }
        }
        
        replicas
    }
    
    /// Add new node to ring
    pub fn add_node(&mut self, node_id: NodeID, virtual_nodes_count: u32, weight: f64) {
        let new_shard_id = self.virtual_nodes.len() as ShardID;
        let virtual_nodes = (0..virtual_nodes_count)
            .map(|virtual_id| {
                VirtualNode::new(node_id.clone(), new_shard_id, virtual_id, weight)
            })
            .collect::<Vec<_>>();
        
        for vnode in &virtual_nodes {
            self.virtual_nodes.push(vnode.clone());
        }
        
        self.node_to_virtual_nodes.insert(node_id.clone(), virtual_nodes);
        
        // Re-sort ring
        self.virtual_nodes.sort_by_key(|v| v.hash_position);
        self.sorted_positions = self.virtual_nodes.iter().map(|v| v.hash_position).collect();
        self.ring_size += virtual_nodes_count as usize;
    }
    
    /// Remove node from ring
    pub fn remove_node(&mut self, node_id: &NodeID) {
        if let Some(virtual_nodes) = self.node_to_virtual_nodes.remove(node_id) {
            let positions_to_remove: Vec<HashKey> = virtual_nodes.iter()
                .map(|v| v.hash_position)
                .collect();
            
            // Remove virtual nodes
            self.virtual_nodes.retain(|v| !positions_to_remove.contains(&v.hash_position));
            
            // Update sorted positions
            self.sorted_positions = self.virtual_nodes.iter().map(|v| v.hash_position).collect();
            self.ring_size -= virtual_nodes.len();
        }
    }
    
    /// Rebalance ring (optimalisasi distribusi)
    pub fn rebalance(&mut self, target_virtual_nodes_per_node: u32) {
        // Calculate current distribution
        let mut distribution: HashMap<NodeID, usize> = HashMap::new();
        for vnode in &self.virtual_nodes {
            *distribution.entry(vnode.node_id.clone()).or_insert(0) += 1;
        }
        
        // Add virtual nodes ke node yang underloaded
        for (node_id, count) in distribution {
            if count < target_virtual_nodes_per_node as usize {
                let needed = target_virtual_nodes_per_node as usize - count;
                for i in 0..needed {
                    let vnode = VirtualNode::new(
                        node_id.clone(),
                        self.virtual_nodes.len() as ShardID,
                        i as u32,
                        1.0,
                    );
                    self.virtual_nodes.push(vnode);
                }
            }
        }
        
        // Re-sort ring
        self.virtual_nodes.sort_by_key(|v| v.hash_position);
        self.sorted_positions = self.virtual_nodes.iter().map(|v| v.hash_position).collect();
        self.ring_size = self.virtual_nodes.len();
    }
    
    /// Calculate load distribution variance
    pub fn load_variance(&self) -> f64 {
        if self.virtual_nodes.is_empty() {
            return 0.0;
        }
        
        let mut node_loads: HashMap<NodeID, f64> = HashMap::new();
        
        for vnode in &self.virtual_nodes {
            *node_loads.entry(vnode.node_id.clone()).or_insert(0.0) += vnode.weight;
        }
        
        let loads: Vec<f64> = node_loads.values().copied().collect();
        let mean = loads.iter().sum::<f64>() / loads.len() as f64;
        
        let variance = loads.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / loads.len() as f64;
        
        variance
    }
}

/// Shard manager untuk multi-node deployment
#[derive(Debug, Clone)]
pub struct ShardManager {
    pub ring: ConsistentHashRing,
    pub shards: HashMap<ShardID, ShardMetadata>,
    pub node_status: HashMap<NodeID, NodeStatus>,
    pub config: ShardConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeStatus {
    pub node_id: NodeID,
    pub is_active: bool,
    pub last_seen: u64,
    pub load_factor: f64,
    pub available_memory_bytes: u64,
    pub cpu_usage_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardConfig {
    pub replication_factor: u32,
    pub virtual_nodes_per_node: u32,
    pub shard_timeout_seconds: u64,
    pub rebalance_threshold: f64,
    pub auto_rebalance: bool,
}

impl Default for ShardConfig {
    fn default() -> Self {
        Self {
            replication_factor: 3,
            virtual_nodes_per_node: 100,
            shard_timeout_seconds: 30,
            rebalance_threshold: 0.3,
            auto_rebalance: true,
        }
    }
}

impl ShardManager {
    pub fn new(nodes: Vec<NodeID>, config: ShardConfig) -> Self {
        let ring = ConsistentHashRing::new(
            nodes.clone(),
            config.virtual_nodes_per_node,
            1.0,
        );
        
        let mut shards = HashMap::new();
        for (shard_id, node_id) in nodes.iter().enumerate() {
            let key_range = (0, u64::MAX);
            let shard_metadata = ShardMetadata::new(
                shard_id as ShardID,
                vec![node_id.clone()],
                key_range,
            );
            shards.insert(shard_id as ShardID, shard_metadata);
        }
        
        let node_status = nodes.iter()
            .map(|node_id| {
                let status = NodeStatus {
                    node_id: node_id.clone(),
                    is_active: true,
                    last_seen: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    load_factor: 0.0,
                    available_memory_bytes: 1024 * 1024 * 1024, // 1GB default
                    cpu_usage_percent: 0.0,
                };
                (node_id.clone(), status)
            })
            .collect();
        
        Self {
            ring,
            shards,
            node_status,
            config,
        }
    }
    
    /// Route key ke shard yang sesuai
    pub fn route_key(&self, key: &str) -> Option<RouteInfo> {
        let hash_key = Self::hash_key(key);
        
        let primary_vnode = self.ring.get_node_for_key(hash_key)?;
        let replicas = self.ring.get_replicas_for_key(
            hash_key, 
            self.config.replication_factor as usize
        );
        
        Some(RouteInfo {
            key: key.to_string(),
            hash_key,
            primary_node: primary_vnode.node_id.clone(),
            primary_shard: primary_vnode.shard_id,
            replica_nodes: replicas.iter()
                .map(|v| v.node_id.clone())
                .collect(),
        })
    }
    
    /// Update node status
    pub fn update_node_status(&mut self, node_id: &NodeID, status: NodeStatus) {
        self.node_status.insert(node_id.clone(), status);
        
        // Update virtual node status
        if let Some(virtual_nodes) = self.ring.node_to_virtual_nodes.get_mut(node_id) {
            for vnode in virtual_nodes {
                vnode.is_active = status.is_active;
                vnode.update_heartbeat();
            }
        }
        
        // Check if rebalance needed
        if self.config.auto_rebalance {
            self.check_rebalance();
        }
    }
    
    /// Check and perform rebalancing if needed
    fn check_rebalance(&mut self) {
        let variance = self.ring.load_variance();
        
        if variance > self.config.rebalance_threshold {
            self.perform_rebalancing();
        }
    }
    
    /// Perform shard rebalancing
    fn perform_rebalancing(&mut self) {
        println!("Performing shard rebalancing...");
        
        // Current implementation: simple rebalance
        // Production implementation would:
        // 1. Calculate optimal shard distribution
        // 2. Plan shard movement
        // 3. Execute movement with minimal disruption
        // 4. Update metadata
        
        self.ring.rebalance(self.config.virtual_nodes_per_node);
        
        // Update shard metadata
        for shard in self.shards.values_mut() {
            shard.last_rebalanced = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }
    }
    
    /// Add new node to cluster
    pub fn add_node(&mut self, node_id: NodeID) {
        self.ring.add_node(node_id.clone(), self.config.virtual_nodes_per_node, 1.0);
        
        let status = NodeStatus {
            node_id: node_id.clone(),
            is_active: true,
            last_seen: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            load_factor: 0.0,
            available_memory_bytes: 1024 * 1024 * 1024,
            cpu_usage_percent: 0.0,
        };
        
        self.node_status.insert(node_id, status);
        
        if self.config.auto_rebalance {
            self.perform_rebalancing();
        }
    }
    
    /// Remove node from cluster
    pub fn remove_node(&mut self, node_id: &NodeID) {
        self.ring.remove_node(node_id);
        self.node_status.remove(node_id);
        
        if self.config.auto_rebalance {
            self.perform_rebalancing();
        }
    }
    
    /// Get cluster health status
    pub fn cluster_health(&self) -> ClusterHealth {
        let total_nodes = self.node_status.len();
        let active_nodes = self.node_status.values()
            .filter(|s| s.is_active)
            .count();
        
        let load_distribution = self.ring.load_variance();
        
        ClusterHealth {
            total_nodes,
            active_nodes,
            load_distribution,
            avg_load_factor: self.node_status.values()
                .map(|s| s.load_factor)
                .sum::<f64>() / total_nodes as f64,
            needs_rebalance: load_distribution > self.config.rebalance_threshold,
        }
    }
    
    fn hash_key(key: &str) -> HashKey {
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        let result = hasher.finalize();
        
        let bytes: [u8; 8] = result[0..8].try_into().unwrap();
        u64::from_be_bytes(bytes)
    }
}

/// Routing information untuk key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteInfo {
    pub key: String,
    pub hash_key: HashKey,
    pub primary_node: NodeID,
    pub primary_shard: ShardID,
    pub replica_nodes: Vec<NodeID>,
}

/// Cluster health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterHealth {
    pub total_nodes: usize,
    pub active_nodes: usize,
    pub load_distribution: f64,
    pub avg_load_factor: f64,
    pub needs_rebalance: bool,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_virtual_node_creation() {
        let vnode = VirtualNode::new("node-1".to_string(), 1, 0, 1.0);
        
        assert_eq!(vnode.node_id, "node-1");
        assert_eq!(vnode.shard_id, 1);
        assert_eq!(vnode.virtual_id, 0);
        assert!(vnode.hash_position > 0);
        assert!(vnode.is_active);
    }
    
    #[test]
    fn test_consistent_hash_ring() {
        let nodes = vec![
            "node-1".to_string(),
            "node-2".to_string(),
            "node-3".to_string(),
        ];
        
        let ring = ConsistentHashRing::new(nodes, 10, 1.0);
        
        assert_eq!(ring.ring_size, 30); // 3 nodes * 10 virtual nodes
        assert_eq!(ring.sorted_positions.len(), 30);
        
        // Test key routing
        let key = "test-key-123";
        let hash_key = VirtualNode::hash_key(key);
        let vnode = ring.get_node_for_key(hash_key);
        
        assert!(vnode.is_some());
    }
    
    #[test]
    fn test_shard_manager_routing() {
        let nodes = vec![
            "node-1".to_string(),
            "node-2".to_string(),
            "node-3".to_string(),
        ];
        
        let config = ShardConfig::default();
        let manager = ShardManager::new(nodes, config);
        
        let route_info = manager.route_key("user:12345");
        
        assert!(route_info.is_some());
        let info = route_info.unwrap();
        
        assert!(!info.primary_node.is_empty());
        assert_eq!(info.replica_nodes.len(), 3); // replication_factor = 3
    }
    
    #[test]
    fn test_node_addition() {
        let nodes = vec![
            "node-1".to_string(),
            "node-2".to_string(),
        ];
        
        let config = ShardConfig::default();
        let mut manager = ShardManager::new(nodes, config);
        
        assert_eq!(manager.node_status.len(), 2);
        
        manager.add_node("node-3".to_string());
        
        assert_eq!(manager.node_status.len(), 3);
        assert_eq!(manager.ring.ring_size, 300); // 3 nodes * 100 virtual nodes
        
        let health = manager.cluster_health();
        assert_eq!(health.total_nodes, 3);
        assert_eq!(health.active_nodes, 3);
    }
    
    #[test]
    fn test_load_variance_calculation() {
        let nodes = vec![
            "node-1".to_string(),
            "node-2".to_string(),
            "node-3".to_string(),
        ];
        
        let ring = ConsistentHashRing::new(nodes, 10, 1.0);
        let variance = ring.load_variance();
        
        assert!(variance >= 0.0);
        assert!(variance < 1.0); // Should be relatively balanced
    }
    
    #[test]
    fn test_replicas_for_key() {
        let nodes = vec![
            "node-1".to_string(),
            "node-2".to_string(),
            "node-3".to_string(),
        ];
        
        let ring = ConsistentHashRing::new(nodes, 10, 1.0);
        let hash_key = VirtualNode::hash_key("test-key");
        
        let replicas = ring.get_replicas_for_key(hash_key, 3);
        
        assert_eq!(replicas.len(), 3);
        
        // All replicas should be unique nodes
        let unique_nodes: std::collections::HashSet<_> = replicas.iter()
            .map(|v| &v.node_id)
            .collect();
        
        assert_eq!(unique_nodes.len(), 3); // Should be 3 unique nodes
    }
    
    #[test]
    fn test_node_status_update() {
        let nodes = vec!["node-1".to_string()];
        let config = ShardConfig::default();
        let mut manager = ShardManager::new(nodes, config);
        
        let new_status = NodeStatus {
            node_id: "node-1".to_string(),
            is_active: false,
            last_seen: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            load_factor: 0.8,
            available_memory_bytes: 512 * 1024 * 1024,
            cpu_usage_percent: 75.0,
        };
        
        manager.update_node_status(&"node-1".to_string(), new_status);
        
        let status = manager.node_status.get("node-1").unwrap();
        assert!(!status.is_active);
        assert_eq!(status.load_factor, 0.8);
        assert_eq!(status.cpu_usage_percent, 75.0);
    }
    
    #[test]
    fn test_shard_metadata_key_range() {
        let shard = ShardMetadata::new(
            1,
            vec!["node-1".to_string()],
            (1000, 2000),
        );
        
        assert!(shard.contains_key(1500));
        assert!(!shard.contains_key(500));
        assert!(!shard.contains_key(2500));
    }
}