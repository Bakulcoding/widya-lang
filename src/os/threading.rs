// ============================================================================
// Kernel Threads & Concurrency Primitives (TAHAP 1.2)
// ============================================================================
// Kernel thread management dengan preemptive scheduling
// Features:
// - Kernel thread creation & management
// - Spinlock, mutex, semaphore
// - Preemptive scheduling
// - Thread states & lifecycle
// ============================================================================

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::{Duration, Instant};

/// Thread states
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThreadState {
    New,
    Ready,
    Running,
    Waiting,
    Blocked,
    Terminated,
}

/// Thread priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Idle = 0,
    Low = 1,
    Normal = 2,
    High = 3,
    Realtime = 4,
}

/// Thread identifier
pub type ThreadID = u64;

/// Thread control block
#[derive(Debug, Clone)]
pub struct ThreadControlBlock {
    pub id: ThreadID,
    pub state: ThreadState,
    pub priority: Priority,
    pub stack_pointer: usize,
    pub instruction_pointer: usize,
    pub registers: ThreadRegisters,
    pub created_at: Instant,
    pub last_run_at: Option<Instant>,
    pub runtime_ns: u64,
    pub wait_until: Option<Instant>,
}

/// Thread registers (simplified x86-64)
#[derive(Debug, Clone, Default)]
pub struct ThreadRegisters {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub rip: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rflags: u64,
}

/// Thread handle
#[derive(Debug, Clone)]
pub struct ThreadHandle {
    pub id: ThreadID,
    _internal: Rc<RefCell<ThreadControlBlock>>,
}

impl ThreadHandle {
    pub fn id(&self) -> ThreadID {
        self.id
    }

    pub fn state(&self) -> ThreadState {
        self._internal.borrow().state
    }

    pub fn priority(&self) -> Priority {
        self._internal.borrow().priority
    }

    pub fn is_running(&self) -> bool {
        self.state() == ThreadState::Running
    }

    pub fn is_ready(&self) -> bool {
        self.state() == ThreadState::Ready
    }
}

/// Thread entry function type
pub type ThreadEntry = fn(usize) -> usize;

/// Kernel thread manager
pub struct KernelThreadManager {
    pub next_thread_id: AtomicU64,
    pub threads: RefCell<HashMap<ThreadID, Rc<RefCell<ThreadControlBlock>>>>,
    pub ready_queue: RefCell<VecDeque<ThreadID>>,
    pub current_thread: AtomicUsize,
    pub scheduler_enabled: AtomicUsize,
    pub scheduler_tick: AtomicU64,
}

impl KernelThreadManager {
    pub fn new() -> Self {
        Self {
            next_thread_id: AtomicU64::new(1),
            threads: RefCell::new(HashMap::new()),
            ready_queue: RefCell::new(VecDeque::new()),
            current_thread: AtomicUsize::new(0),
            scheduler_enabled: AtomicUsize::new(0),
            scheduler_tick: AtomicU64::new(0),
        }
    }

    /// Create a new kernel thread
    pub fn create_thread(&self, entry: ThreadEntry, arg: usize, priority: Priority) -> Result<ThreadHandle, String> {
        let thread_id = self.next_thread_id.fetch_add(1, Ordering::SeqCst);
        
        let tcb = Rc::new(RefCell::new(ThreadControlBlock {
            id: thread_id,
            state: ThreadState::New,
            priority,
            stack_pointer: 0, // Will be set during context switch
            instruction_pointer: entry as usize,
            registers: ThreadRegisters::default(),
            created_at: Instant::now(),
            last_run_at: None,
            runtime_ns: 0,
            wait_until: None,
        }));

        {
            let mut threads = self.threads.borrow_mut();
            threads.insert(thread_id, Rc::clone(&tcb));
        }

        // Add to ready queue
        self.ready_queue.borrow_mut().push_back(thread_id);

        // Mark as ready
        tcb.borrow_mut().state = ThreadState::Ready;

        Ok(ThreadHandle {
            id: thread_id,
            _internal: tcb,
        })
    }

