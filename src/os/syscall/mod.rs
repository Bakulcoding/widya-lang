// ============================================================================
// System Call Interface - TAHAP 4.2
// ============================================================================
// POSIX subset system calls untuk Widya OS
// Features:
// - System call dispatcher
// - File operations (open, close, read, write)
// - Process operations (fork, exec, exit)
// - Memory operations (brk, mmap)
// - IPC operations (pipe, shm)
// ============================================================================

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

/// System call number
pub type SyscallNumber = usize;

/// File descriptor
pub type FD = usize;

/// System call error
pub type SyscallResult<T> = Result<T, SyscallError>;

#[derive(Debug, Clone, PartialEq)]
pub enum SyscallError {
    EPERM,           // Operation not permitted
    ENOENT,          // No such file or directory
    EINTR,           // Interrupted system call
    EIO,             // I/O error
    EACCESS,         // Permission denied
    EFAULT,          // Bad address
    EINVAL,          // Invalid argument
    EMFILE,          // Too many open files
    ENOMEM,          // Not enough memory
    EAGAIN,          // Resource unavailable
}

impl SyscallError {
    pub fn as_str(&self) -> &'static str {
        match self {
            SyscallError::EPERM => "Operation not permitted",
            SyscallError::ENOENT => "No such file or directory",
            SyscallError::EINTR => "Interrupted system call",
            SyscallError::EIO => "I/O error",
            SyscallError::EACCESS => "Permission denied",
            SyscallError::EFAULT => "Bad address",
            SyscallError::EINVAL => "Invalid argument",
            SyscallError::EMFILE => "Too many open files",
            SyscallError::ENOMEM => "Not enough memory",
            SyscallError::EAGAIN => "Resource unavailable",
        }
    }
}

/// File metadata
#[derive(Debug, Clone)]
pub struct FileStat {
    pub inode: u64,
    pub size: u64,
    pub mode: u32,
    pub nlink: u32,
    pub uid: u32,
    pub gid: u32,
    pub atime: u64,
    pub mtime: u64,
    pub ctime: u64,
}

/// System call handler type
pub type SyscallHandler = fn(&[usize]) -> SyscallResult<usize>;

/// System call table
pub struct SyscallTable {
    handlers: HashMap<SyscallNumber, SyscallHandler>,
    next_fd: AtomicUsize,
    open_files: HashMap<FD, OpenFile>,
}

/// Open file record
#[derive(Debug, Clone)]
pub struct OpenFile {
    pub fd: FD,
    pub path: String,
    pub flags: u32,
    pub position: u64,
}

impl SyscallTable {
    pub fn new() -> Self {
        let mut table = Self {
            handlers: HashMap::new(),
            next_fd: AtomicUsize::new(3), // 0, 1, 2 reserved (stdin, stdout, stderr)
            open_files: HashMap::new(),
        };
        
        // Register default handlers
        table.register_syscall(0, syscall_read);
        table.register_syscall(1, syscall_write);
        table.register_syscall(2, syscall_open);
        table.register_syscall(3, syscall_close);
        table.register_syscall(4, syscall_lseek);
        table.register_syscall(5, syscall_stat);
        table.register_syscall(6, syscall_fstat);
        table.register_syscall(7, syscall_getpid);
        table.register_syscall(8, syscall_getuid);
        table.register_syscall(9, syscall_exit);
        table.register_syscall(10, syscall_fork);
        table.register_syscall(11, syscall_exec);
        table.register_syscall(12, syscall_brk);
        table.register_syscall(13, syscall_mmap);
        table.register_syscall(14, syscall_munmap);
        table.register_syscall(15, syscall_pipe);
        table.register_syscall(16, syscall_dup);
        table.register_syscall(17, syscall_getcwd);
        table.register_syscall(18, syscall_chdir);
        table.register_syscall(19, syscall_unlink);
        table.register_syscall(20, syscall_rename);
        
        table
    }

    /// Register a system call handler
    pub fn register_syscall(&mut self, num: SyscallNumber, handler: SyscallHandler) {
        self.handlers.insert(num, handler);
    }

