use crate::models::{error::AppError, house::HouseDetail};
use serde::{de::DeserializeOwned, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;
use uuid::Uuid;

#[derive(Clone)]
pub struct FileStorage {
    base_path: PathBuf,
}

impl FileStorage {
    pub fn new(base_path: &str) -> Self {
        let path = Path::new(base_path);
        if !path.exists() {
            fs::create_dir_all(path).expect("Failed to create storage directory");
        }
        Self {
            base_path: path.to_path_buf(),
        }
    }

    pub fn read_json<T: DeserializeOwned>(&self, path: &str) -> Result<T, AppError> {
        let full_path = self.base_path.join(path);
        let mut file = File::open(&full_path).map_err(|e| {
            AppError::FileSystemError(format!("Failed to open file {}: {}", path, e))
        })?;

        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| {
            AppError::FileSystemError(format!("Failed to read file {}: {}", path, e))
        })?;

        serde_json::from_str(&contents)
            .map_err(|e| AppError::ParseError(format!("Failed to parse JSON from {}: {}", path, e)))
    }

    pub fn write_json<S: Serialize>(&self, path: &str, data: &S) -> Result<(), AppError> {
        let full_path = self.base_path.join(path);
        let parent_dir = full_path
            .parent()
            .ok_or_else(|| AppError::FileSystemError("Invalid file path".to_string()))?;

        // 创建父目录（如果不存在）
        fs::create_dir_all(parent_dir).map_err(|e| {
            AppError::FileSystemError(format!(
                "Failed to create directory {}: {}",
                parent_dir.display(),
                e
            ))
        })?;

        // 创建临时文件
        let mut temp_file = NamedTempFile::new_in(parent_dir)
            .map_err(|e| AppError::FileSystemError(format!("Failed to create temp file: {}", e)))?;

        // 序列化数据并写入临时文件
        let json = serde_json::to_string_pretty(data)
            .map_err(|e| AppError::ParseError(format!("Failed to serialize data: {}", e)))?;
        temp_file.write_all(json.as_bytes()).map_err(|e| {
            AppError::FileSystemError(format!("Failed to write to temp file: {}", e))
        })?;

        // 持久化临时文件到目标路径
        temp_file.persist(&full_path).map_err(|e| {
            AppError::FileSystemError(format!("Failed to save file {}: {}", path, e))
        })?;

        Ok(())
    }

    pub fn verify_password(&self, password: &str, stored_password: &str) -> Result<bool, AppError> {
        Ok(password == stored_password)
    }

    // 房屋详细数据版本控制写入
    pub fn write_house_detail(
        &self,
        house_id: &str,
        new_data: &HouseDetail,
        expected_version: &str,
    ) -> Result<(), AppError> {
        let current: HouseDetail = self.read_json(&format!("house/{}.json", house_id))?;
        if current.version != expected_version {
            return Err(AppError::InvalidVersion);
        }

        let new_version = Uuid::new_v4().to_string();
        let mut updated_data = new_data.clone();
        updated_data.version = new_version;

        self.write_json(&format!("house/{}.json", house_id), &updated_data)
    }

    pub fn delete_file(&self, path: &str) -> Result<(), std::io::Error> {
        let full_path = self.base_path.join(path);
        fs::remove_file(full_path)
    }
}