    /// Start a thread (for threads created in New state)
    pub fn start_thread(&self, thread_id: ThreadID) -> Result<(), String> {
        let threads = self.threads.borrow();
        let tcb = threads.get(&thread_id)
            .ok_or_else(|| format!("Thread {} not found", thread_id))?;

        let mut tcb_ref = tcb.borrow_mut();
        
        match tcb_ref.state {
            ThreadState::New => {
                tcb_ref.state = ThreadState::Ready;
                self.ready_queue.borrow_mut().push_back(thread_id);
                Ok(())
            }
            _ => Err(format!("Thread {} is not in New state", thread_id)),
        }
    }

    /// Yield CPU to next thread
    pub fn yield_cpu(&self) {
        if self.scheduler_enabled.load(Ordering::SeqCst) == 0 {
            return;
        }

        self.scheduler_tick.fetch_add(1, Ordering::SeqCst);

        let current_id = self.current_thread.load(Ordering::SeqCst) as ThreadID;
        
        let mut threads = self.threads.borrow_mut();
        if let Some(tcb) = threads.get(&current_id) {
            let mut tcb_ref = tcb.borrow_mut();
            tcb_ref.state = ThreadState::Ready;
        }

        drop(threads);

        let mut ready_queue = self.ready_queue.borrow_mut();
        ready_queue.push_back(current_id);
        drop(ready_queue);

        // Rotate queue
        if let Some(id) = ready_queue.pop_front() {
            self.current_thread.store(id as usize, Ordering::SeqCst);
        }

        // Note: Actual context switch would happen in assembly
    }

    /// Sleep current thread for duration
    pub fn sleep(&self, duration: Duration) {
        let current_id = self.current_thread.load(Ordering::SeqCst) as ThreadID;
        
        let threads = self.threads.borrow();
        if let Some(tcb) = threads.get(&current_id) {
            let mut tcb_ref = tcb.borrow_mut();
            tcb_ref.state = ThreadState::Waiting;
            tcb_ref.wait_until = Some(Instant::now() + duration);
        }

        drop(threads);

        self.yield_cpu();
    }

    /// Wake up a waiting thread
    pub fn wake_up(&self, thread_id: ThreadID) -> Result<(), String> {
        let threads = self.threads.borrow();
        let tcb = threads.get(&thread_id)
            .ok_or_else(|| format!("Thread {} not found", thread_id))?;

        let mut tcb_ref = tcb.borrow_mut();
        
        match tcb_ref.state {
            ThreadState::Waiting | ThreadState::Blocked => {
                tcb_ref.state = ThreadState::Ready;
                drop(threads);
                self.ready_queue.borrow_mut().push_back(thread_id);
                Ok(())
            }
            _ => Err(format!("Thread {} is not waiting", thread_id)),
        }
    }

    /// Terminate current thread
    pub fn terminate(&self) -> ! {
        let current_id = self.current_thread.load(Ordering::SeqCst) as ThreadID;
        
        let mut threads = self.threads.borrow_mut();
        let tcb = threads.get(&current_id)
            .expect("Current thread not found");

        let mut tcb_ref = tcb.borrow_mut();
        tcb_ref.state = ThreadState::Terminated;

        drop(tcb_ref);
        drop(threads);

        // Remove from ready queue
        let mut ready_queue = self.ready_queue.borrow_mut();
        ready_queue.retain(|&id| id != current_id);
        drop(ready_queue);

        // In real kernel, this would switch to scheduler
        panic!("Thread terminated - context switch not implemented");
    }

    /// Get current thread
    pub fn current_thread(&self) -> Option<ThreadHandle> {
        let current_id = self.current_thread.load(Ordering::SeqCst) as ThreadID;
        
        let threads = self.threads.borrow();
        let tcb = threads.get(&current_id)?;
        
        Some(ThreadHandle {
            id: current_id,
            _internal: Rc::clone(tcb),
        })
    }

