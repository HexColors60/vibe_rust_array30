// Windows GUI using egui/eframe
// Windows 圖形介面

use crate::dict::Dictionary;
use crate::input_engine::InputEngine;
use eframe::egui;
use std::io::{self, Write};
use std::path::PathBuf;

#[cfg(target_os = "windows")]
use crossterm::{
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

pub struct GuiApp {
    engine: InputEngine,
    phrase_file_path: PathBuf,
    cin2_file_path: PathBuf,
    clipboard_content: String,
    show_about: bool,
}

impl GuiApp {
    pub fn new(dict: Dictionary, phrase_file: PathBuf, cin2_file: PathBuf) -> Self {
        Self {
            engine: InputEngine::new(dict),
            phrase_file_path: phrase_file,
            cin2_file_path: cin2_file,
            clipboard_content: String::new(),
            show_about: false,
        }
    }
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("檔案", |ui| {
                    if ui.button("重新載入詞庫").clicked() {
                        // TODO: 實作重新載入
                    }
                    if ui.button("清除輸出").clicked() {
                        self.engine.clear_output();
                    }
                    if ui.button("退出").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("說明", |ui| {
                    if ui.button("關於").clicked() {
                        self.show_about = true;
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("行列 30 輸入法");
            ui.separator();

            // 複製需要使用的狀態資料
            let raw_keys = self.engine.state().raw_keys.clone();
            let current_code = self.engine.state().current_code.clone();
            let output = self.engine.state().output.clone();
            let hint = self.engine.state().get_hint();
            let candidates: Vec<_> = self.engine.current_page_candidates().to_vec();
            let has_candidates = !candidates.is_empty();

            // 鍵盤輸入區
            ui.group(|ui| {
                ui.label("鍵盤輸入區：");
                ui.horizontal(|ui| {
                    ui.label(&raw_keys);
                });
            });

            // 編輯區
            ui.group(|ui| {
                ui.label("編輯區：");
                if !current_code.is_empty() {
                    ui.horizontal(|ui| {
                        ui.label(format!("碼：{}", current_code));
                    });

                    // 候選列表
                    if has_candidates {
                        ui.separator();
                        ui.label("候選字/詞：");
                        ui.horizontal_wrapped(|ui| {
                            for (i, cand) in candidates.iter().enumerate() {
                                let cand_text = cand.text.clone();
                                if ui.button(format!("[{}] {}", i + 1, cand_text)).clicked() {
                                    self.engine.select_candidate(i);
                                }
                            }
                        });

                        // 分頁按鈕
                        ui.horizontal(|ui| {
                            if ui.button("◄ 上一頁").clicked() {
                                self.engine.prev_page();
                            }
                            if ui.button("下一頁 ►").clicked() {
                                self.engine.next_page();
                            }
                        });
                    } else {
                        ui.label("（無候選字）");
                    }
                } else {
                    ui.label("（空）");
                }
            });

            // 輸出區
            ui.group(|ui| {
                ui.label("輸出區：");
                egui::ScrollArea::vertical()
                    .max_height(100.0)
                    .show(ui, |ui| {
                        if output.is_empty() {
                            ui.label("（空）");
                        } else {
                            ui.label(&output);
                        }
                    });
            });

            // 提示區
            ui.group(|ui| {
                ui.label("提示：");
                ui.label(hint);
            });

            // 複製按鈕
            ui.horizontal(|ui| {
                if ui.button("📋 複製輸出到剪貼簿").clicked() {
                    let output_text = self.engine.get_output_text();
                    if let Some(mut clipboard) = arboard::Clipboard::new().ok() {
                        let _ = clipboard.set_text(&output_text);
                        self.clipboard_content = output_text;
                    }
                }

                if !self.clipboard_content.is_empty() {
                    ui.label(format!("已複製 {} 字元", self.clipboard_content.len()));
                }
            });

            // 檔案資訊
            ui.separator();
            ui.label(format!("詞庫：{}", self.phrase_file_path.display()));
            ui.label(format!("字表：{}", self.cin2_file_path.display()));

            // 鍵盤輸入處理
            ui.input(|i| {
                for event in &i.events {
                    if let egui::Event::Key { key, pressed: true, .. } = event {
                        self.handle_egui_key(key);
                    }
                    if let egui::Event::Text(text) = event {
                        for c in text.chars() {
                            // 只處理可見字元
                            if c.is_ascii() && !c.is_ascii_control() {
                                self.engine.handle_key(c);
                            }
                        }
                    }
                }
            });

            // 請求自動重繪以處理鍵盤輸入
            ctx.request_repaint();
        });

        // 關於對話框
        if self.show_about {
            egui::Window::new("關於行列 30 輸入法")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("行列 30 輸入法");
                    ui.label("Rust 實作版本");
                    ui.separator();
                    ui.label("操作說明：");
                    ui.label("• 直接輸入英文字母作為行列碼");
                    ui.label("• 按 ' 進入詞彙輸入模式");
                    ui.label("• 數字鍵 1-9 選擇候選字");
                    ui.label("• 空白鍵或 Enter 確認第一候選");
                    ui.label("• Backspace 刪除");
                    ui.label("• Esc 清空編輯區");
                    ui.separator();
                    if ui.button("關閉").clicked() {
                        self.show_about = false;
                    }
                });
        }
    }
}

impl GuiApp {
    fn handle_egui_key(&mut self, key: &egui::Key) {
        match key {
            egui::Key::Backspace => {
                self.engine.handle_key('\x08');
            }
            egui::Key::Enter => {
                self.engine.handle_key('\n');
            }
            egui::Key::Escape => {
                self.engine.handle_key('\x1b');
            }
            egui::Key::Space => {
                self.engine.handle_key(' ');
            }
            _ => {}
        }
    }
}

pub fn run_gui(dict: Dictionary, phrase_file: PathBuf, cin2_file: PathBuf) -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([600.0, 400.0])
            .with_title("行列 30 輸入法"),
        ..Default::default()
    };

    eframe::run_native(
        "行列 30 輸入法",
        options,
        Box::new(|_cc| {
            Ok(Box::new(GuiApp::new(dict, phrase_file, cin2_file)))
        }),
    )
}

