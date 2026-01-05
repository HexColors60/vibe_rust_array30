// rustarray30 - Array30 Input Method in Rust
// 行列 30 輸入法 - 主程式

#![allow(dead_code)]

use std::env;
use std::path::PathBuf;

mod config;
mod dict;
mod input_engine;
mod keymap;
mod state;

// 平台特定模組
#[cfg(target_os = "windows")]
mod gui;

#[cfg(not(target_os = "windows"))]
mod console;

use dict::Dictionary;

#[cfg(target_os = "windows")]
use gui::run_gui;

#[cfg(not(target_os = "windows"))]
use console::run_console;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // 解析命令列參數
    let (use_big_char, mode) = parse_args(&args);

    // 取得表格檔案路徑
    let base_dir = PathBuf::from("table");
    let phrase_file = base_dir.join("array30-phrase-20210725.txt");

    let cin2_dir = base_dir.join("cin2");
    let char_file = if use_big_char {
        cin2_dir.join("ar30-big-v2023-1.0-20251012.cin2")
    } else {
        cin2_dir.join("ar30-regular-v2023-1.0-20251012.cin2")
    };

    // 載入字典
    println!("載入詞庫：{}", phrase_file.display());
    println!("載入字表：{}", char_file.display());

    let mut dict = Dictionary::new();

    if let Err(e) = dict.load_phrase_file(&phrase_file) {
        eprintln!("無法載入詞庫檔：{}", e);
        eprintln!("請確保檔案存在於：{}", phrase_file.display());
        return Err(e.into());
    }

    if let Err(e) = dict.load_cin2_file(&char_file) {
        eprintln!("無法載入字表檔：{}", e);
        eprintln!("請確保檔案存在於：{}", char_file.display());
        return Err(e.into());
    }

    let (char_count, phrase_count) = dict.stats();
    println!("已載入 {} 個字碼、{} 個詞碼", char_count, phrase_count);
    println!();

    // 根據平台執行對應介面
    #[cfg(target_os = "windows")]
    {
        match mode.as_deref() {
            Some("console") => {
                println!("以終端機模式執行...");
                // Windows 也使用 GUI 模組中的 console 功能
                // 或者可以實作一個跨平台的 console 模式
                gui::run_console_mode(dict)?;
            }
            _ => {
                println!("以 GUI 模式執行...");
                run_gui(dict, phrase_file, char_file)?;
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        println!("以終端機模式執行...");
        run_console(dict)?;
    }

    Ok(())
}

/// 解析命令列參數
/// 回傳 (是否使用大字集, 模式)
fn parse_args(args: &[String]) -> (bool, Option<String>) {
    let mut use_big_char = false;
    let mut mode = None;

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--big" | "-b" => {
                use_big_char = true;
            }
            "--console" | "-c" => {
                mode = Some("console".to_string());
            }
            "--gui" | "-g" => {
                mode = Some("gui".to_string());
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            _ => {
                eprintln!("未知參數：{}", arg);
                print_help();
                std::process::exit(1);
            }
        }
    }

    (use_big_char, mode)
}

fn print_help() {
    println!("行列 30 輸入法 - Rust 實作版本");
    println!();
    println!("使用方法：");
    println!("  rustarray30 [選項]");
    println!();
    println!("選項：");
    println!("  --big, -b       使用大字集字表（預設使用標準版）");
    println!("  --console, -c   強制使用終端機模式（僅 Windows）");
    println!("  --gui, -g       強制使用 GUI 模式（僅 Windows，為預設）");
    println!("  --help, -h      顯示此說明");
    println!();
    println!("表格檔案位置：");
    println!("  詞庫：table/array30-phrase-20210725.txt");
    println!("  字表：table/cin2/ar30-regular-v2023-1.0-20251012.cin2");
    println!("       或 table/cin2/ar30-big-v2023-1.0-20251012.cin2（--big）");
}
