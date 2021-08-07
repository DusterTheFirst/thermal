use std::fmt::{self, Debug, Formatter};

macro_rules! extended_ascii {
    ($($char:literal => $code:literal),+) => {
        fn ascii_to_char(code: u8) -> char {
            match code {
                0x00..=0x7F => char::from_u32(code as u32).unwrap(), // ASCII
                $($code => $char,)+ // Extended ASCII definition
            }
        }

        fn char_to_ascii(c: char) -> Option<u8> {
            match c {
                '\u{0000}'..='\u{007F}' => Some(c as u8), // ASCII
                $($char => Some($code),)+ // Extended ASCII definition
                _ => None // Unimplemented char
            }
        }
    };
}

// https://cdn.cnetcontent.com/d7/88/d788137b-f955-4516-9ba6-0addce718f20.pdf#%5B%7B%22num%22%3A865%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22Fit%22%7D%5D
extended_ascii! {
    'Ç' => 128,
    'ü' => 129,
    'é' => 130,
    'â' => 131,
    'ä' => 132,
    'à' => 133,
    'å' => 134,
    'ç' => 135,
    'ê' => 136,
    'ë' => 137,
    'è' => 138,
    'ï' => 139,
    'î' => 140,
    'ì' => 141,
    'Ä' => 142,
    'Å' => 143,
    'É' => 144,
    'æ' => 145,
    'Æ' => 146,
    'ô' => 147,
    'ö' => 148,
    'ò' => 149,
    'û' => 150,
    'ù' => 151,
    'ÿ' => 152,
    'Ö' => 153,
    'Ü' => 154,
    '¢' => 155,
    '£' => 156,
    '¥' => 157, 
    '¶' => 158, // Debatable
    'ƒ' => 159,
    'á' => 160,
    'í' => 161,
    'ó' => 162,
    'ú' => 163,
    'ñ' => 164,
    'Ñ' => 165,
    'ª' => 166,
    'º' => 167,
    '¿' => 168,
    '⌐' => 169,
    '¬' => 170,
    '½' => 171,
    '¼' => 172,
    '¡' => 173,
    '«' => 174,
    '»' => 175,
    '░' => 176,
    '▒' => 177,
    '▓' => 178,
    '│' => 179,
    '┤' => 180,
    '╡' => 181,
    '╢' => 182,
    '╖' => 183,
    '╕' => 184,
    '╣' => 185,
    '║' => 186,
    '╗' => 187,
    '╝' => 188,
    '╜' => 189,
    '╛' => 190,
    '┐' => 191,
    '└' => 192,
    '┴' => 193,
    '┬' => 194,
    '├' => 195,
    '─' => 196,
    '┼' => 197,
    '╞' => 198,
    '╟' => 199,
    '╚' => 200,
    '╔' => 201,
    '╩' => 202,
    '╦' => 203,
    '╠' => 204,
    '═' => 205,
    '╬' => 206,
    '╧' => 207,
    '╨' => 208,
    '╤' => 209,
    '╥' => 210,
    '╙' => 211,
    '╘' => 212,
    '╒' => 213,
    '╓' => 214,
    '╫' => 215,
    '╪' => 216,
    '┘' => 217,
    '┌' => 218,
    '█' => 219,
    '▄' => 220,
    '▌' => 221,
    '▐' => 222,
    '▀' => 223,
    'α' => 224,
    'β' => 225,
    'Γ' => 226,
    'π' => 227,
    'Σ' => 228,
    'σ' => 229,
    'μ' => 230,
    'τ' => 231,
    'ϕ' => 232,
    'θ' => 233,
    'Ω' => 234,
    'δ' => 235,
    '∞' => 236,
    'ø' => 237,
    '∈' => 238,
    '∩' => 239,
    '≡' => 240,
    '±' => 241,
    '≥' => 242,
    '≤' => 243,
    '⌠' => 244,
    '⌡' => 245,
    '÷' => 246,
    '≈' => 247,
    '°' => 248,
    '•' => 249,
    '·' => 250,
    '√' => 251,
    'ⁿ' => 252,
    '²' => 253,
    '■' => 254,
    '\u{00A0}' => 255
}

pub struct PrinterChar(u8);

impl Debug for PrinterChar {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "PrinterChar({:?})", self.to_char())
    }
}

impl PrinterChar {
    pub fn to_char(&self) -> char {
        ascii_to_char(self.0)
    }

    pub fn from_char(c: char) -> Option<Self> {
        char_to_ascii(c).map(Self)
    }
}

pub struct PrinterString(Vec<PrinterChar>);

impl Debug for PrinterString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "PrinterString({:?})", self.to_string())
    }
}

impl PrinterString {
    pub fn to_string(&self) -> String {
        self.0.iter().map(|c| c.to_char()).collect()
    }

    pub fn bytes<'a>(&'a self) -> impl Iterator<Item = u8> + 'a {
        self.0.iter().map(|c| c.0)
    }

    pub fn from_str(s: &str) -> Option<Self> {
        Some(Self(
            s.chars()
                .map(PrinterChar::from_char)
                .collect::<Option<_>>()?,
        ))
    }
}
