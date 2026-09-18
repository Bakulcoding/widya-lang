// ============================================================================
// Interrupts & Exceptions Framework (TAHAP 1.3)
// ============================================================================
// Interrupt descriptor table, IRQ handling, exception handling
// Features:
// - Interrupt descriptor table (IDT)
// - IRQ handling (hardware interrupts)
// - Exception handling (CPU exceptions)
// - Interrupt masking/unmasking
// ============================================================================

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Interrupt vector number (0-255)
pub type InterruptVector = u8;

/// Interrupt type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterruptType {
    Exception,      // CPU exceptions (0-31)
    IRQ,           // Hardware interrupts (32-47)
    Syscall,       // System calls (128, 0x80)
    Software,      // Software interrupts
}

/// Exception error code present
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExceptionErrorCode {
    Present(u64),
    NotPresent,
}

/// Exception information
#[derive(Debug, Clone)]
pub struct ExceptionInfo {
    pub vector: InterruptVector,
    pub error_code: Option<u64>,
    pub instruction_pointer: usize,
    pub stack_pointer: usize,
    pub faulting_address: Option<usize>,
}

/// Interrupt frame (saved CPU state)
#[derive(Debug, Clone)]
pub struct InterruptFrame {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

/// Interrupt handler callback
pub type InterruptHandler = fn(&InterruptFrame) -> bool;

/// Interrupt descriptor
#[derive(Debug, Clone)]
pub struct InterruptDescriptor {
    pub vector: InterruptVector,
    pub handler: InterruptHandler,
    pub type_: InterruptType,
    pub present: bool,
    pub privilege_level: u8, // 0-3 (0 = kernel, 3 = user)
    pub interrupt_stack_table: Option<usize>, // IST index for double-fault, etc.
}

/// Interrupt Descriptor Table
pub struct InterruptDescriptorTable {
    pub descriptors: RefCell<[Option<InterruptDescriptor>; 256]>,
    pub idtr_base: AtomicUsize,
    pub idtr_limit: AtomicU16,
}

impl InterruptDescriptorTable {
    pub const fn new() -> Self {
        Self {
            descriptors: RefCell::new([const { None }; 256]),
            idtr_base: AtomicUsize::new(0),
            idtr_limit: AtomicU16::new((256 * 16 - 1) as u16),
        }
    }

    /// Set interrupt handler for a vector
    pub fn set_handler(&self, vector: InterruptVector, handler: InterruptHandler, 
                       interrupt_type: InterruptType) -> Result<(), String> {
        let mut descriptors = self.descriptors.borrow_mut();
        
        if vector >= 256 {
            return Err("Invalid interrupt vector".to_string());
        }

        descriptors[vector as usize] = Some(InterruptDescriptor {
            vector,
            handler,
            type_: interrupt_type,
            present: true,
            privilege_level: 0, // Kernel mode only
            interrupt_stack_table: None,
        });

        Ok(())
    }

    /// Get interrupt handler for a vector
    pub fn get_handler(&self, vector: InterruptVector) -> Option<InterruptHandler> {
        let descriptors = self.descriptors.borrow();
        descriptors[vector as usize].as_ref().map(|d| d.handler)
    }

    /// Unset interrupt handler
    pub fn unset_handler(&self, vector: InterruptVector) -> bool {
        let mut descriptors = self.descriptors.borrow_mut();
        if vector >= 256 {
            return false;
        }
        let was_present = descriptors[vector as usize].is_some();
        descriptors[vector as usize] = None;
        was_present
    }

    /// Get interrupt count for a vector
    pub fn handler_count(&self) -> usize {
        let descriptors = self.descriptors.borrow();
        descriptors.iter().filter(|h| h.is_some()).count()
    }

    /// Check if handler is present for vector
    pub fn has_handler(&self, vector: InterruptVector) -> bool {
        let descriptors = self.descriptors.borrow();
        descriptors[vector as usize].is_some()
    }

    /// Enable interrupt for vector
    pub fn enable(&self, vector: InterruptVector) {
        let mut descriptors = self.descriptors.borrow_mut();
        if let Some(ref mut desc) = descriptors[vector as usize] {
            desc.present = true;
        }
    }

    /// Disable interrupt for vector
    pub fn disable(&self, vector: InterruptVector) {
        let mut descriptors = self.descriptors.borrow_mut();
        if let Some(ref mut desc) = descriptors[vector as usize] {
            desc.present = false;
        }
    }

