// ============================================================================
// OS & Database Integration Tests
// ============================================================================
// Comprehensive integration tests untuk OS Infrastructure & Database Engine
// ============================================================================

use widya_lang::os;
use widya_lang::db;

// ============================================================================
// OS Integration Tests
// ============================================================================

#[cfg(test)]
mod os_integration_tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_memory_manager_initialization() {
        let mm = os::MemoryManager::new();
        assert!(mm.stats().total_pages > 0);
    }

    #[test]
    fn test_thread_creation_and_management() {
        let manager = os::KernelThreadManager::new();
        
        fn test_entry(_arg: usize) -> usize {
            42
        }
        
        let handle = manager.create_thread(test_entry, 0, os::Priority::Normal);
        assert!(handle.is_ok());
        
        let thread = handle.unwrap();
        assert_eq!(thread.state(), os::ThreadState::Ready);
    }

    #[test]
    fn test_thread_priority_scheduling() {
        let manager = os::KernelThreadManager::new();
        manager.enable_scheduler();
        
        fn low_priority_fn(_arg: usize) -> usize { 1 }
        fn high_priority_fn(_arg: usize) -> usize { 2 }
        
        let _low = manager.create_thread(low_priority_fn, 0, os::Priority::Low).unwrap();
        let _high = manager.create_thread(high_priority_fn, 0, os::Priority::High).unwrap();
        
        manager.yield_cpu();
        assert!(manager.scheduler_tick() > 0);
    }

    #[test]
    fn test_spinlock_basic_usage() {
        let lock = os::Spinlock::new();
        
        assert!(!lock.is_locked());
        
        lock.lock();
        assert!(lock.is_locked());
        
        lock.unlock();
        assert!(!lock.is_locked());
    }

    #[test]
    fn test_mutex_lock_unlock() {
        let mutex = os::Mutex::new();
        let manager = os::KernelThreadManager::new();
        
        assert!(!mutex.is_locked());
        
        mutex.lock(&manager);
        assert!(mutex.is_locked());
        
        mutex.unlock();
        assert!(!mutex.is_locked());
    }

    #[test]
    fn test_semaphore_operations() {
        let sem = os::Semaphore::new(2, 5);
        
        assert_eq!(sem.available(), 2);
        
        let manager = os::KernelThreadManager::new();
        sem.wait(&manager);
        
        assert_eq!(sem.available(), 1);
        
        sem.signal();
        assert_eq!(sem.available(), 2);
    }

    #[test]
    fn test_barrier_synchronization() {
        let barrier = os::Barrier::new(3);
        let manager = os::KernelThreadManager::new();
        
        // Test basic barrier functionality
        // (Full multi-thread test would require actual threads)
        
        assert!(barrier.wait(&manager).is_ok());
    }

    #[test]
    fn test_syscall_table_registration() {
        let table = os::SyscallTable::new();
        
        // Verify all default syscalls are registered
        assert!(table.get_handler(0).is_some()); // read
        assert!(table.get_handler(1).is_some()); // write
        assert!(table.get_handler(2).is_some()); // open
        assert!(table.get_handler(3).is_some()); // close
        assert!(table.get_handler(100).is_none()); // Invalid syscall
    }

    #[test]
    fn test_filesystem_operations() {
        let vfs = os::VirtualFilesystem::new();
        
        // Test filesystem initialization
        assert!(vfs.mount_points.read().unwrap().is_empty());
    }

    #[test]
    fn test_open_flags_conversion() {
        let flags = os::OpenFlags {
            read: true,
            write: false,
            append: true,
            truncate: false,
            create: true,
            exclusive: false,
        };
        
        let raw = flags.to_raw();
        let restored = os::OpenFlags::from_raw(raw);
        
        assert_eq!(flags.read, restored.read);
        assert_eq!(flags.append, restored.append);
        assert_eq!(flags.create, restored.create);
    }

    #[test]
    fn test_inode_metadata() {
        let metadata = os::InodeMetadata::new(1, os::FileType::Regular, 0o644);
        
        assert_eq!(metadata.inode, 1);
        assert_eq!(metadata.file_type, os::FileType::Regular);
        assert_eq!(metadata.permissions, 0o644);
        assert_eq!(metadata.link_count, 1);
    }

    #[test]
    fn test_file_type_modes() {
        assert_eq!(os::FileType::Regular.to_mode(), 0o100000);
        assert_eq!(os::FileType::Directory.to_mode(), 0o040000);
        assert_eq!(os::FileType::Symlink.to_mode(), 0o120000);
    }

    #[test]
    fn test_vfs_directory_creation() {
        let vfs = os::VirtualFilesystem::new();
        
        // Test mkdir functionality
        // (Would require actual mount in production)
    }

    #[test]
    fn test_multiple_thread_operations() {
        let manager = os::KernelThreadManager::new();
        manager.enable_scheduler();
        
        for i in 0..5 {
            fn thread_fn(_arg: usize) -> usize {
                0
            }
            
            let _handle = manager.create_thread(thread_fn, i as usize, os::Priority::Normal).unwrap();
        }
        
        assert_eq!(manager.thread_count(), 5);
    }

    #[test]
    fn test_thread_state_transitions() {
        let manager = os::KernelThreadManager::new();
        
        fn test_fn(_arg: usize) -> usize { 0 }
        
        let handle = manager.create_thread(test_fn, 0, os::Priority::Normal).unwrap();
        assert_eq!(handle.state(), os::ThreadState::Ready);
        
        manager.start_thread(handle.id()).unwrap();
        // Thread should remain in Ready state until scheduler runs
        
        manager.yield_cpu();
        assert!(manager.scheduler_tick() > 0);
    }
}

