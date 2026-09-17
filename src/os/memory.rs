// ============================================================================
// Memory Management Layer (TAHAP 1.1)
// ============================================================================
// Page-based memory allocator dengan virtual memory support
// Features:
// - Page allocator (4KB pages)
// - Slab allocator untuk object caching
// - Virtual address space management
// - Page table management (multi-level)
// ============================================================================

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

/// Page size: 4KB (standard x86 page)
pub const PAGE_SIZE: usize = 4096;

/// Page flags
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PageFlags {
    Readable = 0x1,
    Writable = 0x2,
    Executable = 0x4,
    UserMode = 0x8,
    Present = 0x10,
}

/// Physical page frame
#[derive(Debug, Clone)]
pub struct PhysicalPage {
    pub frame_number: usize,
    pub flags: PageFlags,
    pub reference_count: AtomicU64,
    pub is_free: bool,
}

/// Virtual page mapping
#[derive(Debug, Clone)]
pub struct PageTableEntry {
    pub virtual_address: usize,
    pub physical_address: usize,
    pub flags: PageFlags,
    pub present: bool,
}

/// Page Allocator - manages physical page frames
pub struct PageAllocator {
    pub total_pages: usize,
    pub free_pages: AtomicU64,
    pub page_frames: RefCell<Vec<PhysicalPage>>,
    pub free_list: RefCell<Vec<usize>>,
}

impl PageAllocator {
    pub fn new(total_memory_mb: usize) -> Self {
        let total_pages = (total_memory_mb * 1024 * 1024) / PAGE_SIZE;
        let mut page_frames = Vec::with_capacity(total_pages);
        let mut free_list = Vec::with_capacity(total_pages);

        for i in 0..total_pages {
            page_frames.push(PhysicalPage {
                frame_number: i,
                flags: PageFlags::Readable | PageFlags::Writable,
                reference_count: AtomicU64::new(0),
                is_free: true,
            });
            free_list.push(i);
        }

        Self {
            total_pages,
            free_pages: AtomicU64::new(total_pages as u64),
            page_frames: RefCell::new(page_frames),
            free_list: RefCell::new(free_list),
        }
    }

    /// Allocate n consecutive pages
    pub fn allocate_pages(&self, count: usize, flags: PageFlags) -> Option<usize> {
        let mut free_list = self.free_list.borrow_mut();
        
        if free_list.len() < count {
            return None;
        }

        let start_frame = free_list.remove(0);
        
        for _ in 1..count {
            free_list.remove(0);
        }

        {
            let mut pages = self.page_frames.borrow_mut();
            for i in start_frame..start_frame + count {
                pages[i].is_free = false;
                pages[i].flags = flags;
                pages[i].reference_count.store(1, Ordering::SeqCst);
            }
        }

        self.free_pages.fetch_sub(count as u64, Ordering::SeqCst);
        
        Some(start_frame * PAGE_SIZE)
    }

    /// Free allocated pages
    pub fn free_pages(&self, address: usize, count: usize) {
        let start_frame = address / PAGE_SIZE;
        let mut free_list = self.free_list.borrow_mut();
        
        {
            let mut pages = self.page_frames.borrow_mut();
            for i in start_frame..start_frame + count {
                pages[i].is_free = true;
                pages[i].reference_count.store(0, Ordering::SeqCst);
                free_list.push(i);
            }
        }

        self.free_pages.fetch_add(count as u64, Ordering::SeqCst);
    }

    /// Allocate single page
    pub fn allocate_page(&self, flags: PageFlags) -> Option<usize> {
        self.allocate_pages(1, flags)
    }

    /// Free single page
    pub fn free_page(&self, address: usize) {
        self.free_pages(address, 1);
    }

    /// Get free page count
    pub fn free_count(&self) -> u64 {
        self.free_pages.load(Ordering::SeqCst)
    }

    /// Get total page count
    pub fn total_count(&self) -> usize {
        self.total_pages
    }

