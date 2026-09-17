// ============================================================================
// Virtual Filesystem Layer - TAHAP 4.3
// ============================================================================
// VFS dengan mount points, inodes, dan file operations
// Features:
// - VFS superblock & inode structure
// - Mount point management
// - File operations abstraction
// - Directory operations
// ============================================================================

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

// ============================================================================
// VFS Structures
// ============================================================================

/// File type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileType {
    Regular,
    Directory,
    Symlink,
    BlockDevice,
    CharDevice,
    FIFO,
    Socket,
}

impl FileType {
    pub fn to_mode(&self) -> u16 {
        match self {
            FileType::Regular => 0o100000,
            FileType::Directory => 0o040000,
            FileType::Symlink => 0o120000,
            FileType::BlockDevice => 0o060000,
            FileType::CharDevice => 0o020000,
            FileType::FIFO => 0o010000,
            FileType::Socket => 0o140000,
        }
    }
}

/// Inode identifier
pub type InodeID = u64;

/// File descriptor
pub type FileDescriptor = usize;

/// File access flags
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OpenFlags {
    pub read: bool,
    pub write: bool,
    pub append: bool,
    pub truncate: bool,
    pub create: bool,
    pub exclusive: bool,
}

impl OpenFlags {
    pub fn from_raw(flags: u32) -> Self {
        Self {
            read: (flags & 0o1) != 0,
            write: (flags & 0o2) != 0,
            append: (flags & 0o4) != 0,
            truncate: (flags & 0o8) != 0,
            create: (flags & 0o16) != 0,
            exclusive: (flags & 0o32) != 0,
        }
    }

    pub fn to_raw(&self) -> u32 {
        let mut flags = 0;
        if self.read { flags |= 0o1; }
        if self.write { flags |= 0o2; }
        if self.append { flags |= 0o4; }
        if self.truncate { flags |= 0o8; }
        if self.create { flags |= 0o16; }
        if self.exclusive { flags |= 0o32; }
        flags
    }
}

/// Inode metadata
#[derive(Debug, Clone)]
pub struct InodeMetadata {
    pub inode: InodeID,
    pub file_type: FileType,
    pub permissions: u16,
    pub owner: u32,
    pub group: u32,
    pub size: u64,
    pub block_count: u64,
    pub access_time: u64,
    pub modify_time: u64,
    pub change_time: u64,
    pub link_count: u32,
}

impl InodeMetadata {
    pub fn new(inode: InodeID, file_type: FileType, permissions: u16) -> Self {
        Self {
            inode,
            file_type,
            permissions,
            owner: 0, // root
            group: 0, // root
            size: 0,
            block_count: 0,
            access_time: 0,
            modify_time: 0,
            change_time: 0,
            link_count: 1,
        }
    }
}

/// Directory entry
#[derive(Debug, Clone)]
pub struct DirEntry {
    pub name: String,
    pub inode: InodeID,
    pub file_type: FileType,
}

/// VFS inode operations
pub trait InodeOps {
    fn lookup(&self, name: &str) -> Result<InodeID, VfsError>;
    fn get_metadata(&self) -> Result<InodeMetadata, VfsError>;
    fn read(&self, offset: u64, buffer: &mut [u8]) -> Result<usize, VfsError>;
    fn write(&self, offset: u64, buffer: &[u8]) -> Result<usize, VfsError>;
    fn truncate(&self, size: u64) -> Result<(), VfsError>;
    fn readdir(&self) -> Result<Vec<DirEntry>, VfsError>;
    fn create(&mut self, name: &str, file_type: FileType, permissions: u16) -> Result<InodeID, VfsError>;
    fn unlink(&mut self, name: &str) -> Result<(), VfsError>;
    fn mkdir(&mut self, name: &str, permissions: u16) -> Result<InodeID, VfsError>;
    fn rmdir(&mut self, name: &str) -> Result<(), VfsError>;
    fn rename(&mut self, old_name: &str, new_name: &str) -> Result<(), VfsError>;
}

