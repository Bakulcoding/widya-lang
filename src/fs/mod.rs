// ============================================================================
// File System VFS Layer - TAHAP 4.3
// ============================================================================
// Virtual File System untuk Widya OS dengan ext2/3/4 compatible layer
// Features:
// - VFS inode abstraction
// - File operations (read, write, open, close)
// - Directory operations
// - Mount points
// - Extensible filesystem drivers
// ============================================================================

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::atomic::{AtomicU64, Ordering};

/// Inode type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InodeType {
    File,
    Directory,
    Symlink,
    BlockDevice,
    CharDevice,
    Socket,
    NamedPipe,
}

/// File mode bits
pub const S_IFMT: u32 = 0o170000;
pub const S_IFREG: u32 = 0o100000;
pub const S_IFDIR: u32 = 0o040000;
pub const S_IFLNK: u32 = 0o120000;

pub const S_IRWXU: u32 = 0o0700;
pub const S_IRUSR: u32 = 0o0400;
pub const S_IWUSR: u32 = 0o0200;
pub const S_IXUSR: u32 = 0o0100;

/// File flags
pub const O_RDONLY: u32 = 0o0000;
pub const O_WRONLY: u32 = 0o0001;
pub const O_RDWR: u32 = 0o0002;
pub const O_CREAT: u32 = 0o0100;
pub const O_TRUNC: u32 = 0o1000;
pub const O_APPEND: u32 = 0o2000;

/// File descriptor
pub type FD = usize;

/// Inode identifier
pub type InodeID = u64;

/// VFS Inode
#[derive(Debug, Clone)]
pub struct Inode {
    pub id: InodeID,
    pub inode_type: InodeType,
    pub mode: u32,
    pub nlink: u32,
    pub uid: u32,
    pub gid: u32,
    pub size: u64,
    pub blocks: u64,
    pub atime: u64,
    pub mtime: u64,
    pub ctime: u64,
    pub data: Vec<u8>, // For small files
}

impl Inode {
    pub fn new_file(id: InodeID, mode: u32) -> Self {
        Self {
            id,
            inode_type: InodeType::File,
            mode: S_IFREG | mode,
            nlink: 1,
            uid: 0,
            gid: 0,
            size: 0,
            blocks: 0,
            atime: 0,
            mtime: 0,
            ctime: 0,
            data: Vec::new(),
        }
    }

    pub fn new_directory(id: InodeID, mode: u32) -> Self {
        Self {
            id,
            inode_type: InodeType::Directory,
            mode: S_IFDIR | mode,
            nlink: 2, // . and ..
            uid: 0,
            gid: 0,
            size: 0,
            blocks: 0,
            atime: 0,
            mtime: 0,
            ctime: 0,
            data: Vec::new(),
        }
    }

    pub fn is_file(&self) -> bool {
        self.inode_type == InodeType::File
    }

    pub fn is_directory(&self) -> bool {
        self.inode_type == InodeType::Directory
    }

    pub fn is_executable(&self) -> bool {
        (self.mode & S_IXUSR) != 0
    }

    pub fn is_readable(&self) -> bool {
        (self.mode & S_IRUSR) != 0
    }

    pub fn is_writable(&self) -> bool {
        (self.mode & S_IWUSR) != 0
    }
}

/// Directory entry
#[derive(Debug, Clone)]
pub struct DirEntry {
    pub inode_id: InodeID,
    pub name: String,
    pub inode_type: InodeType,
}

/// File operation structure
pub struct FileOperations {
    pub open: fn(&str, u32) -> SyscallResult<FD>,
    pub close: fn(FD) -> SyscallResult<()>,
    pub read: fn(FD, &mut [u8]) -> SyscallResult<usize>,
    pub write: fn(FD, &[u8]) -> SyscallResult<usize>,
    pub seek: fn(FD, i64, u32) -> SyscallResult<u64>,
    pub truncate: fn(FD, u64) -> SyscallResult<()>,
    pub stat: fn(FD) -> SyscallResult<FileStat>,
}

/// File operations implementation
impl FileOperations {
    pub fn new() -> Self {
        Self {
            open: default_open,
            close: default_close,
            read: default_read,
            write: default_write,
            seek: default_seek,
            truncate: default_truncate,
            stat: default_stat,
        }
    }
}

fn default_open(_path: &str, _flags: u32) -> SyscallResult<FD> {
    Ok(3)
}

fn default_close(_fd: FD) -> SyscallResult<()> {
    Ok(())
}

fn default_read(_fd: FD, _buf: &mut [u8]) -> SyscallResult<usize> {
    Ok(0)
}

fn default_write(_fd: FD, _buf: &[u8]) -> SyscallResult<usize> {
    Ok(0)
}

fn default_seek(_fd: FD, _offset: i64, _whence: u32) -> SyscallResult<u64> {
    Ok(0)
}

