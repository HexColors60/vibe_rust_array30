// Console interface for Linux/Unix
// 終端機介面（Linux 文字模式）

use crate::dict::Dictionary;
use crate::input_engine::InputEngine;
use crossterm::{
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::io::{self, Write};

pub struct ConsoleApp {
    engine: InputEngine,
    should_quit: bool,
}

impl ConsoleApp {
    pub fn new(dict: Dictionary) -> Self {
        Self {
            engine: InputEngine::new(dict),
            should_quit: false,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();

        self.should_quit = false;

        while !self.should_quit {
            // 繪製介面
            self.draw(&mut stdout)?;

            // 讀取按鍵
            if event::poll(std::time::Duration::from_millis(100))? {
                if let event::Event::Key(key) = event::read()? {
                    self.handle_key_event(key);
                }
            }
        }

        // 清理
        disable_raw_mode()?;
        execute!(stdout, Clear(ClearType::All))?;
        println!("行列 30 輸入法 - 再見！");

        Ok(())
    }

    fn draw(&self, stdout: &mut io::Stdout) -> io::Result<()> {
        execute!(stdout, Clear(ClearType::All), crossterm::cursor::MoveTo(0, 0))?;

        let state = self.engine.state();
        let candidates = self.engine.current_page_candidates();

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
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            // 退出
            KeyCode::Char('c') | KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }

            // 退格
            KeyCode::Backspace => {
                self.engine.handle_key('\x08');
            }

            // Enter
            KeyCode::Enter => {
                self.engine.handle_key('\n');
            }

            // 空白
            KeyCode::Char(' ') => {
                self.engine.handle_key(' ');
            }

            // Esc
            KeyCode::Esc => {
                self.engine.handle_key('\x1b');
            }

            // 一般字元
            KeyCode::Char(c) => {
                self.engine.handle_key(c);
            }

            // 分頁（PageDown/PageUp 或 tab/shift+tab）
            KeyCode::PageDown | KeyCode::Tab => {
                self.engine.next_page();
            }
            KeyCode::PageUp => {
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    self.engine.prev_page();
                }
            }

            _ => {}
        }
    }
}

pub fn run_console(dict: Dictionary) -> io::Result<()> {
    let mut app = ConsoleApp::new(dict);
    app.run()
}
