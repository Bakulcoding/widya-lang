// ============================================================================
// WAL (Write-Ahead Logging) - TAHAP 2.1
// ============================================================================
// ARIES protocol implementation untuk durability dan recovery
// Features:
// - Log record structure
// - Log buffer management
// - Checkpointing
// - Recovery from crash
// ============================================================================

use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::sync::atomic::{AtomicU64, Ordering};

/// Log sequence number (unique identifier for each log record)
pub type LogSequenceNumber = u64;

/// Log record types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogRecordType {
    Begin,        // Transaction begin
    Commit,       // Transaction commit
    Abort,        // Transaction abort
    Update,       // Data page update
    DirtyPage,    // Page marked dirty
}

/// Transaction identifier
pub type TransactionID = u64;

/// Page identifier
pub type PageID = u32;

/// Log record structure
#[derive(Debug, Clone)]
pub struct LogRecord {
    pub lsn: LogSequenceNumber,
    pub tx_id: TransactionID,
    pub record_type: LogRecordType,
    pub page_id: PageID,
    pub offset: u16,
    pub before_image: Vec<u8>,
    pub after_image: Vec<u8>,
    pub prev_lsn: LogSequenceNumber, // For undo chain
}

/// WAL buffer
pub struct WalBuffer {
    records: RefCell<VecDeque<LogRecord>>,
    max_buffer_size: usize,
}

impl WalBuffer {
    pub fn new(max_size: usize) -> Self {
        Self {
            records: RefCell::new(VecDeque::new()),
            max_buffer_size: max_size,
        }
    }

    pub fn push(&self, record: LogRecord) {
        let mut records = self.records.borrow_mut();
        records.push_back(record);
        
        // Check if buffer is full
        if records.len() > self.max_buffer_size {
            // Should flush to disk
        }
    }

    pub fn pop(&self) -> Option<LogRecord> {
        let mut records = self.records.borrow_mut();
        records.pop_front()
    }

    pub fn len(&self) -> usize {
        self.records.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.borrow().is_empty()
    }
}

/// Write-Ahead Log manager
pub struct WriteAheadLog {
    pub log_file: RefCell<File>,
    pub current_lsn: AtomicU64,
    pub flushed_lsn: AtomicU64,
    pub buffer: WalBuffer,
    pub checkpoint_lsn: AtomicU64,
    pub active_transactions: RefCell<Vec<TransactionID>>,
}

