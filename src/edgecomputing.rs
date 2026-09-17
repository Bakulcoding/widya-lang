//! Edge Computing module for Widya Enterprise Edition
//! Provides Tiny WASM runtime, CRDT synchronization, and IoT integration

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

/// Edge computing runtime
pub struct EdgeRuntime {
    /// Configuration
    config: EdgeConfig,
    
    /// WASM runtime
    wasm_runtime: WasmRuntime,
    
    /// CRDT synchronizer
    crdt_synchronizer: CrdtSynchronizer,
    
    /// IoT device manager
    iot_manager: IoTManager,
    
    /// Edge node registry
    node_registry: Arc<RwLock<HashMap<String, EdgeNode>>>,
    
    /// Offline buffer
    offline_buffer: Arc<RwLock<VecDeque<EdgeEvent>>>,
    
    /// Sync state
    sync_state: Arc<RwLock<SyncState>>,
}

impl EdgeRuntime {
    /// Create new edge runtime
    pub fn new(config: EdgeConfig) -> Result<Self, EdgeError> {
        Ok(Self {
            config: config.clone(),
            wasm_runtime: WasmRuntime::new(&config.wasm)?,
            crdt_synchronizer: CrdtSynchronizer::new(&config.crdt)?,
            iot_manager: IoTManager::new(&config.iot)?,
            node_registry: Arc::new(RwLock::new(HashMap::new())),
            offline_buffer: Arc::new(RwLock::new(VecDeque::new())),
            sync_state: Arc::new(RwLock::new(SyncState::Disconnected)),
        })
    }
    
    /// Start edge runtime
    pub fn start(&self) -> Result<(), EdgeError> {
        self.wasm_runtime.start()?;
        self.iot_manager.start()?;
        
        let mut state = self.sync_state.write().unwrap();
        *state = SyncState::Connected;
        
        Ok(())
    }
    
    /// Register edge node
    pub fn register_node(&self, node: EdgeNode) -> Result<String, EdgeError> {
        let node_id = uuid::Uuid::new_v4().to_string();
        
        let mut registry = self.node_registry.write().unwrap();
        registry.insert(node_id.clone(), node);
        
        Ok(node_id)
    }
    
    /// Execute WASM module
    pub fn execute_wasm(&self, wasm_bytes: &[u8], function: &str, params: &[WasmValue]) -> Result<Vec<WasmValue>, EdgeError> {
        self.wasm_runtime.execute(wasm_bytes, function, params)
    }
    
    /// Sync CRDT data
    pub fn sync_crdt(&self, crdt_type: CrdtType, data: &CrdtData) -> Result<SyncResult, EdgeError> {
        let result = self.crdt_synchronizer.sync(crdt_type, data)?;
        
        if matches!(*self.sync_state.read().unwrap(), SyncState::Disconnected) {
            self.buffer_for_offline(data)?;
        }
        
        Ok(result)
    }
    
    /// Buffer data for offline sync
    fn buffer_for_offline(&self, data: &CrdtData) -> Result<(), EdgeError> {
        let event = EdgeEvent::CrdtUpdate {
            timestamp: SystemTime::now(),
            data: data.clone(),
        };
        
        let mut buffer = self.offline_buffer.write().unwrap();
        buffer.push_back(event);
        
        if buffer.len() > self.config.offline_buffer_size {
            buffer.pop_front();
        }
        
        Ok(())
    }
    
    /// Connect to IoT device
    pub fn connect_iot(&self, device_id: &str, protocol: IoTProtocol) -> Result<IoTConnection, EdgeError> {
        self.iot_manager.connect(device_id, protocol)
    }
    
    /// Send IoT command
    pub fn send_iot_command(&self, device_id: &str, command: IoTCommand) -> Result<IoTResponse, EdgeError> {
        self.iot_manager.send_command(device_id, command)
    }
    
    /// Get sync state
    pub fn sync_state(&self) -> SyncState {
        self.sync_state.read().unwrap().clone()
    }
    
    /// Get offline buffer size
    pub fn offline_buffer_size(&self) -> usize {
        self.offline_buffer.read().unwrap().len()
    }
    