    /// Check if interrupt is enabled
    pub fn is_enabled(&self, vector: InterruptVector) -> bool {
        let descriptors = self.descriptors.borrow();
        descriptors[vector as usize].as_ref().map(|d| d.present).unwrap_or(false)
    }
}

impl Default for InterruptDescriptorTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Interrupt controller (simplified APIC/ICMP)
pub struct InterruptController {
    pub idt: InterruptDescriptorTable,
    pub enabled_irqs: AtomicUsize,
    pub pending_irqs: AtomicUsize,
    pub irqs_handled: [AtomicUsize; 16],
}

impl InterruptController {
    pub fn new() -> Self {
        let mut irqs_handled = [AtomicUsize::new(0); 16];
        for i in 0..16 {
            irqs_handled[i] = AtomicUsize::new(0);
        }

        Self {
            idt: InterruptDescriptorTable::new(),
            enabled_irqs: AtomicUsize::new(0),
            pending_irqs: AtomicUsize::new(0),
            irqs_handled,
        }
    }

    /// Register IRQ handler
    pub fn register_irq(&self, irq: u8, handler: InterruptHandler) -> Result<(), String> {
        // Map IRQ to IDT vector (IRQ 0-15 -> vectors 32-47)
        let vector = 32 + irq;
        
        if irq > 15 {
            return Err("Invalid IRQ number (must be 0-15)".to_string());
        }

        self.idt.set_handler(vector, handler, InterruptType::IRQ)?;
        
        // Mark IRQ as enabled
        self.enabled_irqs.fetch_or(1 << irq, Ordering::SeqCst);
        
        Ok(())
    }

    /// Unregister IRQ handler
    pub fn unregister_irq(&self, irq: u8) -> bool {
        if irq > 15 {
            return false;
        }

        let vector = 32 + irq;
        let was_present = self.idt.unset_handler(vector);
        
        if was_present {
            self.enabled_irqs.fetch_and(!(1 << irq), Ordering::SeqCst);
        }

        was_present
    }

    /// Enable IRQ
    pub fn enable_irq(&self, irq: u8) -> bool {
        if irq > 15 {
            return false;
        }

        let vector = 32 + irq;
        self.idt.enable(vector);
        
        self.enabled_irqs.fetch_or(1 << irq, Ordering::SeqCst);
        
        true
    }

    /// Disable IRQ
    pub fn disable_irq(&self, irq: u8) -> bool {
        if irq > 15 {
            return false;
        }

        let vector = 32 + irq;
        self.idt.disable(vector);
        
        self.enabled_irqs.fetch_and(!(1 << irq), Ordering::SeqCst);
        
        true
    }

    /// Check if IRQ is enabled
    pub fn is_irq_enabled(&self, irq: u8) -> bool {
        if irq > 15 {
            return false;
        }
        (self.enabled_irqs.load(Ordering::SeqCst) & (1 << irq)) != 0
    }

    /// Handle interrupt (called from interrupt handler assembly)
    pub fn handle_interrupt(&self, vector: InterruptVector, frame: &InterruptFrame) -> bool {
        // Get handler
        let handler = match self.idt.get_handler(vector) {
            Some(h) => h,
            None => {
                // No handler - unknown interrupt
                return false;
            }
        };

        // Call handler
        let handled = handler(frame);

        if handled && vector >= 32 && vector <= 47 {
            // Update IRQ counter
            let irq = vector - 32;
            self.irqs_handled[irq as usize].fetch_add(1, Ordering::SeqCst);
        }

        handled
    }

    /// Get IRQ handled count
    pub fn irq_count(&self, irq: u8) -> usize {
        if irq > 15 {
            return 0;
        }
        self.irqs_handled[irq as usize].load(Ordering::SeqCst)
    }

    /// Get total IRQs handled
    pub fn total_irqs(&self) -> usize {
        self.irqs_handled.iter().map(|c| c.load(Ordering::SeqCst)).sum()
    }

    /// Mark IRQ as pending
    pub fn set_pending(&self, irq: u8) {
        if irq > 15 {
            return;
        }
        self.pending_irqs.fetch_or(1 << irq, Ordering::SeqCst);
    }

    /// Clear pending IRQ
    pub fn clear_pending(&self, irq: u8) {
        if irq > 15 {
            return;
        }
        self.pending_irqs.fetch_and(!(1 << irq), Ordering::SeqCst);
    }