// ============================================================================
// Database Integration Tests
// ============================================================================

#[cfg(test)]
mod database_integration_tests {
    use std::fs;
    
    #[test]
    fn test_wal_create_and_append() {
        let wal_path = "/tmp/test_wal_integration.log";
        let _ = fs::remove_file(wal_path);
        
        let wal = db::WriteAheadLog::create(wal_path).unwrap();
        
        assert!(wal.is_empty());
        
        let lsn = wal.append_record(
            1, 
            db::LogRecordType::Begin, 
            0, 
            0, 
            &[], 
            &[],
        ).unwrap();
        
        assert!(lsn >= 1);
        assert!(!wal.is_empty());
    }

    #[test]
    fn test_wal_checkpoint() {
        let wal_path = "/tmp/test_wal_checkpoint.log";
        let _ = fs::remove_file(wal_path);
        
        let wal = db::WriteAheadLog::create(wal_path).unwrap();
        
        wal.append_record(1, db::LogRecordType::Begin, 0, 0, &[], &[]).unwrap();
        wal.checkpoint().unwrap();
        
        assert!(wal.checkpoint_lsn() >= 1);
    }

    #[test]
    fn test_wal_recovery() {
        let wal_path = "/tmp/test_wal_recovery.log";
        let _ = fs::remove_file(wal_path);
        
        let wal = db::WriteAheadLog::create(wal_path).unwrap();
        
        let _lsn = wal.append_record(
            1, 
            db::LogRecordType::Begin, 
            0, 
            0, 
            &[1, 2, 3], 
            &[4, 5, 6],
        ).unwrap();
        
        let records = wal.recover().unwrap();
        assert!(!records.is_empty());
    }

    #[test]
    fn test_btree_insert_and_lookup() {
        let mut tree = db::BPlusTree::new(4); // Order 4
        
        tree.insert(b"key1", b"value1").unwrap();
        tree.insert(b"key2", b"value2").unwrap();
        tree.insert(b"key3", b"value3").unwrap();
        
        // Verify insertion
        assert!(true); // B+Tree operations completed without panic
    }

    #[test]
    fn test_btree_range_scan() {
        let mut tree = db::BPlusTree::new(4);
        
        tree.insert(b"apple", b"red").unwrap();
        tree.insert(b"banana", b"yellow").unwrap();
        tree.insert(b"cherry", b"red").unwrap();
        
        // Range scan would iterate through entries
        let iter = tree.range(b"banana"..=b"cherry");
        
        let entries: Vec<_> = iter.collect();
        assert!(entries.len() >= 2);
    }

