// Input Engine for Array30
// 行列 30 輸入法引擎

use crate::dict::Dictionary;
use crate::keymap::Array30Key;
use crate::state::{Candidate, InputMode, InputState};

/// 輸入法引擎
pub struct InputEngine {
    /// 字典
    dict: Dictionary,
    /// 當前狀態
    state: InputState,
    /// 候選列表
    candidates: Vec<Candidate>,
    /// 候選頁面索引
    page_index: usize,
    /// 每頁顯示候選數
    page_size: usize,
}

impl InputEngine {
    pub fn new(dict: Dictionary) -> Self {
        Self {
            dict,
            state: InputState::new(),
            candidates: Vec::new(),
            page_index: 0,
            page_size: 9, // 1-9 鍵選字
        }
    }

    /// 載入字典
    pub fn load_dict(&mut self, dict: Dictionary) {
        self.dict = dict;
    }

    /// 處理按鍵輸入
    /// 回傳是否需要重新整理介面
    pub fn handle_key(&mut self, key: char) -> KeyResult {
        match key {
            // 詞彙終結鍵
            '\'' => {
                if self.state.current_code.len() >= 1 && self.state.current_code.len() <= 4 {
                    self.state.set_phrase_mode();
                    self.update_candidates();
                    KeyResult::NeedUpdate
                } else {
                    // 碼數不正確
                    KeyResult::NeedUpdate
                }
            }

            // 退格鍵
            '\x08' | '\x7f' => {
                // 先清空候選
                if !self.candidates.is_empty() {
                    self.candidates.clear();
                    self.page_index = 0;
                }
                if self.state.backspace() {
                    self.update_candidates();
                }
                KeyResult::NeedUpdate
            }

            // Esc 清空
            '\x1b' => {
                self.state.clear_composing();
                self.candidates.clear();
                self.page_index = 0;
                KeyResult::NeedUpdate
            }

            // Enter 或空白確認第一候選
            '\n' | '\r' | ' ' => {
                if !self.candidates.is_empty() {
                    self.select_candidate(0);
                    KeyResult::NeedUpdate
                } else if !self.state.current_code.is_empty() {
                    // 沒有候選但有碼，嘗試直接上屏
                    KeyResult::NeedUpdate
                } else {
                    KeyResult::NoChange
                }
            }

            // 數字鍵選字
            '1'..='9' => {
                if !self.candidates.is_empty() {
                    let idx = (key as usize) - ('1' as usize);
                    if self.select_candidate(idx) {
                        KeyResult::Committed
                    } else {
                        KeyResult::NeedUpdate
                    }
                } else {
                    // 數字鍵可能直接輸出
                    self.state.commit_direct(&key.to_string());
                    KeyResult::Committed
                }
            }
            '0' => {
                if !self.candidates.is_empty() {
                    self.select_candidate(9);
                    KeyResult::Committed
                } else {
                    self.state.commit_direct(&key.to_string());
                    KeyResult::Committed
                }
            }

            // 行列鍵輸入
            c if Array30Key::from_char(c).is_some() => {
                // 如果已有候選列表，先清空
                if !self.candidates.is_empty() {
                    self.candidates.clear();
                    self.page_index = 0;
                }

                self.state.add_key(c);

                // 根據模式處理
                if self.state.mode == InputMode::PhraseInput {
                    // 詞彙模式：只接受 4 碼
                    if self.state.current_code.len() < 4 {
                        self.state.current_code.push(c);
                    }
                } else {
                    // 一般模式：最多 4 碼
                    if self.state.current_code.len() < 4 {
                        self.state.current_code.push(c);
                    }
                }

                self.update_candidates();
                KeyResult::NeedUpdate
            }

            // 其他字元直接輸出
            _ => {
                // 先確認當前組字
                if !self.state.current_code.is_empty() {
                    self.state.clear_composing();
                }
                self.state.commit_direct(&key.to_string());
                KeyResult::Committed
            }
        }
    }