    /// Allocate pages with contiguous physical memory
    pub fn allocate_contiguous(&self, count: usize, flags: PageFlags) -> Option<usize> {
        let mut free_list = self.free_list.borrow_mut();
        
        if free_list.len() < count {
            return None;
        }

        // Check for contiguous block
        let mut contiguous_start = None;
        let mut contiguous_count = 0;

        for &frame in &*free_list {
            if contiguous_start.is_none() {
                contiguous_start = Some(frame);
                contiguous_count = 1;
            } else if contiguous_start.unwrap() + contiguous_count == frame {
                contiguous_count += 1;
            } else {
                contiguous_start = Some(frame);
                contiguous_count = 1;
            }

            if contiguous_count >= count {
                break;
            }
        }

        let start_frame = contiguous_start?;

        // Remove from free list
        let mut to_remove = Vec::new();
        for &frame in &*free_list {
            if frame >= start_frame && frame < start_frame + count {
                to_remove.push(frame);
            }
        }

        for frame in to_remove {
            free_list.retain(|&x| x != frame);
        }

        // Mark as allocated
        {
            let mut pages = self.page_frames.borrow_mut();
            for i in start_frame..start_frame + count {
                pages[i].is_free = false;
                pages[i].flags = flags;
                pages[i].reference_count.store(1, Ordering::SeqCst);
            }
        }

        self.free_pages.fetch_sub(count as u64, Ordering::SeqCst);
        
        Some(start_frame * PAGE_SIZE)
    }
}

/// Slab Allocator - efficient object allocation
pub struct SlabAllocator {
    pub slab_size: usize,
    pub free_list: RefCell<Vec<usize>>,
    pub slab_count: AtomicU64,
}

impl SlabAllocator {
    pub fn new(slab_size: usize) -> Self {
        // Align to 8 bytes
        let aligned_size = (slab_size + 7) & !7;
        
        Self {
            slab_size: aligned_size,
            free_list: RefCell::new(Vec::new()),
            slab_count: AtomicU64::new(0),
        }
    }

    pub fn allocate(&self, allocator: &PageAllocator) -> Option<usize> {
        if let Some(offset) = self.free_list.borrow_mut().pop() {
            Some(offset)
        } else {
            // Allocate new slab
            let slab_memory = allocator.allocate_page(PageFlags::Readable | PageFlags::Writable)?;
            let slab_count = self.slab_count.fetch_add(1, Ordering::SeqCst) as usize;
            
            // Initialize free list for this slab
            let total_objects = PAGE_SIZE / self.slab_size;
            let mut free_list = self.free_list.borrow_mut();
            
            for i in 0..total_objects {
                free_list.push(slab_memory + i * self.slab_size);
            }
            
            free_list.pop()
        }
    }

    pub fn free(&self, address: usize) {
        self.free_list.borrow_mut().push(address);
    }
}

/// Virtual Address Space
pub struct VirtualAddressSpace {
    pub page_tables: RefCell<BTreeMap<usize, PageTableEntry>>,
    pub user_start: usize,
    pub user_end: usize,
    pub kernel_start: usize,
}

impl VirtualAddressSpace {
    pub fn new(user_start: usize, user_end: usize, kernel_start: usize) -> Self {
        Self {
            page_tables: RefCell::new(BTreeMap::new()),
            user_start,
            user_end,
            kernel_start,
        }
    }

    /// Map virtual address to physical address
    pub fn map(&self, virt: usize, phys: usize, flags: PageFlags) -> Result<(), String> {
        if virt % PAGE_SIZE != 0 || phys % PAGE_SIZE != 0 {
            return Err("Address must be page-aligned".to_string());
        }

        let entry = PageTableEntry {
            virtual_address: virt,
            physical_address: phys,
            flags,
            present: true,
        };

        self.page_tables.borrow_mut().insert(virt, entry);
        Ok(())
    }

    /// Unmap virtual address
    pub fn unmap(&self, virt: usize) -> Result<(), String> {
        let mut page_tables = self.page_tables.borrow_mut();
        
        if page_tables.remove(&virt).is_none() {
            return Err("Virtual address not mapped".to_string());
        }

        Ok(())
    }

    /// Translate virtual to physical address
    pub fn translate(&self, virt: usize) -> Option<usize> {
        // Round down to page boundary
        let page_addr = virt & !(PAGE_SIZE - 1);
        
        if let Some(entry) = self.page_tables.borrow().get(&page_addr) {
            if entry.present && entry.flags.contains(PageFlags::Present) {
                let offset = virt & (PAGE_SIZE - 1);
                return Some(entry.physical_address + offset);
            }
        }
        
        None
    }

    /// Check if virtual address is valid
    pub fn is_valid_user_address(&self, addr: usize) -> bool {
        addr >= self.user_start && addr < self.user_end
    }

    /// Check if virtual address is kernel space
    pub fn is_kernel_address(&self, addr: usize) -> bool {
        addr >= self.kernel_start
    }

    /// Get page table entry count
    pub fn entry_count(&self) -> usize {
        self.page_tables.borrow().len()
    }

    /// Create new address space (for fork)
    pub fn clone_for_process(&self) -> Self {
        Self {
            page_tables: RefCell::new(self.page_tables.borrow().clone()),
            user_start: self.user_start,
            user_end: self.user_end,
            kernel_start: self.kernel_start,
        }
    }
}