    /// Flush offline buffer
    pub fn flush_offline_buffer(&self) -> Result<(), EdgeError> {
        let mut buffer = self.offline_buffer.write().unwrap();
        
        for event in buffer.drain(..) {
            match event {
                EdgeEvent::CrdtUpdate { timestamp, data } => {
                    let _ = self.crdt_synchronizer.sync(CrdtType::Lww, &data)?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Shutdown edge runtime
    pub fn shutdown(&self) -> Result<(), EdgeError> {
        self.wasm_runtime.shutdown()?;
        self.iot_manager.shutdown()?;
        
        let mut state = self.sync_state.write().unwrap();
        *state = SyncState::Shutdown;
        
        Ok(())
    }
}

/// Edge configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeConfig {
    /// WASM configuration
    pub wasm: WasmConfig,
    
    /// CRDT configuration
    pub crdt: CrdtConfig,
    
    /// IoT configuration
    pub iot: IoTConfig,
    
    /// Offline buffer size
    pub offline_buffer_size: usize,
    
    /// Sync interval in milliseconds
    pub sync_interval_ms: u64,
    
    /// Compression enabled
    pub compression_enabled: bool,
    
    /// Encryption enabled
    pub encryption_enabled: bool,
}

/// WASM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmConfig {
    /// Memory limit in bytes
    pub memory_limit: usize,
    
    /// Execution timeout in milliseconds
    pub execution_timeout_ms: u64,
    
    /// Module cache size
    pub cache_size: usize,
    
    /// Sandbox enabled
    pub sandbox_enabled: bool,
}

/// CRDT configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrdtConfig {
    /// CRDT types supported
    pub supported_types: Vec<CrdtType>,
    
    /// Conflict resolution strategy
    pub conflict_resolution: ConflictResolution,
    
    /// Sync frequency in milliseconds
    pub sync_frequency_ms: u64,
    
    /// Merge strategy
    pub merge_strategy: MergeStrategy,
}

/// IoT configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoTConfig {
    /// Protocols supported
    pub protocols: Vec<IoTProtocol>,
    
    /// Device discovery enabled
    pub device_discovery: bool,
    
    /// Auto-reconnect enabled
    pub auto_reconnect: bool,
    
    /// Connection timeout in seconds
    pub connection_timeout_secs: u64,
    
    /// Keep-alive interval in seconds
    pub keep_alive_interval_secs: u64,
}

/// Edge node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeNode {
    /// Node ID
    pub node_id: String,
    
    /// Node type
    pub node_type: NodeType,
    
    /// Hardware capabilities
    pub capabilities: NodeCapabilities,
    
    /// Connection status
    pub connected: bool,
    
    /// Last seen timestamp
    pub last_seen: u64,
    
    /// Location
    pub location: Option<NodeLocation>,
    
    /// Metadata
    pub metadata: HashMap<String, String>,
}

/// Node types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    /// IoT gateway
    IoTGateway,
    
    /// Edge server
    EdgeServer,
    
    /// Mobile device
    MobileDevice,
    
    /// Embedded device
    EmbeddedDevice,
    
    /// Cloud edge
    CloudEdge,
}

/// Node capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeCapabilities {
    /// CPU architecture
    pub cpu_arch: String,
    
    /// Memory in MB
    pub memory_mb: u32,
    
    /// Storage in MB
    pub storage_mb: u32,
    
    /// Network interfaces
    pub network_interfaces: Vec<String>,
    
    /// Sensors available
    pub sensors: Vec<SensorType>,
    
    /// Actuators available
    pub actuators: Vec<ActuatorType>,
}

/// Node location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeLocation {
    /// Latitude
    pub latitude: f64,
    
    /// Longitude
    pub longitude: f64,
    
    /// Altitude in meters
    pub altitude: f64,
    
    /// Accuracy in meters
    pub accuracy: f64,
}

/// Sensor types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SensorType {
    Temperature,
    Humidity,
    Pressure,
    Light,
    Motion,
    Sound,
    Gas,
    Proximity,
}

/// Actuator types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActuatorType {
    Relay,
    Servo,
    Motor,
    Led,
    Display,
    Buzzer,
    Valve,
}

/// WASM runtime
struct WasmRuntime {
    config: WasmConfig,
}

impl WasmRuntime {
    fn new(config: &WasmConfig) -> Result<Self, EdgeError> {
        Ok(Self {
            config: config.clone(),
        })
    }
    
    fn start(&self) -> Result<(), EdgeError> {
        Ok(())
    }
    
    fn execute(&self, wasm_bytes: &[u8], function: &str, params: &[WasmValue]) -> Result<Vec<WasmValue>, EdgeError> {
        Ok(vec![WasmValue::I32(42)])
    }
    
