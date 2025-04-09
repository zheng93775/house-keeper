use crate::{models::error::AppError};
use chrono::Local;
use std::path::{Path, PathBuf};

pub struct BackupManager {
    backup_path: PathBuf,
    storage_base_path: PathBuf, // 新增字段
}

impl BackupManager {
    // 修改构造函数，增加 storage_base_path 参数
    pub fn new(backup_path: &str, storage_base_path: &str) -> Self {
        let backup = Path::new(backup_path);
        if !backup.exists() {
            std::fs::create_dir_all(backup).expect("Failed to create backup directory");
        }
        let storage = Path::new(storage_base_path);
        if !storage.exists() {
            std::fs::create_dir_all(storage).expect("Failed to create storage directory");
        }
        Self {
            backup_path: backup.to_path_buf(),
            storage_base_path: storage.to_path_buf(), // 初始化新增字段
        }
    }

    pub fn needs_backup(&self, file_path: &str) -> Result<bool, AppError> {
        // 使用 storage_base_path 拼接完整的文件路径
        let source_path = self.storage_base_path.join(file_path);
        let backup_file = self.latest_backup_file(file_path)?;

        if !source_path.exists() {
            return Ok(false);
        }

        let source_modified = source_path.metadata()?.modified()?;
        let backup_modified = backup_file
           .map(|p| p.metadata().and_then(|m| m.modified()))
           .transpose()?;

        Ok(backup_modified.map_or(true, |bt| source_modified > bt))
    }

    fn latest_backup_file(&self, file_path: &str) -> Result<Option<PathBuf>, AppError> {
        let file_name = Path::new(file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| AppError::FileSystemError("Invalid file path".to_string()))?;

        let _pattern = format!("{}.*.json", file_name.trim_end_matches(".json"));
        let mut backups: Vec<_> = std::fs::read_dir(&self.backup_path)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let name = entry.file_name().to_str()?.to_string();
                if name.starts_with(file_name) && name.ends_with(".json") {
                    Some(entry.path())
                } else {
                    None
                }
            })
            .collect();

        backups.sort();
        Ok(backups.last().cloned())
    }

    pub fn create_backup(&self, file_path: &str) -> Result<(), AppError> {
        if !self.needs_backup(file_path)? {
            return Ok(());
        }

        let date_str = Local::now().format("%Y%m%d").to_string();
        let backup_path = self.backup_path.join(format!(
            "{}.{}",
            Path::new(file_path).file_name().unwrap().to_str().unwrap(),
            date_str
        ));

        // 使用 storage_base_path 拼接完整的文件路径
        let full_source_path = self.storage_base_path.join(file_path);
        std::fs::copy(full_source_path, backup_path)?;
        Ok(())
    }
}