/// VFS superblock operations
pub trait SuperblockOps {
    fn alloc_inode(&mut self, file_type: FileType, permissions: u16) -> Result<InodeID, VfsError>;
    fn free_inode(&mut self, inode: InodeID) -> Result<(), VfsError>;
    fn sync(&self) -> Result<(), VfsError>;
}

/// VFS filesystem type
#[derive(Debug, Clone)]
pub struct FilesystemType {
    pub name: String,
    pub mount: fn(path: &str) -> Result<Arc<RwLock<dyn SuperblockOps>>, VfsError>,
}

/// Mount point
#[derive(Debug, Clone)]
pub struct MountPoint {
    pub path: PathBuf,
    pub superblock: Arc<RwLock<dyn SuperblockOps>>,
    pub root_inode: InodeID,
}

/// VFS error
#[derive(Debug, Clone, PartialEq)]
pub enum VfsError {
    NotFound,
    PermissionDenied,
    AlreadyExists,
    NotADirectory,
    IsDirectory,
    NotEmpty,
    InvalidArgument,
    IOFailure(String),
    NotSupported,
    OutOfMemory,
}

impl VfsError {
    pub fn as_str(&self) -> &'static str {
        match self {
            VfsError::NotFound => "File or directory not found",
            VfsError::PermissionDenied => "Permission denied",
            VfsError::AlreadyExists => "File already exists",
            VfsError::NotADirectory => "Not a directory",
            VfsError::IsDirectory => "Is a directory",
            VfsError::NotEmpty => "Directory not empty",
            VfsError::InvalidArgument => "Invalid argument",
            VfsError::IOFailure(msg) => msg.as_str(),
            VfsError::NotSupported => "Operation not supported",
            VfsError::OutOfMemory => "Out of memory",
        }
    }
}

// ============================================================================
// VFS Manager
// ============================================================================

pub struct VirtualFilesystem {
    mount_points: RwLock<HashMap<PathBuf, MountPoint>>,
    inode_cache: RwLock<HashMap<InodeID, Arc<RwLock<dyn InodeOps>>>>,
    filesystem_types: RwLock<HashMap<String, FilesystemType>>,
    next_inode: RwLock<InodeID>,
}

impl VirtualFilesystem {
    pub fn new() -> Self {
        Self {
            mount_points: RwLock::new(HashMap::new()),
            inode_cache: RwLock::new(HashMap::new()),
            filesystem_types: RwLock::new(HashMap::new()),
            next_inode: RwLock::new(1), // Start from 1 (0 is reserved)
        }
    }

    /// Register a filesystem type
    pub fn register_filesystem(&self, fs_type: FilesystemType) -> Result<(), VfsError> {
        let mut types = self.filesystem_types.write()
            .map_err(|_| VfsError::IOFailure("Failed to lock filesystem types".to_string()))?;
        
        if types.contains_key(&fs_type.name) {
            return Err(VfsError::AlreadyExists);
        }
        
        types.insert(fs_type.name.clone(), fs_type);
        Ok(())
    }

    /// Mount a filesystem
    pub fn mount(&self, fs_type: &str, source: &str, target: &str) -> Result<(), VfsError> {
        let types = self.filesystem_types.read()
            .map_err(|_| VfsError::IOFailure("Failed to lock filesystem types".to_string()))?;
        
        let fs_info = types.get(fs_type)
            .ok_or(VfsError::NotFound)?;
        
        let mount_fn = fs_info.mount;
        let superblock = mount_fn(source)?;
        
        let mount_point = MountPoint {
            path: PathBuf::from(target),
            superblock: Arc::clone(&superblock),
            root_inode: 1, // Root inode ID
        };
        
        let mut mounts = self.mount_points.write()
            .map_err(|_| VfsError::IOFailure("Failed to lock mount points".to_string()))?;
        
        mounts.insert(PathBuf::from(target), mount_point);
        Ok(())
    }