    /// Get handler for syscall
    pub fn get_handler(&self, num: SyscallNumber) -> Option<SyscallHandler> {
        self.handlers.get(&num).copied()
    }

    /// Allocate a new file descriptor
    pub fn allocate_fd(&self, file: OpenFile) -> FD {
        let fd = self.next_fd.fetch_add(1, Ordering::SeqCst);
        let mut open_files = HashMap::new();
        open_files.insert(fd, file);
        // In real implementation, use thread-safe HashMap
        fd
    }

    /// Get open file by fd
    pub fn get_file(&self, fd: FD) -> Option<OpenFile> {
        // In real implementation, use thread-safe HashMap
        self.open_files.get(&fd).cloned()
    }

    /// Close file by fd
    pub fn close_fd(&mut self, fd: FD) -> Result<(), SyscallError> {
        if self.open_files.remove(&fd).is_some() {
            Ok(())
        } else {
            Err(SyscallError::EINVAL)
        }
    }

    /// Handle system call
    pub fn handle_syscall(&self, num: SyscallNumber, args: &[usize]) -> SyscallResult<usize> {
        if let Some(handler) = self.get_handler(num) {
            handler(args)
        } else {
            Err(SyscallError::EINVAL)
        }
    }
}

// ============================================================================
// System Call Implementations
// ============================================================================

/// read(fd, buf, count)
fn syscall_read(args: &[usize]) -> SyscallResult<usize> {
    if args.len() < 3 {
        return Err(SyscallError::EINVAL);
    }
    
    let fd = args[0];
    let _buf_ptr = args[1];
    let count = args[2];
    
    // In real implementation, read from file descriptor
    // For now, return dummy data
    Ok(count)
}

/// write(fd, buf, count)
fn syscall_write(args: &[usize]) -> SyscallResult<usize> {
    if args.len() < 3 {
        return Err(SyscallError::EINVAL);
    }
    
    let fd = args[0];
    let _buf_ptr = args[1];
    let count = args[2];
    
    // In real implementation, write to file descriptor
    Ok(count)
}

/// open(pathname, flags)
fn syscall_open(args: &[usize]) -> SyscallResult<usize> {
    if args.len() < 2 {
        return Err(SyscallError::EINVAL);
    }
    
    let _pathname_ptr = args[0];
    let _flags = args[1];
    
    // In real implementation, open file and return FD
    Ok(3) // Return first available FD
}

/// close(fd)
fn syscall_close(args: &[usize]) -> SyscallResult<usize> {
    if args.len() < 1 {
        return Err(SyscallError::EINVAL);
    }
    
    let fd = args[0];
    
    // In real implementation, close file descriptor
    Ok(0)
}

/// lseek(fd, offset, whence)
fn syscall_lseek(args: &[usize]) -> SyscallResult<usize> {
    if args.len() < 3 {
        return Err(SyscallError::EINVAL);
    }
    
    let _fd = args[0];
    let offset = args[1] as i64;
    let _whence = args[2];
    
    // In real implementation, seek in file
    Ok(offset as usize)
}

/// stat(pathname, statbuf)
fn syscall_stat(args: &[usize]) -> SyscallResult<usize> {
    if args.len() < 2 {
        return Err(SyscallError::EINVAL);
    }
    
    let _pathname_ptr = args[0];
    let _statbuf_ptr = args[1];
    
    // In real implementation, get file stats
    Ok(0)
}

/// fstat(fd, statbuf)
fn syscall_fstat(args: &[usize]) -> SyscallResult<usize> {
    if args.len() < 2 {
        return Err(SyscallError::EINVAL);
    }
    
    let _fd = args[0];
    let _statbuf_ptr = args[1];
    
    // In real implementation, get file stats by FD
    Ok(0)
}

/// getpid()
fn syscall_getpid(_args: &[usize]) -> SyscallResult<usize> {
    Ok(1) // Default PID
}

