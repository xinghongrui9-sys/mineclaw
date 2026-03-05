//! Checkpoint 模块测试 - 基础覆盖

use mineclaw::checkpoint::CheckpointManager;
use mineclaw::config::CheckpointConfig;
use mineclaw::error::Error;
use tempfile::tempdir;

#[tokio::test]
async fn test_new_manager() {
    let temp_dir = tempdir().unwrap();

    let config = CheckpointConfig {
        enabled: true,
        directory: ".test_checkpoints".to_string(),
        max_per_session: 10,
        auto_cleanup_days: 30,
    };

    // 测试创建manager
    let result = CheckpointManager::new(config, &temp_dir);
    assert!(result.is_ok());

    // 验证目录已创建
    let checkpoint_dir = temp_dir.path().join(".test_checkpoints");
    assert!(checkpoint_dir.exists());
}

#[tokio::test]
async fn test_checkpoint_disabled() {
    let temp_dir = tempdir().unwrap();

    // 禁用checkpoint
    let config = CheckpointConfig {
        enabled: false,
        directory: ".test_checkpoints".to_string(),
        max_per_session: 10,
        auto_cleanup_days: 30,
    };

    let manager = CheckpointManager::new(config, &temp_dir).unwrap();

    let test_file = temp_dir.path().join("test.txt");
    std::fs::write(&test_file, "Test content").unwrap();

    let session_id = "test-session";
    let paths = vec![test_file];

    // before_operation应该返回None
    let checkpoint_opt = manager
        .before_operation(session_id, "write_file", &paths)
        .await
        .unwrap();

    assert!(checkpoint_opt.is_none());
}

#[tokio::test]
async fn test_find_nonexistent_checkpoint() {
    let temp_dir = tempdir().unwrap();

    let config = CheckpointConfig {
        enabled: true,
        directory: ".test_checkpoints".to_string(),
        max_per_session: 10,
        auto_cleanup_days: 30,
    };

    let manager = CheckpointManager::new(config, &temp_dir).unwrap();

    // 尝试恢复不存在的checkpoint应该返回错误
    let result = manager.restore_checkpoint("nonexistent-id").await;
    assert!(matches!(result, Err(Error::CheckpointNotFound(_))));

    // 尝试删除不存在的checkpoint也应该返回错误
    let result = manager.delete_checkpoint("nonexistent-id").await;
    assert!(matches!(result, Err(Error::CheckpointNotFound(_))));
}

#[tokio::test]
async fn test_before_operation_no_existing_files() {
    let temp_dir = tempdir().unwrap();

    let config = CheckpointConfig {
        enabled: true,
        directory: ".test_checkpoints".to_string(),
        max_per_session: 10,
        auto_cleanup_days: 30,
    };

    let manager = CheckpointManager::new(config, &temp_dir).unwrap();

    let session_id = "test-session";

    // 所有文件都不存在
    let paths = vec![temp_dir.path().join("nonexistent.txt")];
    let checkpoint_opt = manager
        .before_operation(session_id, "write_file", &paths)
        .await
        .unwrap();

    // 应该返回None
    assert!(checkpoint_opt.is_none());
}