// Input state management for Array30
// 輸入狀態機

/// 輸入模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    /// 一般字詞輸入
    Normal,
    /// 詞彙輸入模式（已按下 ' 等待詞碼）
    PhraseInput,
}

/// 輸入狀態
#[derive(Debug, Clone)]
pub struct InputState {
    /// 原始鍵序區：使用者輸入的按鍵序列
    pub raw_keys: String,
    /// 編輯區：已確定的漢字或詞彙（尚未上屏）
    pub composing: String,
    /// 輸出區：已經確定輸出的文字
    pub output: String,
    /// 目前輸入模式
    pub mode: InputMode,
    /// 當前輸入的碼
    pub current_code: String,
    /// 是否有詞彙終結符
    pub has_phrase_marker: bool,
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}

impl InputState {
    pub fn new() -> Self {
        Self {
            raw_keys: String::new(),
            composing: String::new(),
            output: String::new(),
            mode: InputMode::Normal,
            current_code: String::new(),
            has_phrase_marker: false,
        }
    }

    /// 清空編輯區
    pub fn clear_composing(&mut self) {
        self.raw_keys.clear();
        self.composing.clear();
        self.current_code.clear();
        self.has_phrase_marker = false;
        self.mode = InputMode::Normal;
    }

    /// 清空全部
    pub fn clear_all(&mut self) {
        self.clear_composing();
        self.output.clear();
    }

    /// 添加按鍵到原始鍵序
    pub fn add_key(&mut self, key: char) {
        self.raw_keys.push(key);
    }

    /// 設置為詞彙輸入模式
    pub fn set_phrase_mode(&mut self) {
        self.mode = InputMode::PhraseInput;
        self.has_phrase_marker = true;
        self.add_key('\'');
    }

    /// 更新當前碼
    pub fn update_code(&mut self, code: String) {
        self.current_code = code;
    }

    /// 將編輯區內容移到輸出區
    pub fn commit_composing(&mut self) {
        if !self.composing.is_empty() {
            self.output.push_str(&self.composing);
            self.clear_composing();
        }
    }

    /// 直接添加文字到輸出區
    pub fn commit_direct(&mut self, text: &str) {
        self.output.push_str(text);
    }

    /// 退格：刪除最後一個字元
    pub fn backspace(&mut self) -> bool {
        if self.current_code.pop().is_some() {
            if let Some(c) = self.raw_keys.pop() {
                // 如果刪除的是詞彙標記，退出詞彙模式
                if c == '\'' {
                    self.mode = InputMode::Normal;
                    self.has_phrase_marker = false;
                }
                return true;
            }
        }
        false
    }

    /// 取得提示文字
    pub fn get_hint(&self) -> String {
        match self.mode {
            InputMode::Normal => {
                "提示：按 ' 進入詞彙輸入；空白鍵上第一候選；數字鍵選字；Esc 清空".to_string()
            }
            InputMode::PhraseInput => {
                "詞彙模式：輸入四碼後會自動查找詞庫".to_string()
            }
        }
    }
}

/// 候選項
#[derive(Debug, Clone)]
pub struct Candidate {
    /// 顯示文字（漢字或詞彙）
    pub text: String,
    /// 對應的行列碼
    pub code: String,
    /// 是否為詞彙
    pub is_phrase: bool,
}

impl Candidate {
    pub fn new(text: String, code: String, is_phrase: bool) -> Self {
        Self {
            text,
            code,
            is_phrase,
        }
    }

    pub fn char(text: String, code: String) -> Self {
        Self::new(text, code, false)
    }

    pub fn phrase(text: String, code: String) -> Self {
        Self::new(text, code, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_initialization() {
        let state = InputState::new();
        assert!(state.raw_keys.is_empty());
        assert!(state.composing.is_empty());
        assert!(state.output.is_empty());
        assert_eq!(state.mode, InputMode::Normal);
    }

    #[test]
    fn test_add_key() {
        let mut state = InputState::new();
        state.add_key('a');
        state.add_key('b');
        assert_eq!(state.raw_keys, "ab");
    }

    #[test]
    fn test_backspace() {
        let mut state = InputState::new();
        state.raw_keys = "abc".to_string();
        state.current_code = "abc".to_string();
        assert!(state.backspace());
        assert_eq!(state.raw_keys, "ab");
        assert_eq!(state.current_code, "ab");
    }

    #[test]
    fn test_commit() {
        let mut state = InputState::new();
        state.composing = "台灣".to_string();
        state.commit_composing();
        assert_eq!(state.output, "台灣");
        assert!(state.composing.is_empty());
    }
}