/// getuid()
fn syscall_getuid(_args: &[usize]) -> SyscallResult<usize> {
    Ok(0) // Root user
}

/// exit(status)
fn syscall_exit(_args: &[usize]) -> SyscallResult<usize> {
    // In real implementation, terminate process
    Ok(0)
}

/// fork()
fn syscall_fork(_args: &[usize]) -> SyscallResult<usize> {
    // In real implementation, create child process
    Ok(0) // Child returns 0
}

/// exec(pathname, argv)
fn syscall_exec(args: &[usize]) -> SyscallResult<usize> {
    if args.len() < 2 {
        return Err(SyscallError::EINVAL);
    }
    
    let _pathname_ptr = args[0];
    let _argv_ptr = args[1];
    
    // In real implementation, execute new program
    Ok(0)
}

/// brk(addr)
fn syscall_brk(args: &[usize]) -> SyscallResult<usize> {
    if args.len() < 1 {
        return Err(SyscallError::EINVAL);
    }
    
    let _addr = args[0];
    
    // In real implementation, set program break
    Ok(0)
}

/// mmap(addr, length, prot, flags, fd, offset)
fn syscall_mmap(_args: &[usize]) -> SyscallResult<usize> {
    // In real implementation, map memory
    Ok(0x10000000) // Return dummy address
}

/// munmap(addr, length)
fn syscall_munmap(_args: &[usize]) -> SyscallResult<usize> {
    Ok(0)
}

/// pipe(fildes)
fn syscall_pipe(_args: &[usize]) -> SyscallResult<usize> {
    // In real implementation, create pipe
    Ok(0)
}

/// dup(oldfd)
fn syscall_dup(args: &[usize]) -> SyscallResult<usize> {
    if args.len() < 1 {
        return Err(SyscallError::EINVAL);
    }
    
    let _oldfd = args[0];
    
    // In real implementation, duplicate file descriptor
    Ok(3)
}

/// getcwd(buf, size)
fn syscall_getcwd(args: &[usize]) -> SyscallResult<usize> {
    if args.len() < 2 {
        return Err(SyscallError::EINVAL);
    }
    
    let _buf_ptr = args[0];
    let _size = args[1];
    
    // In real implementation, get current working directory
    Ok(1) // Success
}

/// chdir(path)
fn syscall_chdir(args: &[usize]) -> SyscallResult<usize> {
    if args.len() < 1 {
        return Err(SyscallError::EINVAL);
    }
    
    let _path_ptr = args[0];
    
    // In real implementation, change directory
    Ok(0)
}

/// unlink(pathname)
fn syscall_unlink(args: &[usize]) -> SyscallResult<usize> {
    if args.len() < 1 {
        return Err(SyscallError::EINVAL);
    }
    
    let _pathname_ptr = args[0];
    
    // In real implementation, delete file
    Ok(0)
}

/// rename(oldpath, newpath)
fn syscall_rename(args: &[usize]) -> SyscallResult<usize> {
    if args.len() < 2 {
        return Err(SyscallError::EINVAL);
    }
    
    let _oldpath_ptr = args[0];
    let _newpath_ptr = args[1];
    
    // In real implementation, rename file
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syscall_table() {
        let table = SyscallTable::new();
        
        // Test that handlers are registered
        assert!(table.get_handler(0).is_some()); // read
        assert!(table.get_handler(1).is_some()); // write
        assert!(table.get_handler(2).is_some()); // open
        assert!(table.get_handler(3).is_some()); // close
    }

    #[test]
    fn test_error_strings() {
        assert_eq!(SyscallError::ENOENT.as_str(), "No such file or directory");
        assert_eq!(SyscallError::EACCESS.as_str(), "Permission denied");
    }

    #[test]
    fn test_read_syscall() {
        let result = syscall_read(&[0, 0x1000, 100]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 100);
    }

    #[test]
    fn test_write_syscall() {
        let result = syscall_write(&[1, 0x1000, 50]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 50);
    }

    #[test]
    fn test_getpid() {
        let result = syscall_getpid(&[]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }
}