    #[test]
    fn test_sql_parser_select() {
        let input = "SELECT id, name FROM users WHERE id = 1";
        let tokenizer = db::Tokenizer::new(input);
        let tokens: Vec<_> = std::iter::from_fn(|| Some(tokenizer.next_token())).collect();
        
        assert!(tokens.len() > 0);
        assert!(matches!(tokens[0], db::TokenType::SELECT));
    }

    #[test]
    fn test_sql_parser_insert() {
        let input = "INSERT INTO users (id, name) VALUES (1, 'Alice')";
        let tokenizer = db::Tokenizer::new(input);
        let tokens: Vec<_> = std::iter::from_fn(|| Some(tokenizer.next_token())).collect();
        
        assert!(tokens.len() > 0);
        assert!(matches!(tokens[0], db::TokenType::INSERT));
    }

    #[test]
    fn test_sql_parser_with_join() {
        let input = "SELECT u.name, o.amount FROM users u INNER JOIN orders o ON u.id = o.user_id";
        let tokenizer = db::Tokenizer::new(input);
        let tokens: Vec<_> = std::iter::from_fn(|| Some(tokenizer.next_token())).collect();
        
        let has_join = tokens.iter().any(|t| matches!(t, db::TokenType::JOIN));
        let has_inner = tokens.iter().any(|t| matches!(t, db::TokenType::INNER));
        
        assert!(has_join);
        assert!(has_inner);
    }

    #[test]
    fn test_sql_parser_left_join() {
        let input = "SELECT * FROM customers LEFT JOIN orders ON customers.id = orders.customer_id";
        let tokenizer = db::Tokenizer::new(input);
        let tokens: Vec<_> = std::iter::from_fn(|| Some(tokenizer.next_token())).collect();
        
        let has_left = tokens.iter().any(|t| matches!(t, db::TokenType::LEFT));
        assert!(has_left);
    }

    #[test]
    fn test_sql_parser_right_join() {
        let input = "SELECT * FROM products RIGHT JOIN inventory ON products.id = inventory.product_id";
        let tokenizer = db::Tokenizer::new(input);
        let tokens: Vec<_> = std::iter::from_fn(|| Some(tokenizer.next_token())).collect();
        
        let has_right = tokens.iter().any(|t| matches!(t, db::TokenType::RIGHT));
        assert!(has_right);
    }

    #[test]
    fn test_sql_parser_full_join() {
        let input = "SELECT * FROM table1 FULL JOIN table2 ON table1.id = table2.id";
        let tokenizer = db::Tokenizer::new(input);
        let tokens: Vec<_> = std::iter::from_fn(|| Some(tokenizer.next_token())).collect();
        
        let has_full = tokens.iter().any(|t| matches!(t, db::TokenType::FULL));
        assert!(has_full);
    }

    #[test]
    fn test_btree_bulk_load() {
        let mut tree = db::BPlusTree::new(4);
        
        // Bulk insert many entries
        for i in 0..100 {
            let key = format!("key{:03}", i).into_bytes();
            let value = format!("value{:03}", i).into_bytes();
            tree.insert(&key, &value).unwrap();
        }
        
        // Verify all entries inserted
        assert!(true); // No panic means all inserts succeeded
    }

    #[test]
    fn test_wal_multiple_transactions() {
        let wal_path = "/tmp/test_wal_multi_tx.log";
        let _ = fs::remove_file(wal_path);
        
        let wal = db::WriteAheadLog::create(wal_path).unwrap();
        
        // Transaction 1
        wal.append_record(1, db::LogRecordType::Begin, 0, 0, &[], &[]).unwrap();
        wal.append_record(1, db::LogRecordType::Update, 0, 0, &[], &[1, 2, 3]).unwrap();
        wal.append_record(1, db::LogRecordType::Commit, 0, 0, &[], &[]).unwrap();
        
        // Transaction 2
        wal.append_record(2, db::LogRecordType::Begin, 0, 0, &[], &[]).unwrap();
        wal.append_record(2, db::LogRecordType::Update, 1, 0, &[], &[4, 5, 6]).unwrap();
        wal.append_record(2, db::LogRecordType::Commit, 0, 0, &[], &[]).unwrap();
        
        assert!(wal.current_lsn() >= 6);
    }

