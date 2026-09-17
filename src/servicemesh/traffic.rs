//! Traffic management module for Widya service mesh
//! Provides circuit breaker, rate limiting, and traffic routing

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crate::servicemesh::{ServiceMeshError, Result, LoadBalancingStrategy, CircuitBreakerConfig};

/// Traffic manager for routing and load balancing
pub struct TrafficManager {
    /// Circuit breakers per service
    circuit_breakers: Arc<RwLock<HashMap<String, CircuitBreaker>>>,
    
    /// Rate limiters per tenant
    rate_limiters: Arc<RwLock<HashMap<String, RateLimiter>>>,
    
    /// Load balancing strategy
    load_balancing: LoadBalancingStrategy,
    
    /// Endpoint weights for weighted load balancing
    endpoint_weights: Arc<RwLock<HashMap<String, HashMap<String, u32>>>>,
}

impl TrafficManager {
    /// Create new traffic manager
    pub fn new() -> Self {
        Self {
            circuit_breakers: Arc::new(RwLock::new(HashMap::new())),
            rate_limiters: Arc::new(RwLock::new(HashMap::new())),
            load_balancing: LoadBalancingStrategy::RoundRobin,
            endpoint_weights: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Get or create circuit breaker for service
    pub fn get_circuit_breaker(&self, service: &str, config: CircuitBreakerConfig) -> CircuitBreaker {
        let mut breakers = self.circuit_breakers.write().unwrap();
        
        breakers.entry(service.to_string())
            .or_insert_with(|| CircuitBreaker::new(service.to_string(), config))
            .clone()
    }
    
    /// Check if circuit breaker allows request
    pub fn allow_request(&self, service: &str) -> Result<bool> {
        let breakers = self.circuit_breakers.read().unwrap();
        
        if let Some(circuit) = breakers.get(service) {
            Ok(circuit.allow_request())
        } else {
            Ok(true) // Allow if circuit breaker not configured
        }
    }
    
    /// Record request success
    pub fn record_success(&self, service: &str) -> Result<()> {
        let mut breakers = self.circuit_breakers.write().unwrap();
        
        if let Some(circuit) = breakers.get_mut(service) {
            circuit.record_success();
        }
        
        Ok(())
    }
    
    /// Record request failure
    pub fn record_failure(&self, service: &str) -> Result<()> {
        let mut breakers = self.circuit_breakers.write().unwrap();
        
        if let Some(circuit) = breakers.get_mut(service) {
            circuit.record_failure();
        }
        
        Ok(())
    }
    
    /// Get or create rate limiter for tenant
    pub fn get_rate_limiter(&self, tenant_id: &str, config: RateLimiterConfig) -> RateLimiter {
        let mut limiters = self.rate_limiters.write().unwrap();
        
        limiters.entry(tenant_id.to_string())
            .or_insert_with(|| RateLimiter::new(config))
            .clone()
    }
    
    /// Check if request is allowed by rate limiter
    pub fn check_rate_limit(&self, tenant_id: &str, request_weight: u32) -> Result<bool> {
        let limiters = self.rate_limiters.read().unwrap();
        
        if let Some(limiter) = limiters.get(tenant_id) {
            Ok(limiter.check(request_weight))
        } else {
            Ok(true) // Allow if rate limiter not configured
        }
    }
    
    /// Record request usage for rate limiter
    pub fn record_request(&self, tenant_id: &str, request_weight: u32) -> Result<()> {
        let mut limiters = self.rate_limiters.write().unwrap();
        
        if let Some(limiter) = limiters.get_mut(tenant_id) {
            limiter.record(request_weight);
        }
        
        Ok(())
    }
    
    /// Select endpoint using configured load balancing strategy
    pub fn select_endpoint(&self, service: &str, endpoints: &[Endpoint]) -> Result<Option<Endpoint>> {
        if endpoints.is_empty() {
            return Ok(None);
        }
        
        match self.load_balancing {
            LoadBalancingStrategy::RoundRobin => self.round_robin(endpoints),
            LoadBalancingStrategy::LeastConnections => self.least_connections(endpoints),
            LoadBalancingStrategy::Random => self.random(endpoints),
            LoadBalancingStrategy::RingHash => self.ring_hash(service, endpoints),
            LoadBalancingStrategy::Maglev => self.maglev(service, endpoints),
        }
    }
    
    /// Round-robin load balancing
    fn round_robin(&self, endpoints: &[Endpoint]) -> Result<Option<Endpoint>> {
        // In production, would track per-service round-robin state
        Ok(endpoints.first().cloned())
    }
    
    /// Least connections load balancing
    fn least_connections(&self, endpoints: &[Endpoint]) -> Result<Option<Endpoint>> {
        endpoints.iter()
            .min_by_key(|e| e.active_connections)
            .cloned()
            .ok_or_else(|| ServiceMeshError::DiscoveryError("No endpoints available".to_string()))
    }
    
    /// Random load balancing
    fn random(&self, endpoints: &[Endpoint]) -> Result<Option<Endpoint>> {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        Ok(endpoints.choose(&mut rng).cloned())
    }
    
    /// Ring hash load balancing
    fn ring_hash(&self, service: &str, endpoints: &[Endpoint]) -> Result<Option<Endpoint>> {
        // Simplified ring hash implementation
        Ok(endpoints.first().cloned())
    }
    
    /// Maglev load balancing
    fn maglev(&self, service: &str, endpoints: &[Endpoint]) -> Result<Option<Endpoint>> {
        // Simplified Maglev implementation
        Ok(endpoints.first().cloned())
    }
    
    /// Set endpoint weights for weighted load balancing
    pub fn set_endpoint_weights(&self, service: &str, weights: HashMap<String, u32>) {
        let mut weight_map = self.endpoint_weights.write().unwrap();
        weight_map.insert(service.to_string(), weights);
    }
    
    /// Get service stats
    pub fn get_stats(&self) -> TrafficManagerStats {
        let breakers = self.circuit_breakers.read().unwrap();
        let limiters = self.rate_limiters.read().unwrap();
        
        let mut circuit_breaker_states = HashMap::new();
        for (service, breaker) in breakers.iter() {
            circuit_breaker_states.insert(service.clone(), breaker.state());
        }
        
        TrafficManagerStats {
            circuit_breakers: circuit_breaker_states,
            rate_limiters: limiters.len(),
        }
    }
}

/// Circuit breaker state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    /// Circuit is closed, requests pass through
    Closed,
    
    /// Circuit is open, requests are rejected
    Open,
    
    /// Circuit is half-open, testing if service recovered
    HalfOpen,
}

/// Circuit breaker for service protection
pub struct CircuitBreaker {
    /// Service name
    service: String,
    