fn default_truncate(_fd: FD, _size: u64) -> SyscallResult<()> {
    Ok(())
}

fn default_stat(_fd: FD) -> SyscallResult<FileStat> {
    Ok(FileStat {
        inode: 0,
        size: 0,
        mode: 0,
        nlink: 0,
        uid: 0,
        gid: 0,
        atime: 0,
        mtime: 0,
        ctime: 0,
    })
}

/// File Stat
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

/// Mount point
#[derive(Debug, Clone)]
pub struct MountPoint {
    pub device_id: String,
    pub mount_point: String,
    pub filesystem_type: String,
    pub flags: u32,
}

/// VFS Root
pub struct VFSRoot {
    pub root_inode: Inode,
    pub inodes: RefCell<HashMap<InodeID, Rc<Inode>>>,
    pub next_inode_id: AtomicU64,
    pub mount_points: RefCell<Vec<MountPoint>>,
    pub open_files: RefCell<HashMap<FD, OpenFile>>,
}

/// Open file record
#[derive(Debug, Clone)]
pub struct OpenFile {
    pub fd: FD,
    pub inode_id: InodeID,
    pub position: u64,
    pub flags: u32,
}

impl VFSRoot {
    pub fn new() -> Self {
        let root = Inode::new_directory(0, S_IRWXU | S_IRWXG | S_IRWXO);
        
        Self {
            root_inode: root,
            inodes: RefCell::new(HashMap::new()),
            next_inode_id: AtomicU64::new(1),
            mount_points: RefCell::new(Vec::new()),
            open_files: RefCell::new(HashMap::new()),
        }
    }

    /// Create new inode
    pub fn create_inode(&self, inode_type: InodeType, mode: u32) -> Rc<Inode> {
        let id = self.next_inode_id.fetch_add(1, Ordering::SeqCst);
        let inode = match inode_type {
            InodeType::File => Inode::new_file(id, mode),
            InodeType::Directory => Inode::new_directory(id, mode),
            _ => Inode::new_file(id, mode),
        };
        
        let inode_rc = Rc::new(inode);
        self.inodes.borrow_mut().insert(id, Rc::clone(&inode_rc));
        
        inode_rc
    }

    /// Get inode by ID
    pub fn get_inode(&self, id: InodeID) -> Option<Rc<Inode>> {
        self.inodes.borrow().get(&id).cloned()
    }

    /// Find inode by path (simplified)
    pub fn lookup(&self, path: &str) -> Option<Rc<Inode>> {
        if path == "/" || path.is_empty() {
            return Some(Rc::new(self.root_inode.clone()));
        }
        
        // Simplified path lookup
        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        if parts.is_empty() {
            return None;
        }
        
        // Return root inode for now
        Some(Rc::new(self.root_inode.clone()))
    }

    /// Open file by path
    pub fn open_file(&self, path: &str, flags: u32) -> SyscallResult<FD> {
        let inode = self.lookup(path);
        
        if inode.is_none() && (flags & O_CREAT) == 0 {
            return Err(SyscallError::ENOENT);
        }
        
        let inode_id = inode.map(|i| i.id).unwrap_or(0);
        
        let mut open_files = self.open_files.borrow_mut();
        let fd = 3 + open_files.len();
        
        open_files.insert(fd, OpenFile {
            fd,
            inode_id,
            position: 0,
            flags,
        });
        
        Ok(fd)
    }

    /// Close file by FD
    pub fn close_file(&self, fd: FD) -> SyscallResult<()> {
        let mut open_files = self.open_files.borrow_mut();
        
        if open_files.remove(&fd).is_some() {
            Ok(())
        } else {
            Err(SyscallError::EINVAL)
        }
    }

    /// Read from file
    pub fn read_file(&self, fd: FD, buf: &mut [u8]) -> SyscallResult<usize> {
        let open_files = self.open_files.borrow();
        
        if let Some(file) = open_files.get(&fd) {
            let inode = self.get_inode(file.inode_id);
            
            if let Some(inode) = inode {
                let data = &inode.data;
                let count = std::cmp::min(buf.len(), data.len() - file.position as usize);
                
                if count > 0 {
                    buf[..count].copy_from_slice(&data[file.position as usize..file.position as usize + count]);
                    Ok(count)
                } else {
                    Ok(0)
                }
            } else {
                Err(SyscallError::EINVAL)
            }
        } else {
            Err(SyscallError::EINVAL)
        }
    }