    /// Check if IRQ is pending
    pub fn is_pending(&self, irq: u8) -> bool {
        if irq > 15 {
            return false;
        }
        (self.pending_irqs.load(Ordering::SeqCst) & (1 << irq)) != 0
    }

    /// Get pending IRQs bitmask
    pub fn pending_mask(&self) -> usize {
        self.pending_irqs.load(Ordering::SeqCst)
    }
}

/// Exception handler for CPU exceptions
pub struct ExceptionHandler {
    pub idt: InterruptDescriptorTable,
    pub exceptions: RefCell<HashMap<InterruptVector, ExceptionHandlerFn>>,
}

type ExceptionHandlerFn = fn(&ExceptionInfo) -> bool;

impl ExceptionHandler {
    pub fn new() -> Self {
        let mut exceptions = HashMap::new();
        
        // Register default handlers
        for vector in 0..32 {
            exceptions.insert(vector, Self::default_exception_handler);
        }

        Self {
            idt: InterruptDescriptorTable::new(),
            exceptions: RefCell::new(exceptions),
        }
    }

    /// Default exception handler
    fn default_exception_handler(_info: &ExceptionInfo) -> bool {
        // In real implementation, this would:
        // 1. Print error message
        // 2. Dump registers
        // 3. Panic or attempt recovery
        
        // For now, return false to indicate non-recoverable
        false
    }

    /// Register custom exception handler
    pub fn register_exception(&self, vector: InterruptVector, handler: ExceptionHandlerFn) {
        let mut exceptions = self.exceptions.borrow_mut();
        exceptions.insert(vector, handler);
    }

    /// Handle exception
    pub fn handle_exception(&self, info: ExceptionInfo) -> bool {
        let exceptions = self.exceptions.borrow();
        let handler = exceptions.get(&info.vector);
        
        match handler {
            Some(h) => h(&info),
            None => Self::default_exception_handler(&info),
        }
    }

    /// Get exception handler count
    pub fn handler_count(&self) -> usize {
        self.exceptions.borrow().len()
    }
}

/// System call handler
pub struct SyscallHandler {
    pub handlers: RefCell<HashMap<u64, SyscallHandlerFn>>,
}

type SyscallHandlerFn = fn(&[usize]) -> usize;

impl SyscallHandler {
    pub fn new() -> Self {
        Self {
            handlers: RefCell::new(HashMap::new()),
        }
    }

    /// Register system call handler
    pub fn register(&self, syscall_num: u64, handler: SyscallHandlerFn) {
        let mut handlers = self.handlers.borrow_mut();
        handlers.insert(syscall_num, handler);
    }

    /// Handle system call
    pub fn handle(&self, syscall_num: u64, args: &[usize]) -> usize {
        let handlers = self.handlers.borrow();
        
        if let Some(handler) = handlers.get(&syscall_num) {
            handler(args)
        } else {
            // syscall not implemented
            usize::MAX
        }
    }

    /// Get handler count
    pub fn handler_count(&self) -> usize {
        self.handlers.borrow().len()
    }
}

/// Global interrupt controller
pub struct GlobalInterruptController {
    pub PIC: InterruptController,
    pub exception_handler: ExceptionHandler,
    pub syscall_handler: SyscallHandler,
    pub interrupts_enabled: AtomicUsize,
}

impl GlobalInterruptController {
    pub fn new() -> Self {
        Self {
            PIC: InterruptController::new(),
            exception_handler: ExceptionHandler::new(),
            syscall_handler: SyscallHandler::new(),
            interrupts_enabled: AtomicUsize::new(0),
        }
    }

    /// Enable all interrupts
    pub fn enable_all_interrupts(&self) {
        // In real implementation, execute `sti` instruction
        self.interrupts_enabled.store(1, Ordering::SeqCst);
    }

    /// Disable all interrupts
    pub fn disable_all_interrupts(&self) {
        // In real implementation, execute `cli` instruction
        self.interrupts_enabled.store(0, Ordering::SeqCst);
    }

    /// Check if interrupts are enabled
    pub fn are_interrupts_enabled(&self) -> bool {
        self.interrupts_enabled.load(Ordering::SeqCst) == 1
    }