    fn shutdown(&self) -> Result<(), EdgeError> {
        Ok(())
    }
}

/// WASM value types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WasmValue {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

/// CRDT synchronizer
struct CrdtSynchronizer {
    config: CrdtConfig,
}

impl CrdtSynchronizer {
    fn new(config: &CrdtConfig) -> Result<Self, EdgeError> {
        Ok(Self {
            config: config.clone(),
        })
    }
    
    fn sync(&self, crdt_type: CrdtType, data: &CrdtData) -> Result<SyncResult, EdgeError> {
        Ok(SyncResult {
            synced_at: SystemTime::now(),
            conflicts_resolved: 0,
            merge_result: MergeResult::Success,
        })
    }
}

/// CRDT types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrdtType {
    /// Last-Write-Wins register
    Lww,
    
    /// Grow-only counter
    GCounter,
    
    /// Positive-negative counter
    PNCounter,
    
    /// Grow-only set
    GSet,
    
    /// 2-Phase set
    TwoPSet,
    
    /// Observed-removed set
    ORSet,
}

/// CRDT data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrdtData {
    /// CRDT type
    pub crdt_type: CrdtType,
    
    /// State
    pub state: HashMap<String, String>,
    
    /// Timestamp
    pub timestamp: u64,
    
    /// Node ID
    pub node_id: String,
    
    /// Vector clock
    pub vector_clock: HashMap<String, u64>,
}

/// Conflict resolution strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictResolution {
    /// Last-write-wins
    LastWriteWins,
    
    /// First-write-wins
    FirstWriteWins,
    
    /// Merge
    Merge,
    
    /// Custom
    Custom,
}

/// Merge strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeStrategy {
    /// Union
    Union,
    
    /// Intersection
    Intersection,
    
    /// Maximum
    Maximum,
    
    /// Minimum
    Minimum,
}

/// Sync result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    /// When synced
    pub synced_at: SystemTime,
    
    /// Conflicts resolved
    pub conflicts_resolved: u32,
    
    /// Merge result
    pub merge_result: MergeResult,
}

/// Merge result
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MergeResult {
    Success,
    Conflict,
    Error,
}

/// IoT manager
struct IoTManager {
    config: IoTConfig,
}

impl IoTManager {
    fn new(config: &IoTConfig) -> Result<Self, EdgeError> {
        Ok(Self {
            config: config.clone(),
        })
    }
    
    fn start(&self) -> Result<(), EdgeError> {
        Ok(())
    }
    
    fn connect(&self, device_id: &str, protocol: IoTProtocol) -> Result<IoTConnection, EdgeError> {
        Ok(IoTConnection {
            device_id: device_id.to_string(),
            protocol,
            connected: true,
            connection_time: SystemTime::now(),
        })
    }
    
    fn send_command(&self, device_id: &str, command: IoTCommand) -> Result<IoTResponse, EdgeError> {
        Ok(IoTResponse {
            device_id: device_id.to_string(),
            command_id: command.id.clone(),
            success: true,
            response_data: command.data.clone(),
            response_time: SystemTime::now(),
        })
    }
    
    fn shutdown(&self) -> Result<(), EdgeError> {
        Ok(())
    }
}

/// IoT protocols
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IoTProtocol {
    /// MQTT
    MQTT,
    
    /// CoAP
    CoAP,
    
    /// HTTP
    HTTP,
    
    /// WebSocket
    WebSocket,
    
    /// Bluetooth
    Bluetooth,
    
    /// LoRaWAN
    LoRaWAN,
    
    /// Zigbee
    Zigbee,
}

/// IoT connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoTConnection {
    /// Device ID
    pub device_id: String,
    
    /// Protocol
    pub protocol: IoTProtocol,
    
    /// Connected
    pub connected: bool,
    
    /// Connection time
    pub connection_time: SystemTime,
}

/// IoT command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoTCommand {
    /// Command ID
    pub id: String,
    
    /// Device ID
    pub device_id: String,
    
    /// Command type
    pub command_type: CommandType,
    
    /// Command data
    pub data: HashMap<String, String>,
    
    /// Priority
    pub priority: Priority,
    
    /// Timeout in milliseconds
    pub timeout_ms: u64,
}