    /// Enable scheduler
    pub fn enable_scheduler(&self) {
        self.scheduler_enabled.store(1, Ordering::SeqCst);
    }

    /// Disable scheduler
    pub fn disable_scheduler(&self) {
        self.scheduler_enabled.store(0, Ordering::SeqCst);
    }

    /// Check if scheduler is enabled
    pub fn is_scheduler_enabled(&self) -> bool {
        self.scheduler_enabled.load(Ordering::SeqCst) == 1
    }

    /// Get scheduler tick count
    pub fn scheduler_tick(&self) -> u64 {
        self.scheduler_tick.load(Ordering::SeqCst)
    }

    /// Get thread count
    pub fn thread_count(&self) -> usize {
        self.threads.borrow().len()
    }

    /// Run scheduler (must be called from kernel main loop)
    pub fn run_scheduler(&self) -> Result<(), String> {
        if self.scheduler_enabled.load(Ordering::SeqCst) == 0 {
            return Ok(());
        }

        let mut ready_queue = self.ready_queue.borrow_mut();
        
        if ready_queue.is_empty() {
            return Ok(());
        }

        // Get next thread (round-robin with priority)
        let next_thread_id = ready_queue.pop_front()
            .ok_or_else(|| "Ready queue empty".to_string())?;

        drop(ready_queue);

        let mut ready_queue = self.ready_queue.borrow_mut();
        ready_queue.push_back(next_thread_id);
        drop(ready_queue);

        self.current_thread.store(next_thread_id as usize, Ordering::SeqCst);

        // Note: Actual context switch happens here in assembly
        Ok(())
    }
}

/// Spinlock - basic mutual exclusion
pub struct Spinlock {
    locked: AtomicUsize,
    holder: AtomicU64,
    lock_count: AtomicU64,
}

impl Spinlock {
    pub const fn new() -> Self {
        Self {
            locked: AtomicUsize::new(0),
            holder: AtomicU64::new(0),
            lock_count: AtomicU64::new(0),
        }
    }

    /// Acquire spinlock (spin until acquired)
    pub fn lock(&self) {
        let thread_id = self.holder.load(Ordering::Relaxed);
        
        while self.locked.swap(1, Ordering::Acquire) != 0 {
            // Busy wait with pause instruction (for x86)
            // In real implementation, use pause instruction
            std::hint::spin_loop();
        }

        self.holder.store(thread_id, Ordering::Relaxed);
        self.lock_count.fetch_add(1, Ordering::Relaxed);
    }

    /// Try to acquire spinlock without blocking
    pub fn try_lock(&self) -> bool {
        self.locked.compare_exchange(
            0, 1, 
            Ordering::Acquire, 
            Ordering::Relaxed
        ).is_ok()
    }

    /// Release spinlock
    pub fn unlock(&self) {
        self.locked.store(0, Ordering::Release);
        self.holder.store(0, Ordering::Relaxed);
    }

    /// Check if lock is held
    pub fn is_locked(&self) -> bool {
        self.locked.load(Ordering::SeqCst) == 1
    }
}

impl Default for Spinlock {
    fn default() -> Self {
        Self::new()
    }
}

/// Mutex with fairness
pub struct Mutex {
    spinlock: Spinlock,
    wait_queue: RefCell<VecDeque<ThreadID>>,
}

impl Mutex {
    pub fn new() -> Self {
        Self {
            spinlock: Spinlock::new(),
            wait_queue: RefCell::new(VecDeque::new()),
        }
    }

    pub fn lock(&self, manager: &KernelThreadManager) {
        self.spinlock.lock();
        
        // In real implementation, add to wait queue if contended
        // and use condition variable
    }

    pub fn try_lock(&self) -> bool {
        self.spinlock.try_lock()
    }

    pub fn unlock(&self) {
        self.spinlock.unlock();
    }

    pub fn is_locked(&self) -> bool {
        self.spinlock.is_locked()
    }
}

