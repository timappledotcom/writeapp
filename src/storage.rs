use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FlowEntry {
    pub timestamp: DateTime<Utc>,
    pub duration_minutes: u32,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub default_extension: String,
    pub storage_path: String,
    pub vim_mode: bool,
}

impl Default for Settings {
    fn default() -> Self {
        let storage_path = if let Some(user_dirs) = directories::UserDirs::new() {
             if let Some(docs) = user_dirs.document_dir() {
                 docs.join("WriteApp").to_string_lossy().to_string()
             } else {
                 user_dirs.home_dir().join("WriteApp").to_string_lossy().to_string()
             }
        } else {
             "WriteApp".to_string()
        };

        Self {
            default_extension: "txt".to_string(),
            storage_path,
            vim_mode: false,
        }
    }
}

pub struct Storage;

impl Storage {
    fn get_app_dir() -> Result<PathBuf> {
        if let Some(proj_dirs) = directories::ProjectDirs::from("com", "writeapp", "writeapp") {
            let config_dir = proj_dirs.config_dir();
            if !config_dir.exists() {
                fs::create_dir_all(config_dir)?;
            }
            Ok(config_dir.to_path_buf())
        } else {
            // Fallback to local .config/writeapp in home if ProjectDirs fails
            let home = directories::UserDirs::new()
                .ok_or_else(|| anyhow::anyhow!("Could not find user home directory"))?;
            let path = home.home_dir().join(".config").join("writeapp");
            if !path.exists() {
                fs::create_dir_all(&path)?;
            }
            Ok(path)
        }
    }

    fn get_content_dir() -> Result<PathBuf> {
        let settings = Self::load_settings()?;
        let path = PathBuf::from(settings.storage_path);
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }
        Ok(path)
    }

    fn get_history_path() -> Result<PathBuf> {
        let dir = Self::get_content_dir()?;
        Ok(dir.join("flow_history.json"))
    }

    fn get_settings_path() -> Result<PathBuf> {
        let dir = Self::get_app_dir()?;
        Ok(dir.join("settings.json"))
    }

    pub fn load_settings() -> Result<Settings> {
        let path = Self::get_settings_path()?;
        if !path.exists() {
            return Ok(Settings::default());
        }
        let content = fs::read_to_string(path)?;
        let settings: Settings = serde_json::from_str(&content).unwrap_or_default();
        Ok(settings)
    }

    pub fn save_settings(settings: &Settings) -> Result<()> {
        let path = Self::get_settings_path()?;
        let content = serde_json::to_string_pretty(settings)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn load_flow_history() -> Result<Vec<FlowEntry>> {
        let path = Self::get_history_path()?;
        if !path.exists() {
            return Ok(Vec::new());
        }
        let content = fs::read_to_string(path)?;
        let history: Vec<FlowEntry> = serde_json::from_str(&content).unwrap_or_default();
        Ok(history)
    }

    pub fn save_flow_entry(entry: FlowEntry) -> Result<()> {
        let mut history = Self::load_flow_history()?;
        history.push(entry);
        // Sort by timestamp descending
        history.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        let path = Self::get_history_path()?;
        let content = serde_json::to_string_pretty(&history)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn save_draft(filename: &str, content: &str) -> Result<()> {
        let dir = Self::get_content_dir()?.join("drafts");
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        let path = dir.join(filename);
        fs::write(path, content)?;
        Ok(())
    }

    pub fn list_drafts() -> Result<Vec<String>> {
        let dir = Self::get_content_dir()?.join("drafts");
        if !dir.exists() {
            return Ok(Vec::new());
        }
        let mut drafts = Vec::new();
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name() {
                    drafts.push(name.to_string_lossy().to_string());
                }
            }
        }
        drafts.sort();
        Ok(drafts)
    }

    pub fn load_draft(filename: &str) -> Result<String> {
        let dir = Self::get_content_dir()?.join("drafts");
        let path = dir.join(filename);
        let content = fs::read_to_string(path)?;
        Ok(content)
    }

    pub fn rename_draft(old_name: &str, new_name: &str) -> Result<()> {
        let dir = Self::get_content_dir()?.join("drafts");
        let old_path = dir.join(old_name);
        let new_path = dir.join(new_name);
        fs::rename(old_path, new_path)?;
        Ok(())
    }

    pub fn delete_draft(filename: &str) -> Result<()> {
        let dir = Self::get_content_dir()?.join("drafts");
        let path = dir.join(filename);
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }
}