    /// 更新候選列表
    fn update_candidates(&mut self) {
        self.candidates.clear();
        self.page_index = 0;

        let code = &self.state.current_code;

        if code.is_empty() {
            return;
        }

        // 詞彙模式優先查找詞庫
        if self.state.mode == InputMode::PhraseInput {
            if let Some(phrases) = self.dict.lookup_phrases(code) {
                for phrase in phrases {
                    self.candidates
                        .push(Candidate::phrase(phrase.clone(), code.clone()));
                }
            }
        }

        // 一般模式查找字庫
        if self.candidates.is_empty() {
            if let Some(chars) = self.dict.lookup_chars(code) {
                for char_str in chars {
                    self.candidates
                        .push(Candidate::char(char_str.clone(), code.clone()));
                }
            }
        }
    }

    /// 選擇候選字
    /// 回傳是否成功選擇
    pub fn select_candidate(&mut self, index: usize) -> bool {
        let actual_index = self.page_index * self.page_size + index;

        if actual_index < self.candidates.len() {
            let candidate = self.candidates[actual_index].clone();
            self.state.composing = candidate.text;
            self.state.commit_composing();
            self.candidates.clear();
            self.page_index = 0;
            true
        } else {
            false
        }
    }

    /// 取得當前狀態的唯讀參考
    pub fn state(&self) -> &InputState {
        &self.state
    }

    /// 取得當前候選列表
    pub fn candidates(&self) -> &[Candidate] {
        &self.candidates
    }

    /// 取得當前頁面的候選
    pub fn current_page_candidates(&self) -> &[Candidate] {
        let start = self.page_index * self.page_size;
        let end = (start + self.page_size).min(self.candidates.len());
        &self.candidates[start..end]
    }

    /// 下一頁
    pub fn next_page(&mut self) -> bool {
        let total_pages = (self.candidates.len() + self.page_size - 1) / self.page_size;
        if self.page_index + 1 < total_pages {
            self.page_index += 1;
            true
        } else {
            false
        }
    }

    /// 上一頁
    pub fn prev_page(&mut self) -> bool {
        if self.page_index > 0 {
            self.page_index -= 1;
            true
        } else {
            false
        }
    }

    /// 清空輸出區
    pub fn clear_output(&mut self) {
        self.state.clear_all();
    }

    /// 複製輸出區文字
    pub fn get_output_text(&self) -> String {
        self.state.output.clone()
    }
}

/// 按鍵處理結果
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyResult {
    /// 無變化
    NoChange,
    /// 需要更新介面顯示
    NeedUpdate,
    /// 已確認輸出（需要更新剪貼簿等）
    Committed,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_dict() -> Dictionary {
        let mut dict = Dictionary::new();
        // 測試用簡單數據
        dict.char_table
            .entry("abc".to_string())
            .or_insert_with(Vec::new)
            .push("測".to_string());
        dict.phrase_table
            .entry("abcd".to_string())
            .or_insert_with(Vec::new)
            .push("測試".to_string());
        dict
    }

    #[test]
    fn test_engine_creation() {
        let dict = create_test_dict();
        let engine = InputEngine::new(dict);
        assert!(engine.state().current_code.is_empty());
        assert!(engine.candidates().is_empty());
    }

    #[test]
    fn test_handle_key() {
        let dict = create_test_dict();
        let mut engine = InputEngine::new(dict);

        // 輸入 'a'
        let result = engine.handle_key('a');
        assert_eq!(result, KeyResult::NeedUpdate);
        assert_eq!(engine.state().current_code, "a");

        // 輸入 'b'
        engine.handle_key('b');
        assert_eq!(engine.state().current_code, "ab");

        // 輸入 'c'
        engine.handle_key('c');
        assert_eq!(engine.state().current_code, "abc");
    }

    #[test]
    fn test_backspace() {
        let dict = create_test_dict();
        let mut engine = InputEngine::new(dict);

        engine.handle_key('a');
        engine.handle_key('b');
        engine.handle_key('\x08');
        assert_eq!(engine.state().current_code, "a");
    }
}
