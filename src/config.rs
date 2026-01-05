// Configuration management for Array30 Input Method
// 設定檔管理

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const DEFAULT_FONT_SIZE: f32 = 20.0;
const CONFIG_FILENAME: &str = "settings.ini";

/// 字根表位置
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RootTablePosition {
    /// 上方
    Up,
    /// 下方
    Down,
    /// 左側
    Left,
    /// 右側
    Right,
}

impl RootTablePosition {
    pub fn as_str(&self) -> &'static str {
        match self {
            RootTablePosition::Up => "up",
            RootTablePosition::Down => "down",
            RootTablePosition::Left => "left",
            RootTablePosition::Right => "right",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            RootTablePosition::Up => "上方",
            RootTablePosition::Down => "下方",
            RootTablePosition::Left => "左側",
            RootTablePosition::Right => "右側",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "up" => Some(RootTablePosition::Up),
            "down" => Some(RootTablePosition::Down),
            "left" => Some(RootTablePosition::Left),
            "right" => Some(RootTablePosition::Right),
            _ => None,
        }
    }
}

/// 應用程式設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// 字型檔案路徑
    pub font_path: String,
    /// 字型大小
    pub font_size: f32,
    /// 顯示行列字根表
    pub show_root_table: bool,
    /// 字根表圖片縮放比例 (0.1 - 2.0)
    pub root_table_scale: f32,
    /// 視窗寬度
    pub window_width: f32,
    /// 視窗高度
    pub window_height: f32,
    /// 字根表位置
    pub root_table_position: RootTablePosition,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            font_path: get_default_font_path(),
            font_size: DEFAULT_FONT_SIZE,
            show_root_table: true,
            root_table_scale: 0.5,
            window_width: 1600.0,
            window_height: 900.0,
            root_table_position: RootTablePosition::Up,
        }
    }
}

impl Config {
    /// 設定檔路徑
    pub fn config_file_path() -> Option<PathBuf> {
        // 優先使用當前目錄
        let local_path = PathBuf::from(CONFIG_FILENAME);
        if local_path.exists() {
            return Some(local_path);
        }

        // 嘗試使用設定目錄
        if let Some(config_dir) = dirs::config_dir() {
            let app_config_dir = config_dir.join("rustarray30");
            let config_path = app_config_dir.join(CONFIG_FILENAME);

            // 如果目錄不存在，嘗試建立
            if !app_config_dir.exists() {
                if let Err(e) = std::fs::create_dir_all(&app_config_dir) {
                    eprintln!("無法建立設定目錄: {}", e);
                    return Some(local_path);
                }
            }

            return Some(config_path);
        }

        Some(local_path)
    }

    /// 載入設定檔
    pub fn load() -> Self {
        if let Some(path) = Self::config_file_path() {
            if path.exists() {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    // 嘗試解析 INI 檔案
                    if let Ok(config) = Self::parse_ini(&content) {
                        return config;
                    }
                }
            }
        }

        // 如果載入失敗，返回預設值並儲存
        let default = Self::default();
        let _ = default.save();
        default
    }

    /// 解析 INI 格式設定檔
    fn parse_ini(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut font_path = String::new();
        let mut font_size = DEFAULT_FONT_SIZE;
        let mut show_root_table = true;
        let mut root_table_scale = 0.5;
        let mut window_width = 1600.0;
        let mut window_height = 900.0;
        let mut root_table_position = RootTablePosition::Up;

        for line in content.lines() {
            let line = line.trim();
            // 跳過註解和空行
            if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                continue;
            }

            // 解析 key=value
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "font_path" => font_path = value.to_string(),
                    "font_size" => {
                        if let Ok(size) = value.parse::<f32>() {
                            font_size = size.max(10.0).min(72.0);
                        }
                    }
                    "show_root_table" => {
                        show_root_table = value.eq_ignore_ascii_case("true") ||
                                         value == "1" ||
                                         value.eq_ignore_ascii_case("yes");
                    }
                    "root_table_scale" => {
                        if let Ok(scale) = value.parse::<f32>() {
                            root_table_scale = scale.max(0.1).min(2.0);
                        }
                    }
                    "window_width" => {
                        if let Ok(w) = value.parse::<f32>() {
                            window_width = w.max(800.0).min(3840.0);
                        }
                    }
                    "window_height" => {
                        if let Ok(h) = value.parse::<f32>() {
                            window_height = h.max(600.0).min(2160.0);
                        }
                    }
                    "root_table_position" => {
                        if let Some(pos) = RootTablePosition::from_str(value) {
                            root_table_position = pos;
                        }
                    }
                    _ => {}
                }
            }
        }

        // 如果沒有設定字型，使用預設
        if font_path.is_empty() {
            font_path = get_default_font_path();
        }

        Ok(Self {
            font_path,
            font_size,
            show_root_table,
            root_table_scale,
            window_width,
            window_height,
            root_table_position,
        })
    }

    /// 儲存設定檔
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(path) = Self::config_file_path() {
            let content = format!(
                "# Array30 Input Method Settings\n\
                 # 設定檔\n\
                 \n\
                 # Font file path (字型檔案路徑)\n\
                 font_path={}\n\
                 \n\
                 # Font size in points (字型大小)\n\
                 font_size={}\n\
                 \n\
                 # Show root table image (顯示字根表)\n\
                 show_root_table={}\n\
                 \n\
                 # Root table image scale (字根表縮放比例 0.1-2.0)\n\
                 root_table_scale={}\n\
                 \n\
                 # Window size (視窗大小)\n\
                 window_width={}\n\
                 window_height={}\n\
                 \n\
                 # Root table position (字根表位置: up/down/left/right)\n\
                 root_table_position={}",
                self.font_path,
                self.font_size,
                self.show_root_table,
                self.root_table_scale,
                self.window_width,
                self.window_height,
                self.root_table_position.as_str()
            );

            std::fs::write(&path, content)?;
            Ok(())
        } else {
            Err("無法取得設定檔路徑".into())
        }
    }

    /// 載入字型資料
    pub fn load_font_data(&self) -> Option<Vec<u8>> {
        std::fs::read(&self.font_path).ok()
    }
}