    /// Unmount a filesystem
    pub fn unmount(&self, target: &str) -> Result<(), VfsError> {
        let mut mounts = self.mount_points.write()
            .map_err(|_| VfsError::IOFailure("Failed to lock mount points".to_string()))?;
        
        let target_path = PathBuf::from(target);
        
        if mounts.remove(&target_path).is_some() {
            Ok(())
        } else {
            Err(VfsError::NotFound)
        }
    }

    /// Get mount point for path
    pub fn get_mount_point(&self, path: &str) -> Option<MountPoint> {
        let mounts = self.mount_points.read().ok()?;
        
        let mut best_match: Option<(&PathBuf, &MountPoint)> = None;
        let target_path = PathBuf::from(path);
        
        for (mount_path, mount) in mounts.iter() {
            if target_path.starts_with(mount_path) {
                if let Some((best_path, _)) = best_match {
                    if mount_path.starts_with(best_path) {
                        best_match = Some((mount_path, mount));
                    }
                } else {
                    best_match = Some((mount_path, mount));
                }
            }
        }
        
        best_match.map(|(_, mount)| mount.clone())
    }

    /// Lookup inode by path
    pub fn lookup(&self, path: &str) -> Result<Arc<RwLock<dyn InodeOps>>, VfsError> {
        let mount_point = self.get_mount_point(path)
            .ok_or(VfsError::NotFound)?;
        
        // For now, return dummy inode
        let inode = self.create_dummy_inode()?;
        Ok(inode)
    }

    /// Open file
    pub fn open(&self, path: &str, flags: OpenFlags) -> Result<FileDescriptor, VfsError> {
        // Lookup inode
        let inode = self.lookup(path)?;
        
        // Check permissions
        let metadata = inode.read()
            .map_err(|_| VfsError::IOFailure("Failed to lock inode".to_string()))?
            .get_metadata()?;
        
        // In real implementation, check file permissions
        if flags.write && metadata.permissions & 0o200 == 0 {
            return Err(VfsError::PermissionDenied);
        }
        
        // Create file descriptor
        let fd = self.allocate_fd();
        Ok(fd)
    }

    /// Close file
    pub fn close(&self, fd: FileDescriptor) -> Result<(), VfsError> {
        // In real implementation, release file resources
        Ok(())
    }

    /// Read from file
    pub fn read(&self, fd: FileDescriptor, buffer: &mut [u8], offset: u64) -> Result<usize, VfsError> {
        // In real implementation, get inode from fd and read
        if buffer.len() > 0 {
            buffer[0] = b'A'; // Dummy data
            Ok(1)
        } else {
            Ok(0)
        }
    }

    /// Write to file
    pub fn write(&self, fd: FileDescriptor, buffer: &[u8], offset: u64) -> Result<usize, VfsError> {
        // In real implementation, get inode from fd and write
        Ok(buffer.len())
    }

    /// Create directory
    pub fn mkdir(&self, path: &str, permissions: u16) -> Result<(), VfsError> {
        let mount_point = self.get_mount_point(path)
            .ok_or(VfsError::NotFound)?;
        
        // In real implementation, create directory
        Ok(())
    }

    /// Remove directory
    pub fn rmdir(&self, path: &str) -> Result<(), VfsError> {
        let mount_point = self.get_mount_point(path)
            .ok_or(VfsError::NotFound)?;
        
        // In real implementation, remove directory
        Ok(())
    }

    /// Delete file
    pub fn unlink(&self, path: &str) -> Result<(), VfsError> {
        let mount_point = self.get_mount_point(path)
            .ok_or(VfsError::NotFound)?;
        
        // In real implementation, delete file
        Ok(())
    }

    /// Rename file
    pub fn rename(&self, old_path: &str, new_path: &str) -> Result<(), VfsError> {
        let old_mount = self.get_mount_point(old_path)
            .ok_or(VfsError::NotFound)?;
        
        let new_mount = self.get_mount_point(new_path)
            .ok_or(VfsError::NotFound)?;
        
        if old_mount.root_inode != new_mount.root_inode {
            return Err(VfsError::NotSupported); // Cross-filesystem rename not supported
        }
        
        // In real implementation, rename file
        Ok(())
    }