    /// Current state
    state: CircuitState,
    
    /// Configuration
    config: CircuitBreakerConfig,
    
    /// Failure counter
    failure_count: u32,
    
    /// Success counter
    success_count: u32,
    
    /// Last failure time
    last_failure_time: Option<Instant>,
    
    /// Next allowed request time (for half-open state)
    next_request_time: Option<Instant>,
}

impl CircuitBreaker {
    /// Create new circuit breaker
    pub fn new(service: String, config: CircuitBreakerConfig) -> Self {
        Self {
            service,
            state: CircuitState::Closed,
            config,
            failure_count: 0,
            success_count: 0,
            last_failure_time: None,
            next_request_time: None,
        }
    }
    
    /// Check if request is allowed
    pub fn allow_request(&self) -> bool {
        match self.state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                // Check if recovery timeout has passed
                if let Some(next_time) = self.next_request_time {
                    Instant::now() >= next_time
                } else {
                    false
                }
            }
            CircuitState::HalfOpen => true,
        }
    }
    
    /// Record request success
    pub fn record_success(&mut self) {
        self.success_count += 1;
        self.failure_count = 0; // Reset failure count on success
        
        // Transition from half-open to closed
        if self.state == CircuitState::HalfOpen {
            self.state = CircuitState::Closed;
        }
    }
    
    /// Record request failure
    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure_time = Some(Instant::now());
        
        // Check if we should open the circuit
        if self.failure_count >= self.config.error_threshold_percentage as u32 {
            self.state = CircuitState::Open;
            self.next_request_time = Some(
                Instant::now() + Duration::from_millis(self.config.recovery_timeout_ms)
            );
        }
    }
    
    /// Transition to half-open state
    pub fn half_open(&mut self) {
        if self.state == CircuitState::Open {
            self.state = CircuitState::HalfOpen;
        }
    }
    
    /// Get current state
    pub fn state(&self) -> CircuitState {
        self.state
    }
    
    /// Get failure count
    pub fn failure_count(&self) -> u32 {
        self.failure_count
    }
    
    /// Get service name
    pub fn service(&self) -> &str {
        &self.service
    }
}