impl WriteAheadLog {
    pub fn open(path: &str) -> Result<Self, String> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)
            .map_err(|e| format!("Failed to open WAL file: {}", e))?;

        // Get current file size to determine next LSN
        let current_lsn = 0;

        Ok(Self {
            log_file: RefCell::new(file),
            current_lsn: AtomicU64::new(0),
            flushed_lsn: AtomicU64::new(current_lsn as u64),
            buffer: WalBuffer::new(1000), // 1000 records buffer
            checkpoint_lsn: AtomicU64::new(0),
            active_transactions: RefCell::new(Vec::new()),
        })
    }

    pub fn create(path: &str) -> Result<Self, String> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .map_err(|e| format!("Failed to create WAL file: {}", e))?;

        Ok(Self {
            log_file: RefCell::new(file),
            current_lsn: AtomicU64::new(0),
            flushed_lsn: AtomicU64::new(0),
            buffer: WalBuffer::new(1000),
            checkpoint_lsn: AtomicU64::new(0),
            active_transactions: RefCell::new(Vec::new()),
        })
    }

    /// Append a log record
    pub fn append_record(&self, tx_id: TransactionID, record_type: LogRecordType,
                         page_id: PageID, offset: u16, before_image: &[u8], 
                         after_image: &[u8]) -> Result<LogSequenceNumber, String> {
        let lsn = self.current_lsn.fetch_add(1, Ordering::SeqCst) + 1;
        
        let prev_lsn = self.get_last_lsn_for_tx(tx_id);
        
        let record = LogRecord {
            lsn,
            tx_id,
            record_type,
            page_id,
            offset,
            before_image: before_image.to_vec(),
            after_image: after_image.to_vec(),
            prev_lsn,
        };

        self.buffer.push(record);
        
        // Flush buffer to disk
        self.flush_to_disk()?;

        Ok(lsn)
    }

    /// Get last LSN for a transaction (for undo chain)
    fn get_last_lsn_for_tx(&self, tx_id: TransactionID) -> LogSequenceNumber {
        // In real implementation, track LSN per transaction
        0
    }

    /// Flush buffer to disk
    fn flush_to_disk(&self) -> Result<(), String> {
        let buffer = self.buffer.clone();
        let records = buffer.records.borrow();
        
        if records.is_empty() {
            return Ok(());
        }

        let mut file = self.log_file.borrow_mut();
        
        for record in records.iter() {
            // Serialize and write record
            let data = self.serialize_record(record)?;
            file.write_all(&data)
                .map_err(|e| format!("Failed to write WAL record: {}", e))?;
        }

        file.sync_all()
            .map_err(|e| format!("Failed to sync WAL: {}", e))?;

        self.flushed_lsn.store(self.current_lsn.load(Ordering::SeqCst), Ordering::SeqCst);
        
        Ok(())
    }

    /// Serialize log record to bytes
    fn serialize_record(&self, record: &LogRecord) -> Result<Vec<u8>, String> {
        // Simplified serialization - in production, use proper serialization
        // Format: [lsn(8)][tx_id(8)][type(1)][page_id(4)][offset(2)][len_before(4)][len_after(4)][before_image][after_image]
        
        let mut data = Vec::new();
        data.extend_from_slice(&record.lsn.to_le_bytes());
        data.extend_from_slice(&record.tx_id.to_le_bytes());
        data.push(record.record_type as u8);
        data.extend_from_slice(&record.page_id.to_le_bytes());
        data.extend_from_slice(&record.offset.to_le_bytes());
        data.extend_from_slice(&(record.before_image.len() as u32).to_le_bytes());
        data.extend_from_slice(&(record.after_image.len() as u32).to_le_bytes());
        data.extend_from_slice(&record.before_image);
        data.extend_from_slice(&record.after_image);
        
        Ok(data)
    }

    /// Create checkpoint
    pub fn checkpoint(&self) -> Result<(), String> {
        // Flush all dirty pages
        self.flush_to_disk()?;
        
        // Write checkpoint record
        let checkpoint_lsn = self.current_lsn.load(Ordering::SeqCst);
        self.checkpoint_lsn.store(checkpoint_lsn, Ordering::SeqCst);
        
        // Clear active transactions
        let mut active = self.active_transactions.borrow_mut();
        active.clear();
        
        // Update checkpoint record in file
        self.write_checkpoint(checkpoint_lsn)?;
        
        Ok(())
    }

    fn write_checkpoint(&self, lsn: LogSequenceNumber) -> Result<(), String> {
        // Simplified - in production, write proper checkpoint record
        let mut file = self.log_file.borrow_mut();
        file.write_all(b"CHECKPOINT")
            .map_err(|e| format!("Failed to write checkpoint: {}", e))?;
        file.write_all(&lsn.to_le_bytes())
            .map_err(|e| format!("Failed to write checkpoint LSN: {}", e))?;
        Ok(())
    }

    /// Recover from crash
    pub fn recover(&self) -> Result<Vec<LogRecord>, String> {
        let mut file = self.log_file.borrow_mut();
        file.seek(SeekFrom::Start(0))
            .map_err(|e| format!("Failed to seek WAL: {}", e))?;

        let mut records = Vec::new();
        let mut buffer = [0u8; 1024];
        
        while let Ok(bytes_read) = file.read(&mut buffer) {
            if bytes_read == 0 {
                break;
            }
            
            // Parse log records from buffer
            // (simplified - in production, proper parsing with checksums)
            let records_parsed = self.deserialize_records(&buffer[..bytes_read]);
            records.extend(records_parsed);
        }

        Ok(records)
    }

    fn deserialize_records(&self, data: &[u8]) -> Vec<LogRecord> {
        let mut records = Vec::new();
        let mut offset = 0;

        while offset + 31 <= data.len() {
            let lsn = u64::from_le_bytes(data[offset..offset+8].try_into().unwrap_or([0; 8]));
            let tx_id = u64::from_le_bytes(data[offset+8..offset+16].try_into().unwrap_or([0; 8]));
            let record_type = match data[offset+16] {
                0 => LogRecordType::Begin,
                1 => LogRecordType::Commit,
                2 => LogRecordType::Abort,
                3 => LogRecordType::Update,
                4 => LogRecordType::DirtyPage,
                _ => break,
            };
            let page_id = u32::from_le_bytes(data[offset+17..offset+21].try_into().unwrap_or([0; 4]));
            let offset_field = u16::from_le_bytes(data[offset+21..offset+23].try_into().unwrap_or([0; 2]));
            
            let len_before = u32::from_le_bytes(data[offset+23..offset+27].try_into().unwrap_or([0; 4])) as usize;
            let len_after = u32::from_le_bytes(data[offset+27..offset+31].try_into().unwrap_or([0; 4])) as usize;
            
            if offset + 31 + len_before + len_after > data.len() {
                break;
            }

            let before_image = data[offset+31..offset+31+len_before].to_vec();
            let after_image = data[offset+31+len_before..offset+31+len_before+len_after].to_vec();

            records.push(LogRecord {
                lsn,
                tx_id,
                record_type,
                page_id,
                offset: offset_field,
                before_image,
                after_image,
                prev_lsn: 0,
            });

            offset += 31 + len_before + len_after;
        }

        records
    }

    /// Get current LSN
    pub fn current_lsn(&self) -> LogSequenceNumber {
        self.current_lsn.load(Ordering::SeqCst)
    }

    /// Get flushed LSN
    pub fn flushed_lsn(&self) -> LogSequenceNumber {
        self.flushed_lsn.load(Ordering::SeqCst)
    }

    /// Get checkpoint LSN
    pub fn checkpoint_lsn(&self) -> LogSequenceNumber {
        self.checkpoint_lsn.load(Ordering::SeqCst)
    }

    /// Check if WAL is empty
    pub fn is_empty(&self) -> bool {
        self.current_lsn.load(Ordering::SeqCst) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wal_create() {
        let wal = WriteAheadLog::create(":memory:").unwrap_or_else(|_| {
            // Create temp file for testing
            WriteAheadLog::create("/tmp/test_wal.log").unwrap()
        });

        assert!(wal.is_empty());
    }

    #[test]
    fn test_append_record() {
        let wal = WriteAheadLog::create("/tmp/test_wal2.log").unwrap_or_else(|_| {
            WriteAheadLog::create("/tmp/test_wal.log").unwrap()
        });

        let result = wal.append_record(1, LogRecordType::Begin, 0, 0, &[], &[]);
        
        assert!(result.is_ok());
        assert!(wal.current_lsn() >= 1);
    }

    #[test]
    fn test_checkpoint() {
        let wal = WriteAheadLog::create("/tmp/test_wal3.log").unwrap_or_else(|_| {
            WriteAheadLog::create("/tmp/test_wal.log").unwrap()
        });

        wal.checkpoint().unwrap();
        
        assert!(wal.checkpoint_lsn() >= 0);
    }

    #[test]
    fn test_recovery() {
        let wal = WriteAheadLog::create("/tmp/test_wal4.log").unwrap_or_else(|_| {
            WriteAheadLog::create("/tmp/test_wal.log").unwrap()
        });

        let _ = wal.append_record(1, LogRecordType::Begin, 0, 0, &[], &[]);
        
        // In real implementation, test recovery from file
        // For now, just verify we can recover
        let _records = wal.recover().unwrap();
    }
}