    // Helper methods
    fn allocate_fd(&self) -> FileDescriptor {
        // In real implementation, allocate from pool
        3 // Start from 3 (0,1,2 reserved for stdin, stdout, stderr)
    }

    fn create_dummy_inode(&self) -> Result<Arc<RwLock<dyn InodeOps>>, VfsError> {
        let inode = DummyInode::new(self.allocate_inode_id());
        Ok(Arc::new(RwLock::new(inode)))
    }

    fn allocate_inode_id(&self) -> InodeID {
        let mut next = self.next_inode.write().unwrap();
        let id = *next;
        *next += 1;
        id
    }
}

impl Default for VirtualFilesystem {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Dummy Inode Implementation (for testing)
// ============================================================================

struct DummyInode {
    metadata: InodeMetadata,
}

impl DummyInode {
    fn new(inode_id: InodeID) -> Self {
        Self {
            metadata: InodeMetadata::new(inode_id, FileType::Regular, 0o644),
        }
    }
}

impl InodeOps for DummyInode {
    fn lookup(&self, _name: &str) -> Result<InodeID, VfsError> {
        Err(VfsError::NotFound)
    }

    fn get_metadata(&self) -> Result<InodeMetadata, VfsError> {
        Ok(self.metadata.clone())
    }

    fn read(&self, _offset: u64, buffer: &mut [u8]) -> Result<usize, VfsError> {
        if buffer.len() > 0 {
            buffer[0] = b'T';
            Ok(1)
        } else {
            Ok(0)
        }
    }

    fn write(&self, _offset: u64, buffer: &[u8]) -> Result<usize, VfsError> {
        Ok(buffer.len())
    }

    fn truncate(&self, _size: u64) -> Result<(), VfsError> {
        Ok(())
    }

    fn readdir(&self) -> Result<Vec<DirEntry>, VfsError> {
        if self.metadata.file_type == FileType::Directory {
            Ok(vec![])
        } else {
            Err(VfsError::NotADirectory)
        }
    }

    fn create(&mut self, _name: &str, _file_type: FileType, _permissions: u16) -> Result<InodeID, VfsError> {
        Err(VfsError::NotADirectory)
    }

    fn unlink(&mut self, _name: &str) -> Result<(), VfsError> {
        Err(VfsError::NotADirectory)
    }

    fn mkdir(&mut self, _name: &str, _permissions: u16) -> Result<InodeID, VfsError> {
        Err(VfsError::NotADirectory)
    }

    fn rmdir(&mut self, _name: &str) -> Result<(), VfsError> {
        Err(VfsError::NotADirectory)
    }

    fn rename(&mut self, _old_name: &str, _new_name: &str) -> Result<(), VfsError> {
        Err(VfsError::NotADirectory)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filetype_mode() {
        assert_eq!(FileType::Regular.to_mode(), 0o100000);
        assert_eq!(FileType::Directory.to_mode(), 0o040000);
        assert_eq!(FileType::Symlink.to_mode(), 0o120000);
    }

    #[test]
    fn test_open_flags() {
        let flags = OpenFlags {
            read: true,
            write: false,
            append: true,
            truncate: false,
            create: true,
            exclusive: false,
        };
        
        let raw = flags.to_raw();
        let restored = OpenFlags::from_raw(raw);
        
        assert_eq!(flags.read, restored.read);
        assert_eq!(flags.append, restored.append);
        assert_eq!(flags.create, restored.create);
    }

    #[test]
    fn test_vfs_initialization() {
        let vfs = VirtualFilesystem::new();
        // Should not panic
    }

    #[test]
    fn test_dummy_inode() {
        let inode = DummyInode::new(1);
        let metadata = inode.get_metadata().unwrap();
        
        assert_eq!(metadata.inode, 1);
        assert_eq!(metadata.file_type, FileType::Regular);
        assert_eq!(metadata.permissions, 0o644);
    }
}