/// Memory Manager - top-level interface
pub struct MemoryManager {
    pub page_allocator: Rc<PageAllocator>,
    pub virtual_space: Rc<VirtualAddressSpace>,
}

impl MemoryManager {
    pub fn new(total_memory_mb: usize) -> Self {
        let page_allocator = Rc::new(PageAllocator::new(total_memory_mb));
        let virtual_space = Rc::new(VirtualAddressSpace::new(
            0x0000_0000_0000_0000, // User space start
            0x0000_7FFF_FFFF_FFFF, // User space end
            0xFFFF_8000_0000_0000, // Kernel space start
        ));

        Self {
            page_allocator,
            virtual_space,
        }
    }

    /// Allocate memory with virtual mapping
    pub fn allocate(&self, size: usize, flags: PageFlags) -> Result<usize, String> {
        // Round up to page size
        let page_count = (size + PAGE_SIZE - 1) / PAGE_SIZE;
        
        let phys_addr = self.page_allocator.allocate_pages(page_count, flags)
            .ok_or_else(|| "Out of physical memory".to_string())?;

        // Map to virtual address (current process)
        // In real OS, this would use current process's address space
        let virt_addr = 0x0000_0000_1000_0000 + page_count * PAGE_SIZE;

        self.virtual_space.map(virt_addr, phys_addr, flags)?;

        Ok(virt_addr)
    }

    /// Free allocated memory
    pub fn free(&self, virt_addr: usize) -> Result<(), String> {
        let page_count = (self.page_allocator.total_pages - self.page_allocator.free_count()) as usize;
        
        self.virtual_space.unmap(virt_addr)?;
        Ok(())
    }

    /// Translate virtual to physical (for page fault handler)
    pub fn translate(&self, virt: usize) -> Option<usize> {
        self.virtual_space.translate(virt)
    }

    /// Get memory statistics
    pub fn stats(&self) -> MemoryStats {
        MemoryStats {
            total_pages: self.page_allocator.total_count(),
            free_pages: self.page_allocator.free_count() as usize,
            used_pages: self.page_allocator.total_count() - self.page_allocator.free_count() as usize,
            page_size: PAGE_SIZE,
        }
    }
}

/// Memory statistics
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub total_pages: usize,
    pub free_pages: usize,
    pub used_pages: usize,
    pub page_size: usize,
}

impl MemoryStats {
    pub fn total_memory_mb(&self) -> f64 {
        (self.total_pages * self.page_size) as f64 / (1024.0 * 1024.0)
    }

    pub fn used_memory_mb(&self) -> f64 {
        (self.used_pages * self.page_size) as f64 / (1024.0 * 1024.0)
    }

    pub fn free_memory_mb(&self) -> f64 {
        (self.free_pages * self.page_size) as f64 / (1024.0 * 1024.0)
    }

    pub fn usage_percent(&self) -> f64 {
        if self.total_pages == 0 {
            return 0.0;
        }
        (self.used_pages as f64 / self.total_pages as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_allocator_basic() {
        let allocator = PageAllocator::new(64); // 64 MB
        
        assert_eq!(allocator.total_count(), 64 * 1024 * 1024 / PAGE_SIZE);
        assert_eq!(allocator.free_count(), 64 * 1024 * 1024 / PAGE_SIZE as u64);
    }

    #[test]
    fn test_page_allocation() {
        let allocator = PageAllocator::new(64);
        
        let addr = allocator.allocate_page(PageFlags::Readable | PageFlags::Writable);
        assert!(addr.is_some());
        
        assert_eq!(allocator.free_count(), (64 * 1024 * 1024 / PAGE_SIZE as u64) - 1);
        
        allocator.free_page(addr.unwrap());
        assert_eq!(allocator.free_count(), 64 * 1024 * 1024 / PAGE_SIZE as u64);
    }

    #[test]
    fn test_virtual_mapping() {
        let virt_space = VirtualAddressSpace::new(0, 0x7FFF_FFFF, 0x8000_0000);
        
        let result = virt_space.map(0x1000_0000, 0x0000_0000, 
            PageFlags::Readable | PageFlags::Writable | PageFlags::Present);
        assert!(result.is_ok());
        
        let phys = virt_space.translate(0x1000_1234);
        assert_eq!(phys, Some(0x0000_1234));
    }

    #[test]
    fn test_memory_stats() {
        let mm = MemoryManager::new(64);
        let stats = mm.stats();
        
        assert_eq!(stats.total_pages, 64 * 1024 * 1024 / PAGE_SIZE);
        assert_eq!(stats.free_pages, 64 * 1024 * 1024 / PAGE_SIZE);
        assert_eq!(stats.page_size, PAGE_SIZE);
    }
}