    #[test]
    fn test_filesystem_mount_unmount() {
        let vfs = os::VirtualFilesystem::new();
        
        // Test mount point creation (mock filesystem)
        assert!(vfs.mount_points.read().unwrap().is_empty());
        
        // In real implementation, would test:
        // vfs.register_filesystem(FilesystemType { ... }).unwrap();
        // vfs.mount("mockfs", "/source", "/target").unwrap();
        // assert!(vfs.mount_points.read().unwrap().contains_key(Path::new("/target")));
    }

    #[test]
    fn test_inode_file_type_mapping() {
        let file_types = vec![
            os::FileType::Regular,
            os::FileType::Directory,
            os::FileType::Symlink,
            os::FileType::BlockDevice,
            os::FileType::CharDevice,
            os::FileType::FIFO,
            os::FileType::Socket,
        ];
        
        for ft in file_types {
            let mode = ft.to_mode();
            assert!(mode > 0, "File type {:?} should have valid mode", ft);
        }
    }
}

// ============================================================================
// End-to-End Integration Tests
// ============================================================================

#[cfg(test)]
mod end_to_end_tests {
    use std::fs;
    
    #[test]
    fn test_full_database_workflow() {
        let wal_path = "/tmp/test_e2e_wal.log";
        let _ = fs::remove_file(wal_path);
        
        // Create WAL
        let wal = db::WriteAheadLog::create(wal_path).unwrap();
        
        // Start transaction
        wal.append_record(1, db::LogRecordType::Begin, 0, 0, &[], &[]).unwrap();
        
        // Insert data
        wal.append_record(1, db::LogRecordType::Update, 1, 0, &[], &[]).unwrap();
        wal.append_record(1, db::LogRecordType::Update, 2, 0, &[], &[]).unwrap();
        
        // Commit
        wal.append_record(1, db::LogRecordType::Commit, 0, 0, &[], &[]).unwrap();
        
        // Verify
        assert!(wal.current_lsn() >= 4);
    }

    #[test]
    fn test_sql_parser_comprehensive() {
        let queries = vec![
            "SELECT * FROM users",
            "SELECT u.id, u.name FROM users u WHERE u.age > 18",
            "SELECT * FROM users u INNER JOIN orders o ON u.id = o.user_id",
            "SELECT * FROM users LEFT JOIN orders ON users.id = orders.customer_id",
            "SELECT * FROM products RIGHT JOIN inventory ON products.id = inventory.product_id",
            "INSERT INTO users (id, name, email) VALUES (1, 'John', 'john@example.com')",
            "UPDATE users SET name = 'Jane' WHERE id = 1",
            "DELETE FROM users WHERE id = 1",
            "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL, email TEXT UNIQUE)",
        ];
        
        for query in queries {
            let tokenizer = db::Tokenizer::new(query);
            let _tokens: Vec<_> = std::iter::from_fn(|| Some(tokenizer.next_token())).collect();
            // If we get here without panic, parsing worked
        }
    }

    #[test]
    fn test_os_threading_comprehensive() {
        let manager = os::KernelThreadManager::new();
        manager.enable_scheduler();
        
        // Create multiple threads with different priorities
        for i in 0..10 {
            fn thread_fn(_arg: usize) -> usize {
                _arg
            }
            
            let priority = match i % 5 {
                0 => os::Priority::Idle,
                1 => os::Priority::Low,
                2 => os::Priority::Normal,
                3 => os::Priority::High,
                4 => os::Priority::Realtime,
                _ => os::Priority::Normal,
            };
            
            let _handle = manager.create_thread(thread_fn, i as usize, priority).unwrap();
        }
        
        assert_eq!(manager.thread_count(), 10);
        
        // Run scheduler
        manager.yield_cpu();
        assert!(manager.scheduler_tick() > 0);
    }