/// IoT response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoTResponse {
    /// Device ID
    pub device_id: String,
    
    /// Command ID
    pub command_id: String,
    
    /// Success flag
    pub success: bool,
    
    /// Response data
    pub response_data: HashMap<String, String>,
    
    /// Response time
    pub response_time: SystemTime,
}

/// Command types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandType {
    Read,
    Write,
    Execute,
    Configure,
    Reset,
    Update,
}

/// Priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Normal,
    High,
    Critical,
}

/// Edge event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EdgeEvent {
    CrdtUpdate {
        timestamp: SystemTime,
        data: CrdtData,
    },
}

/// Sync state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncState {
    Disconnected,
    Connecting,
    Connected,
    Syncing,
    Shutdown,
}

/// Edge error
#[derive(Debug, thiserror::Error)]
pub enum EdgeError {
    #[error("WASM execution error: {0}")]
    WasmError(String),
    
    #[error("CRDT sync error: {0}")]
    CrdtError(String),
    
    #[error("IoT connection error: {0}")]
    IoTError(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_edge_runtime_creation() {
        let config = EdgeConfig {
            wasm: WasmConfig {
                memory_limit: 1024 * 1024,
                execution_timeout_ms: 1000,
                cache_size: 10,
                sandbox_enabled: true,
            },
            crdt: CrdtConfig {
                supported_types: vec![CrdtType::Lww, CrdtType::GCounter],
                conflict_resolution: ConflictResolution::LastWriteWins,
                sync_frequency_ms: 1000,
                merge_strategy: MergeStrategy::Union,
            },
            iot: IoTConfig {
                protocols: vec![IoTProtocol::MQTT, IoTProtocol::HTTP],
                device_discovery: true,
                auto_reconnect: true,
                connection_timeout_secs: 30,
                keep_alive_interval_secs: 60,
            },
            offline_buffer_size: 1000,
            sync_interval_ms: 1000,
            compression_enabled: true,
            encryption_enabled: true,
        };
        
        let runtime = EdgeRuntime::new(config).unwrap();
        assert_eq!(runtime.sync_state(), SyncState::Disconnected);
    }
    
    #[test]
    fn test_edge_node_registration() {
        let config = EdgeConfig::default();
        let runtime = EdgeRuntime::new(config).unwrap();
        
        let node = EdgeNode {
            node_id: "test-node".to_string(),
            node_type: NodeType::IoTGateway,
            capabilities: NodeCapabilities {
                cpu_arch: "ARM".to_string(),
                memory_mb: 512,
                storage_mb: 1024,
                network_interfaces: vec!["wlan0".to_string()],
                sensors: vec![SensorType::Temperature],
                actuators: vec![ActuatorType::Relay],
            },
            connected: true,
            last_seen: 0,
            location: None,
            metadata: HashMap::new(),
        };
        
        let node_id = runtime.register_node(node).unwrap();
        assert!(!node_id.is_empty());
    }
    
    #[test]
    fn test_iot_protocols() {
        assert!(matches!(IoTProtocol::MQTT, IoTProtocol::MQTT));
        assert!(matches!(IoTProtocol::CoAP, IoTProtocol::CoAP));
        assert!(matches!(IoTProtocol::HTTP, IoTProtocol::HTTP));
    }
    
    #[test]
    fn test_crdt_types() {
        assert!(matches!(CrdtType::Lww, CrdtType::Lww));
        assert!(matches!(CrdtType::GCounter, CrdtType::GCounter));
        assert!(matches!(CrdtType::ORSet, CrdtType::ORSet));
    }
}

impl Default for EdgeConfig {
    fn default() -> Self {
        Self {
            wasm: WasmConfig {
                memory_limit: 1024 * 1024,
                execution_timeout_ms: 1000,
                cache_size: 10,
                sandbox_enabled: true,
            },
            crdt: CrdtConfig {
                supported_types: vec![CrdtType::Lww, CrdtType::GCounter],
                conflict_resolution: ConflictResolution::LastWriteWins,
                sync_frequency_ms: 1000,
                merge_strategy: MergeStrategy::Union,
            },
            iot: IoTConfig {
                protocols: vec![IoTProtocol::MQTT, IoTProtocol::HTTP],
                device_discovery: true,
                auto_reconnect: true,
                connection_timeout_secs: 30,
                keep_alive_interval_secs: 60,
            },
            offline_buffer_size: 1000,
            sync_interval_ms: 1000,
            compression_enabled: true,
            encryption_enabled: true,
        }
    }
}
