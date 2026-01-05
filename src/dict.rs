// Dictionary loading for Array30
// 字典與詞庫載入

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// 字典結構
#[derive(Debug, Clone)]
pub struct Dictionary {
    /// 單字碼表：code -> vec of characters
    char_table: HashMap<String, Vec<String>>,
    /// 詞彙碼表：code -> vec of phrases
    phrase_table: HashMap<String, Vec<String>>,
}

impl Default for Dictionary {
    fn default() -> Self {
        Self::new()
    }
}

impl Dictionary {
    pub fn new() -> Self {
        Self {
            char_table: HashMap::new(),
            phrase_table: HashMap::new(),
        }
    }

    /// 載入詞彙檔 (array30-phrase-20210725.txt)
    /// 格式: ,,,/	燦爛
    /// 第一欄是碼，第二欄是詞彙，以 tab 分隔
    pub fn load_phrase_file<P: AsRef<Path>>(&mut self, path: P) -> std::io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();

            // 跳過空行和註解
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // 解析行：格式為 "code\tword"
            if let Some((code_part, word_part)) = line.split_once('\t') {
                let code = code_part.trim().to_string();
                let word = word_part.trim().to_string();

                if !code.is_empty() && !word.is_empty() {
                    self.phrase_table
                        .entry(code)
                        .or_insert_with(Vec::new)
                        .push(word);
                }
            }
        }

        Ok(())
    }

    /// 載入 cin2 格式的字表
    /// %chardef 開始後的行為 "code\tchar"
    pub fn load_cin2_file<P: AsRef<Path>>(&mut self, path: P) -> std::io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut in_chardef = false;

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();

            // 檢查是否進入 chardef 區塊
            if line == "%chardef begin" {
                in_chardef = true;
                continue;
            }
            if line == "%chardef end" {
                in_chardef = false;
                continue;
            }

            // 只在 chardef 區塊內解析
            if !in_chardef {
                continue;
            }

            // 跳過空行和註解
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // 解析行：格式為 "code\tchar"
            if let Some((code_part, char_part)) = line.split_once('\t') {
                let code = code_part.trim().to_string();
                let char_str = char_part.trim().to_string();

                if !code.is_empty() && !char_str.is_empty() {
                    self.char_table
                        .entry(code)
                        .or_insert_with(Vec::new)
                        .push(char_str);
                }
            }
        }

        Ok(())
    }

    /// 查找單字候選
    pub fn lookup_chars(&self, code: &str) -> Option<&[String]> {
        self.char_table.get(code).map(|v| v.as_slice())
    }

    /// 查找詞彙候選
    pub fn lookup_phrases(&self, code: &str) -> Option<&[String]> {
        self.phrase_table.get(code).map(|v| v.as_slice())
    }

    /// 檢查碼是否存在（單字或詞彙）
    pub fn has_code(&self, code: &str) -> bool {
        self.char_table.contains_key(code) || self.phrase_table.contains_key(code)
    }

    /// 取得統計資訊
    pub fn stats(&self) -> (usize, usize) {
        (self.char_table.len(), self.phrase_table.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dictionary_creation() {
        let dict = Dictionary::new();
        assert_eq!(dict.stats(), (0, 0));
        assert!(!dict.has_code("test"));
    }

    #[test]
    fn test_lookup_empty() {
        let dict = Dictionary::new();
        assert!(dict.lookup_chars("abc").is_none());
        assert!(dict.lookup_phrases("abc").is_none());
    }
}