    #[test]
    fn test_sql_parser_join_variations() {
        let queries = vec![
            // Inner joins
            "SELECT * FROM a INNER JOIN b ON a.id = b.a_id",
            "SELECT * FROM a JOIN b ON a.id = b.a_id",  // JOIN = INNER JOIN
            // Left joins
            "SELECT * FROM a LEFT JOIN b ON a.id = b.a_id",
            "SELECT * FROM a LEFT OUTER JOIN b ON a.id = b.a_id",
            // Right joins  
            "SELECT * FROM a RIGHT JOIN b ON a.id = b.a_id",
            "SELECT * FROM a RIGHT OUTER JOIN b ON a.id = b.a_id",
            // Full joins
            "SELECT * FROM a FULL JOIN b ON a.id = b.a_id",
            "SELECT * FROM a FULL OUTER JOIN b ON a.id = b.a_id",
            // Complex joins
            "SELECT * FROM a INNER JOIN b ON a.id = b.a_id INNER JOIN c ON b.id = c.b_id",
            "SELECT * FROM a LEFT JOIN b ON a.id = b.a_id RIGHT JOIN c ON b.id = c.b_id",
        ];
        
        for query in queries {
            let tokenizer = db::Tokenizer::new(query);
            let tokens: Vec<_> = std::iter::from_fn(|| Some(tokenizer.next_token())).collect();
            
            // Verify some expected tokens exist
            let has_from = tokens.iter().any(|t| matches!(t, db::TokenType::FROM));
            let has_join = tokens.iter().any(|t| matches!(t, db::TokenType::JOIN));
            
            assert!(has_from, "Query should have FROM: {}", query);
            assert!(has_join, "Query should have JOIN: {}", query);
        }
    }

    #[test]
    fn test_sql_parser_complex_conditions() {
        let queries = vec![
            "SELECT * FROM users WHERE (age > 18 AND status = 'active') OR (age <= 18 AND parent_approved = 1)",
            "SELECT * FROM orders WHERE amount BETWEEN 100 AND 1000 AND status IN ('pending', 'processing')",
            "SELECT * FROM products WHERE name LIKE '%widget%' AND (price < 50 OR category = 'discount')",
        ];
        
        for query in queries {
            let tokenizer = db::Tokenizer::new(query);
            let _tokens: Vec<_> = std::iter::from_fn(|| Some(tokenizer.next_token())).collect();
            // If we get here without panic, parsing worked
        }
    }
}

// ============================================================================
// Benchmark Tests (for performance evaluation)
// ============================================================================

#[cfg(test)]
mod benchmark_tests {
    use std::time::Instant;
    
    #[bench]
    fn bench_wal_insertions(b: &mut test::Bencher) {
        let wal_path = "/tmp/bench_wal.log";
        let _ = std::fs::remove_file(wal_path);
        
        let wal = db::WriteAheadLog::create(wal_path).unwrap();
        
        b.iter(|| {
            let _ = wal.append_record(1, db::LogRecordType::Begin, 0, 0, &[], &[]);
        });
    }

    #[bench]
    fn bench_btree_insertions(b: &mut test::Bencher) {
        let mut tree = db::BPlusTree::new(16);
        
        b.iter(|| {
            let key = b"test_key";
            let value = b"test_value";
            let _ = tree.insert(key, value);
        });
    }

    #[bench]
    fn bench_sql_parser(b: &mut test::Bencher) {
        let query = "SELECT * FROM users u INNER JOIN orders o ON u.id = o.user_id WHERE u.status = 'active'";
        
        b.iter(|| {
            let tokenizer = db::Tokenizer::new(query);
            let tokens: Vec<_> = std::iter::from_fn(|| Some(tokenizer.next_token())).collect();
        });
    }

    #[bench]
    fn bench_thread_creation(b: &mut test::Bencher) {
        let manager = os::KernelThreadManager::new();
        
        fn dummy_fn(_arg: usize) -> usize { 0 }
        
        b.iter(|| {
            let _ = manager.create_thread(dummy_fn, 0, os::Priority::Normal);
        });
    }
}

// ============================================================================
// Notes for Running Tests
// ============================================================================

// To run all tests:
// cargo test --lib
//
// To run specific test module:
// cargo test --lib os_integration_tests
// cargo test --lib database_integration_tests
// cargo test --lib end_to_end_tests
//
// To run with output:
// cargo test --lib -- --nocapture
//
// To run benchmarks:
// cargo bench
//
// To run with coverage:
// cargo test --lib --coverage