/// 終端機模式（跨平台）
pub fn run_console_mode(dict: Dictionary) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    let mut engine = InputEngine::new(dict);
    let mut should_quit = false;

    while !should_quit {
        // 繪製介面
        execute!(stdout, Clear(ClearType::All), crossterm::cursor::MoveTo(0, 0))?;

        let state = engine.state();
        let candidates = engine.current_page_candidates();

        // 第一行：標題
        println!("行列 30 輸入法 - 終端機模式");
        println!();

        // 第二行：鍵盤輸入區
        println!("鍵盤輸入：{}", state.raw_keys);
        println!();

        // 第三行：編輯區
        if !state.current_code.is_empty() {
            println!("編輯區：碼 = {}", state.current_code);
            if !candidates.is_empty() {
                print!("候選：");
                for (i, cand) in candidates.iter().enumerate() {
                    print!("[{}]{} ", i + 1, cand.text);
                }
                println!();
            } else {
                println!("編輯區：無候選字");
            }
        } else {
            println!("編輯區：（空）");
        }
        println!();

        // 第四行：輸出區
        let output = if state.output.is_empty() {
            "（空）"
        } else {
            &state.output
        };
        println!("輸出區：{}", output);
        println!();

        // 第五行：提示區
        let hint = state.get_hint();
        println!("提示：{}", hint);
        println!();
        println!("按 Ctrl+C 或 Ctrl+Q 離開");

        stdout.flush()?;

        // 讀取按鍵
        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                should_quit = handle_console_key_event(&mut engine, key);
            }
        }
    }

    // 清理
    disable_raw_mode()?;
    execute!(stdout, Clear(ClearType::All))?;
    println!("行列 30 輸入法 - 再見！");

    Ok(())
}

fn handle_console_key_event(engine: &mut InputEngine, key: KeyEvent) -> bool {
    match key.code {
        // 退出
        KeyCode::Char('c') | KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            return true;
        }

        // 退格
        KeyCode::Backspace => {
            engine.handle_key('\x08');
        }

        // Enter
        KeyCode::Enter => {
            engine.handle_key('\n');
        }

        // 空白
        KeyCode::Char(' ') => {
            engine.handle_key(' ');
        }

        // Esc
        KeyCode::Esc => {
            engine.handle_key('\x1b');
        }

        // 一般字元
        KeyCode::Char(c) => {
            engine.handle_key(c);
        }

        // 分頁
        KeyCode::PageDown | KeyCode::Tab => {
            engine.next_page();
        }
        KeyCode::PageUp => {
            if key.modifiers.contains(KeyModifiers::SHIFT) {
                engine.prev_page();
            }
        }

        _ => {}
    }
    false
}