    /// Initialize IDT
    pub fn init_idt(&self) {
        // Set up exception handlers (vectors 0-31)
        let exceptions = [
            (0, "Divide Error"),
            (1, "Debug"),
            (2, "NMI"),
            (3, "Breakpoint"),
            (4, "Overflow"),
            (5, "Bound Range"),
            (6, "Invalid Opcode"),
            (7, "Device Not Available"),
            (8, "Double Fault"),
            (9, "Coprocessor Segment Overrun"),
            (10, "Invalid TSS"),
            (11, "Segment Not Present"),
            (12, "Stack Segment Fault"),
            (13, "General Protection"),
            (14, "Page Fault"),
            (15, "Reserved"),
            (16, "x87 FPU Error"),
            (17, "Alignment Check"),
            (18, "Machine Check"),
            (19, "SIMD FPU"),
            (20, "Virtualization"),
            (21, "Control Protection"),
            (22, "Reserved"),
            (23, "Reserved"),
            (24, "Reserved"),
            (25, "Reserved"),
            (26, "Reserved"),
            (27, "Reserved"),
            (28, "Reserved"),
            (29, "Reserved"),
            (30, "Security Exception"),
            (31, "Reserved"),
        ];

        for (vector, name) in exceptions {
            let name = name.to_string();
            self.exception_handler.register_exception(vector as u8, move |info| {
                // In real implementation, print error and panic
                let _ = name;
                let _ = info;
                false
            });
        }

        // Set up system call handler
        self.register_syscall(0x80, |args| {
            // Linux-style system call
            let syscall_num = args[0];
            
            // Handle common syscalls
            match syscall_num {
                0 => 0, // read
                1 => 0, // write
                2 => 0, // open
                3 => 0, // close
                60 => 0, // exit
                63 => 0, // brk
                _ => usize::MAX,
            }
        });
    }

    /// Register system call
    pub fn register_syscall(&self, syscall_num: u64, handler: SyscallHandlerFn) {
        self.syscall_handler.register(syscall_num, handler);
    }

    /// Handle interrupt from assembly
    pub fn handle_from_asm(&self, vector: u8, frame: &InterruptFrame) -> bool {
        if vector < 32 {
            // Exception
            let info = ExceptionInfo {
                vector,
                error_code: None,
                instruction_pointer: frame.rip as usize,
                stack_pointer: frame.rsp as usize,
                faulting_address: if vector == 14 { Some(0) } else { None }, // CR2 for page fault
            };
            self.exception_handler.handle_exception(info)
        } else if vector == 128 || vector == 0x80 {
            // System call
            // Parse arguments from frame
            false // Not implemented
        } else {
            // IRQ
            self.PIC.handle_interrupt(vector, frame)
        }
    }
}

impl Default for GlobalInterruptController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idt_basic() {
        let idt = InterruptDescriptorTable::new();
        
        fn test_handler(_frame: &InterruptFrame) -> bool {
            true
        }

        idt.set_handler(32, test_handler, InterruptType::IRQ).unwrap();
        assert!(idt.has_handler(32));
        
        let handler = idt.get_handler(32);
        assert!(handler.is_some());
    }

    #[test]
    fn test_interrupt_controller() {
        let ic = InterruptController::new();
        
        fn test_handler(_frame: &InterruptFrame) -> bool { true }
        
        ic.register_irq(0, test_handler).unwrap();
        assert!(ic.is_irq_enabled(0));
        
        ic.disable_irq(0);
        assert!(!ic.is_irq_enabled(0));
        
        ic.enable_irq(0);
        assert!(ic.is_irq_enabled(0));
    }

    #[test]
    fn test_exception_handler() {
        let eh = ExceptionHandler::new();
        
        let info = ExceptionInfo {
            vector: 14, // Page fault
            error_code: Some(0),
            instruction_pointer: 0x1000,
            stack_pointer: 0x2000,
            faulting_address: Some(0x3000),
        };
        
        let handled = eh.handle_exception(info);
        assert!(!handled); // Default handler returns false
    }

    #[test]
    fn test_system_call() {
        let sh = SyscallHandler::new();
        
        sh.register(0, |args| args[0] + args[1]);
        
        let result = sh.handle(0, &[10, 20]);
        assert_eq!(result, 30);
    }

    #[test]
    fn test_global_interrupt_controller() {
        let gic = GlobalInterruptController::new();
        gic.init_idt();
        
        gic.enable_all_interrupts();
        assert!(gic.are_interrupts_enabled());
        
        gic.disable_all_interrupts();
        assert!(!gic.are_interrupts_enabled());
    }
}