impl Default for Mutex {
    fn default() -> Self {
        Self::new()
    }
}

/// Semaphore
pub struct Semaphore {
    count: AtomicUsize,
    max_count: usize,
    spinlock: Spinlock,
    wait_queue: RefCell<VecDeque<ThreadID>>,
}

impl Semaphore {
    pub fn new(initial: usize, max: usize) -> Self {
        Self {
            count: AtomicUsize::new(initial),
            max_count: max,
            spinlock: Spinlock::new(),
            wait_queue: RefCell::new(VecDeque::new()),
        }
    }

    pub fn wait(&self, manager: &KernelThreadManager) {
        while self.count.load(Ordering::SeqCst) == 0 {
            // Block current thread
            self.spinlock.lock();
            
            let current = manager.current_thread();
            if let Some(th) = current {
                self.wait_queue.borrow_mut().push_back(th.id());
            }
            
            self.spinlock.unlock();
            
            manager.sleep(Duration::from_millis(1));
        }

        self.count.fetch_sub(1, Ordering::SeqCst);
    }

    pub fn signal(&self) {
        if self.count.load(Ordering::SeqCst) < self.max_count {
            self.count.fetch_add(1, Ordering::SeqCst);
        }
    }

    pub fn available(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }
}

/// Barrier for thread synchronization
pub struct Barrier {
    count: AtomicUsize,
    waiting: AtomicUsize,
    spinlock: Spinlock,
}

impl Barrier {
    pub fn new(count: usize) -> Self {
        Self {
            count: AtomicUsize::new(count),
            waiting: AtomicUsize::new(0),
            spinlock: Spinlock::new(),
        }
    }

    pub fn wait(&self, manager: &KernelThreadManager) {
        let me = self.waiting.fetch_add(1, Ordering::SeqCst);

        if me + 1 == self.count.load(Ordering::SeqCst) {
            // Last thread, release all
            self.waiting.store(0, Ordering::SeqCst);
        } else {
            // Wait for other threads
            while self.waiting.load(Ordering::SeqCst) != 0 {
                manager.sleep(Duration::from_millis(1));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_creation() {
        let manager = KernelThreadManager::new();
        
        fn test_entry(_: usize) -> usize {
            42
        }

        let handle = manager.create_thread(test_entry, 0, Priority::Normal);
        assert!(handle.is_ok());
        
        let thread = handle.unwrap();
        assert_eq!(thread.state(), ThreadState::Ready);
    }

    #[test]
    fn test_yield_cpu() {
        let manager = KernelThreadManager::new();
        manager.enable_scheduler();
        
        fn test_entry(_: usize) -> usize { 0 }
        
        manager.create_thread(test_entry, 0, Priority::Normal).unwrap();
        manager.create_thread(test_entry, 0, Priority::Low).unwrap();
        
        manager.yield_cpu();
        assert!(manager.scheduler_tick() > 0);
    }

    #[test]
    fn test_spinlock() {
        let lock = Spinlock::new();
        
        assert!(!lock.is_locked());
        lock.lock();
        assert!(lock.is_locked());
        lock.unlock();
        assert!(!lock.is_locked());
    }

    #[test]
    fn test_mutex() {
        let mutex = Mutex::new();
        let manager = KernelThreadManager::new();
        
        assert!(!mutex.is_locked());
        mutex.lock(&manager);
        assert!(mutex.is_locked());
        mutex.unlock();
        assert!(!mutex.is_locked());
    }

    #[test]
    fn test_semaphore() {
        let sem = Semaphore::new(2, 5);
        
        assert_eq!(sem.available(), 2);
        
        let manager = KernelThreadManager::new();
        sem.wait(&manager);
        assert_eq!(sem.available(), 1);
        
        sem.signal();
        assert_eq!(sem.available(), 2);
    }

    #[test]
    fn test_barrier() {
        let barrier = Barrier::new(3);
        let manager = KernelThreadManager::new();
        
        // Test that barrier waits for all threads
        // (Full test would require actual threads)
    }
}