    /// Write to file
    pub fn write_file(&self, fd: FD, buf: &[u8]) -> SyscallResult<usize> {
        let mut open_files = self.open_files.borrow_mut();
        
        if let Some(file) = open_files.get_mut(&fd) {
            let mut inodes = self.inodes.borrow_mut();
            
            if let Some(inode) = inodes.get_mut(&file.inode_id) {
                let pos = file.position as usize;
                let end = pos + buf.len();
                
                if end > inode.data.len() {
                    inode.data.resize(end, 0);
                }
                
                inode.data[pos..end].copy_from_slice(buf);
                inode.size = inode.data.len() as u64;
                inode.mtime = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                
                file.position += buf.len() as u64;
                Ok(buf.len())
            } else {
                Err(SyscallError::EINVAL)
            }
        } else {
            Err(SyscallError::EINVAL)
        }
    }

    /// Seek in file
    pub fn seek_file(&self, fd: FD, offset: i64, whence: u32) -> SyscallResult<u64> {
        let mut open_files = self.open_files.borrow_mut();
        
        if let Some(file) = open_files.get_mut(&fd) {
            let current_pos = file.position as i64;
            
            file.position = match whence as i32 {
                0 => offset.max(0) as u64, // SEEK_SET
                1 => (current_pos + offset).max(0) as u64, // SEEK_CUR
                2 => 0, // SEEK_END (simplified)
                _ => return Err(SyscallError::EINVAL),
            };
            
            Ok(file.position)
        } else {
            Err(SyscallError::EINVAL)
        }
    }

    /// Mount filesystem
    pub fn mount(&self, device: &str, mount_point: &str, fs_type: &str, flags: u32) -> SyscallResult<()> {
        let mut mount_points = self.mount_points.borrow_mut();
        
        mount_points.push(MountPoint {
            device_id: device.to_string(),
            mount_point: mount_point.to_string(),
            filesystem_type: fs_type.to_string(),
            flags,
        });
        
        Ok(())
    }

    /// Unmount filesystem
    pub fn umount(&self, mount_point: &str) -> SyscallResult<()> {
        let mut mount_points = self.mount_points.borrow_mut();
        
        let len = mount_points.len();
        mount_points.retain(|m| m.mount_point != mount_point);
        
        if mount_points.len() < len {
            Ok(())
        } else {
            Err(SyscallError::EINVAL)
        }
    }
}

/// Simple in-memory filesystem driver
pub struct InMemoryFS;

impl InMemoryFS {
    pub fn new() -> Self {
        Self
    }
    
    pub fn init(&self, vfs: &VFSRoot) {
        // Create basic directory structure
        let bin = vfs.create_inode(InodeType::Directory, S_IRWXU | S_IRGRP | S_IXGRP | S_IROTH | S_IXOTH);
        let _etc = vfs.create_inode(InodeType::Directory, S_IRWXU | S_IRGRP | S_IROTH);
        let _home = vfs.create_inode(InodeType::Directory, S_IRWXU);
        
        // Create basic files
        let _hello = {
            let mut inode = vfs.create_inode(InodeType::File, S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH);
            let mut inodes = vfs.inodes.borrow_mut();
            if let Some(ref mut i) = inodes.get_mut(&inode.id) {
                i.data = b"Hello, World!\n".to_vec();
                i.size = i.data.len() as u64;
            }
            inode
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vfs_creation() {
        let vfs = VFSRoot::new();
        
        assert_eq!(vfs.root_inode.inode_type, InodeType::Directory);
        assert_eq!(vfs.root_inode.id, 0);
    }

    #[test]
    fn test_create_inode() {
        let vfs = VFSRoot::new();
        
        let inode = vfs.create_inode(InodeType::File, S_IRWXU);
        
        assert_eq!(inode.inode_type, InodeType::File);
        assert!(inode.id > 0);
    }

    #[test]
    fn test_file_operations() {
        let vfs = VFSRoot::new();
        
        let fd = vfs.open_file("/test.txt", O_CREAT | O_RDWR).unwrap();
        
        let result = vfs.write_file(fd, b"Hello");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5);
        
        let mut buf = [0u8; 10];
        let result = vfs.read_file(fd, &mut buf);
        assert!(result.is_ok());
        assert_eq!(&buf[..5], b"Hello");
    }

    #[test]
    fn test_mount() {
        let vfs = VFSRoot::new();
        
        vfs.mount("/dev/sda1", "/mnt/data", "ext4", 0).unwrap();
        
        let mount_points = vfs.mount_points.borrow();
        assert_eq!(mount_points.len(), 1);
        assert_eq!(mount_points[0].mount_point, "/mnt/data");
    }

    #[test]
    fn test_seek() {
        let vfs = VFSRoot::new();
        
        let fd = vfs.open_file("/test.txt", O_CREAT | O_RDWR).unwrap();
        let _ = vfs.write_file(fd, b"Hello World");
        
        let pos = vfs.seek_file(fd, 6, 0).unwrap(); // SEEK_SET
        assert_eq!(pos, 6);
        
        let mut buf = [0u8; 5];
        let _ = vfs.read_file(fd, &mut buf);
        assert_eq!(&buf, b"World");
    }
}
