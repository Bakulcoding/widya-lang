// ============================================================================
// OS Module Library
// ============================================================================
// Core OS primitives untuk Widya OS development
// ============================================================================

pub mod memory;
pub mod threading;
pub mod interrupts;
pub mod syscall;
pub mod vfs;

pub use memory::{MemoryManager, MemoryStats, PageAllocator, PageFlags, PageTableEntry, VirtualAddressSpace};
pub use threading::{KernelThreadManager, Priority, ThreadHandle, ThreadID, ThreadState};
pub use interrupts::{GlobalInterruptController, InterruptController, ExceptionHandler, SyscallHandler};
pub use syscall::{SyscallTable, SyscallError, FileStat, OpenFile, SyscallNumber, FD};
pub use vfs::{VirtualFilesystem, OpenFlags, InodeMetadata, FileType, DirEntry, VfsError};