/// 取得預設字型路徑 (Microsoft JhengHei)
#[cfg(target_os = "windows")]
fn get_default_font_path() -> String {
    let font_paths = [
        r"C:\Windows\Fonts\msjh.ttc",
        r"C:\Windows\Fonts\MSJH.TTC",
        r"C:\Windows\Fonts\msjh.ttf",
        r"C:\Windows\Fonts\MSJH.TTF",
    ];

    for path in &font_paths {
        if Path::new(path).exists() {
            return path.to_string();
        }
    }

    // 如果找不到，返回第一個選項（讓系統處理錯誤）
    font_paths[1].to_string()
}

/// 取得預設字型路徑 (非 Windows)
#[cfg(not(target_os = "windows"))]
fn get_default_font_path() -> String {
    // 非 Windows 系統使用空字串，讓 egui 使用預設字型
    String::new()
}

/// 列出 Windows 字型目錄中的字型檔案
#[cfg(target_os = "windows")]
pub fn list_system_fonts() -> Vec<FontInfo> {
    let fonts_dir = PathBuf::from(r"C:\Windows\Fonts");
    let mut font_list = Vec::new();

    if let Ok(entries) = std::fs::read_dir(&fonts_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                let ext_lower = ext.to_string_lossy().to_lowercase();
                if ext_lower == "ttf" || ext_lower == "ttc" || ext_lower == "otf" {
                    let file_name = path.file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    let display_name = format_font_name(&file_name);

                    font_list.push(FontInfo {
                        name: display_name,
                        file_name,
                        path: path.to_string_lossy().to_string(),
                    });
                }
            }
        }
    }

    // 排序：常用字型優先
    font_list.sort_by(|a, b| {
        let a_priority = get_font_priority(&a.file_name);
        let b_priority = get_font_priority(&b.file_name);
        b_priority.cmp(&a_priority)
    });

    font_list
}

/// 字型資訊
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontInfo {
    pub name: String,
    pub file_name: String,
    pub path: String,
}

/// 格式化字型名稱顯示
fn format_font_name(filename: &str) -> String {
    let name = filename.to_lowercase();

    // 常用中文字型名稱映射
    let display: &str = match name.as_str() {
        "msjh.ttc" | "msjh.ttf" | "msjhbd.ttc" | "msjhbd.ttf" => "Microsoft JhengHei (微軟正黑體)",
        "msyh.ttc" | "msyh.ttf" | "msyhbd.ttc" | "msyhbd.ttf" => "Microsoft YaHei (微軟雅黑)",
        "kaiu.ttf" => "DFKai-SB (標楷體)",
        "mingliu.ttc" | "mingliu.ttf" => "PMingLiU (新細明體)",
        "simhei.ttf" => "SimHei (黑體)",
        "simsun.ttc" => "SimSun (宋體)",
        _ => {
            // 移除副檔名，返回字串切片
            let end = filename.len().saturating_sub(4);
            &filename[..end]
        }
    };

    display.to_string()
}

/// 取得字型優先級（用於排序）
fn get_font_priority(filename: &str) -> i32 {
    let name = filename.to_lowercase();
    match name.as_str() {
        "msjh.ttc" => 100,
        "msjh.ttf" => 99,
        "msjhbd.ttc" => 98,
        "msjhbd.ttf" => 97,
        "msyh.ttc" => 90,
        "msyh.ttf" => 89,
        "kaiu.ttf" => 80,
        "mingliu.ttc" => 70,
        "mingliu.ttf" => 69,
        _ => 0,
    }
}

/// 列出系統字型（非 Windows，返回空列表）
#[cfg(not(target_os = "windows"))]
pub fn list_system_fonts() -> Vec<FontInfo> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(!config.font_path.is_empty());
        assert_eq!(config.font_size, 20.0);
    }
}