/// Rate limiter configuration
#[derive(Debug, Clone)]
pub struct RateLimiterConfig {
    /// Requests per second
    pub requests_per_second: f64,
    
    /// Burst size
    pub burst_size: u32,
    
    /// Time window in seconds
    pub window_seconds: u64,
}

/// Token bucket rate limiter
pub struct RateLimiter {
    /// Configuration
    config: RateLimiterConfig,
    
    /// Token bucket
    tokens: f64,
    
    /// Last update time
    last_update: Instant,
    
    /// Request count in current window
    request_count: u64,
    
    /// Window start time
    window_start: Instant,
}

impl RateLimiter {
    /// Create new rate limiter
    pub fn new(config: RateLimiterConfig) -> Self {
        Self {
            config,
            tokens: config.burst_size as f64,
            last_update: Instant::now(),
            request_count: 0,
            window_start: Instant::now(),
        }
    }
    
    /// Check if request is allowed
    pub fn check(&mut self, weight: u32) -> bool {
        self.refill_tokens();
        
        if self.tokens >= weight as f64 {
            true
        } else {
            // Check if within rate limit
            let elapsed = self.window_start.elapsed().as_secs();
            if elapsed >= self.config.window_seconds {
                self.window_start = Instant::now();
                self.request_count = 0;
            }
            
            let current_rate = self.request_count as f64 / self.config.window_seconds as f64;
            current_rate < self.config.requests_per_second
        }
    }
    
    /// Record request usage
    pub fn record(&mut self, weight: u32) {
        self.tokens = (self.tokens - weight as f64).max(0.0);
        self.request_count += 1;
    }
    
    /// Refill tokens based on elapsed time
    fn refill_tokens(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update).as_secs_f64();
        
        self.tokens = (self.tokens + elapsed * self.config.requests_per_second as f64)
            .min(self.config.burst_size as f64);
        
        self.last_update = now;
    }
}

/// Traffic manager statistics
#[derive(Debug, Clone)]
pub struct TrafficManagerStats {
    pub circuit_breakers: HashMap<String, CircuitState>,
    pub rate_limiters: usize,
}

/// Endpoint information for load balancing
#[derive(Debug, Clone)]
pub struct Endpoint {
    /// Endpoint address
    pub address: String,
    
    /// Port
    pub port: u16,
    
    /// Weight for weighted load balancing
    pub weight: u32,
    
    /// Active connections
    pub active_connections: u32,
    
    /// Health status
    pub health: HealthStatus,
    
    /// Metadata
    pub metadata: HashMap<String, String>,
}

/// Health status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    /// Endpoint is healthy
    Healthy,
    
    /// Endpoint is unhealthy
    Unhealthy,
    
    /// Health status unknown
    Unknown,
}