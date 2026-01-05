// Key mapping for Array30 Input Method
// 行列 30 鍵位配置

/// Array30 鍵盤配置
/// 將行列鍵碼對應到實際按鍵
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Array30Key {
    A = 0,  // 1-
    B,      // 5v
    C,      // 3v
    D,      // 3-
    E,      // 3^
    F,      // 4-
    G,      // 5-
    H,      // 6-
    I,      // 8^
    J,      // 7-
    K,      // 8-
    L,      // 9-
    M,      // 7v
    N,      // 6v
    O,      // 9^
    P,      // 0^
    Q,      // 1^
    R,      // 4^
    S,      // 2-
    T,      // 5^
    U,      // 7^
    V,      // 4v
    W,      // 2^
    X,      // 2v
    Y,      // 6^
    Z,      // 1v
    Period, // 9v
    Slash,  // 0v
    Semicolon, // 0-
    Comma,  // 8v
}

impl Array30Key {
    /// 從字元轉換為 Array30Key
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'a' | 'A' => Some(Array30Key::A),
            'b' | 'B' => Some(Array30Key::B),
            'c' | 'C' => Some(Array30Key::C),
            'd' | 'D' => Some(Array30Key::D),
            'e' | 'E' => Some(Array30Key::E),
            'f' | 'F' => Some(Array30Key::F),
            'g' | 'G' => Some(Array30Key::G),
            'h' | 'H' => Some(Array30Key::H),
            'i' | 'I' => Some(Array30Key::I),
            'j' | 'J' => Some(Array30Key::J),
            'k' | 'K' => Some(Array30Key::K),
            'l' | 'L' => Some(Array30Key::L),
            'm' | 'M' => Some(Array30Key::M),
            'n' | 'N' => Some(Array30Key::N),
            'o' | 'O' => Some(Array30Key::O),
            'p' | 'P' => Some(Array30Key::P),
            'q' | 'Q' => Some(Array30Key::Q),
            'r' | 'R' => Some(Array30Key::R),
            's' | 'S' => Some(Array30Key::S),
            't' | 'T' => Some(Array30Key::T),
            'u' | 'U' => Some(Array30Key::U),
            'v' | 'V' => Some(Array30Key::V),
            'w' | 'W' => Some(Array30Key::W),
            'x' | 'X' => Some(Array30Key::X),
            'y' | 'Y' => Some(Array30Key::Y),
            'z' | 'Z' => Some(Array30Key::Z),
            '.' => Some(Array30Key::Period),
            '/' => Some(Array30Key::Slash),
            ';' => Some(Array30Key::Semicolon),
            ',' => Some(Array30Key::Comma),
            '\'' => Some(Array30Key::Slash), // ' 用於詞彙輸入，映射到 Slash
            _ => None,
        }
    }

    /// 取得鍵的字元代碼（用於組碼）
    pub fn code_char(&self) -> char {
        match self {
            Array30Key::A => 'a',
            Array30Key::B => 'b',
            Array30Key::C => 'c',
            Array30Key::D => 'd',
            Array30Key::E => 'e',
            Array30Key::F => 'f',
            Array30Key::G => 'g',
            Array30Key::H => 'h',
            Array30Key::I => 'i',
            Array30Key::J => 'j',
            Array30Key::K => 'k',
            Array30Key::L => 'l',
            Array30Key::M => 'm',
            Array30Key::N => 'n',
            Array30Key::O => 'o',
            Array30Key::P => 'p',
            Array30Key::Q => 'q',
            Array30Key::R => 'r',
            Array30Key::S => 's',
            Array30Key::T => 't',
            Array30Key::U => 'u',
            Array30Key::V => 'v',
            Array30Key::W => 'w',
            Array30Key::X => 'x',
            Array30Key::Y => 'y',
            Array30Key::Z => 'z',
            Array30Key::Period => '.',
            Array30Key::Slash => '/',
            Array30Key::Semicolon => ';',
            Array30Key::Comma => ',',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_conversion() {
        assert_eq!(Array30Key::from_char('a'), Some(Array30Key::A));
        assert_eq!(Array30Key::from_char('A'), Some(Array30Key::A));
        assert_eq!(Array30Key::from_char('.'), Some(Array30Key::Period));
        assert_eq!(Array30Key::from_char('\''), Some(Array30Key::Slash));
        assert_eq!(Array30Key::from_char('1'), None);
    }
}
