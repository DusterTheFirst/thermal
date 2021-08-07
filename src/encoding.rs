use std::fmt::{self, Debug, Formatter};

macro_rules! map_char_to_code {
    ($($char:literal => $code:literal),+) => {
        fn char_from_code(code: u8) -> char {
            match code {
                $($code => $char,)+
            }
        }

        fn code_from_char(c: char) -> Option<u8> {
            match c {
                $($char => Some($code),)+
                _ => None
            }
        }
    };
}

// https://cdn.cnetcontent.com/d7/88/d788137b-f955-4516-9ba6-0addce718f20.pdf#%5B%7B%22num%22%3A865%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22Fit%22%7D%5D
map_char_to_code! {
    '\u{0000}' => 0,    // NUL
    '\u{0001}' => 1,
    '\u{0002}' => 2,
    '\u{0003}' => 3,
    '\u{0004}' => 4,    // EOT
    '\u{0005}' => 5,    // ENQ
    '\u{0006}' => 6,
    '\u{0007}' => 7,
    '\u{0008}' => 8,    // BS
    '\u{0009}' => 9,    // HT
    '\u{000A}' => 10,   // LF
    '\u{000B}' => 11,
    '\u{000C}' => 12,   // FF
    '\u{000D}' => 13,   // CR
    '\u{000E}' => 14,
    '\u{000F}' => 15,
    '\u{0010}' => 16,   // DLE
    '\u{0011}' => 17,   // XON
    '\u{0012}' => 18,
    '\u{0013}' => 19,   // XOFF
    '\u{0014}' => 20,   // DC4
    '\u{0015}' => 21,
    '\u{0016}' => 22,
    '\u{0017}' => 23,
    '\u{0018}' => 24,   // CAN
    '\u{0019}' => 25,
    '\u{001A}' => 26,
    '\u{001B}' => 27,
    '\u{001C}' => 28,
    '\u{001D}' => 29,
    '\u{001E}' => 30,
    '\u{001F}' => 31,
    '\u{0020}' => 32,
    '\u{0021}' => 33,
    '\u{0022}' => 34,
    '\u{0023}' => 35,
    '\u{0024}' => 36,
    '\u{0025}' => 37,
    '\u{0026}' => 38,
    '\u{0027}' => 39,
    '\u{0028}' => 40,
    '\u{0029}' => 41,
    '\u{002A}' => 42,
    '\u{002B}' => 43,
    '\u{002C}' => 44,
    '\u{002D}' => 45,
    '\u{002E}' => 46,
    '\u{002F}' => 47,
    '\u{0030}' => 48,
    '\u{0031}' => 49,
    '\u{0032}' => 50,
    '\u{0033}' => 51,
    '\u{0034}' => 52,
    '\u{0035}' => 53,
    '\u{0036}' => 54,
    '\u{0037}' => 55,
    '\u{0038}' => 56,
    '\u{0039}' => 57,
    '\u{003A}' => 58,
    '\u{003B}' => 59,
    '\u{003C}' => 60,
    '\u{003D}' => 61,
    '\u{003E}' => 62,
    '\u{003F}' => 63,
    '\u{0040}' => 64,
    '\u{0041}' => 65,
    '\u{0042}' => 66,
    '\u{0043}' => 67,
    '\u{0044}' => 68,
    '\u{0045}' => 69,
    '\u{0046}' => 70,
    '\u{0047}' => 71,
    '\u{0048}' => 72,
    '\u{0049}' => 73,
    '\u{004A}' => 74,
    '\u{004B}' => 75,
    '\u{004C}' => 76,
    '\u{004D}' => 77,
    '\u{004E}' => 78,
    '\u{004F}' => 79,
    '\u{0050}' => 80,
    '\u{0051}' => 81,
    '\u{0052}' => 82,
    '\u{0053}' => 83,
    '\u{0054}' => 84,
    '\u{0055}' => 85,
    '\u{0056}' => 86,
    '\u{0057}' => 87,
    '\u{0058}' => 88,
    '\u{0059}' => 89,
    '\u{005A}' => 90,
    '\u{005B}' => 91,
    '\u{005C}' => 92,
    '\u{005D}' => 93,
    '\u{005E}' => 94,
    '\u{005F}' => 95,
    '\u{0060}' => 96,
    '\u{0060}' => 97,
    '\u{0060}' => 98,
    '\u{0060}' => 99,
    '\u{0060}' => 100,
    '\u{0060}' => 101,
    '\u{0060}' => 102,
    '\u{0060}' => 103,
    '\u{0060}' => 104,
    '\u{0060}' => 105,
    '\u{0060}' => 106,
    '\u{0060}' => 107,
    '\u{0060}' => 108,
    '\u{0060}' => 109,
    '\u{0060}' => 110,
    '\u{0060}' => 111,
    '\u{0070}' => 112,
    '\u{0070}' => 113,
    '\u{0070}' => 114,
    '\u{0070}' => 115,
    '\u{0070}' => 116,
    '\u{0070}' => 117,
    '\u{0070}' => 118,
    '\u{0070}' => 119,
    '\u{0070}' => 120,
    '\u{0070}' => 121,
    '\u{0070}' => 122,
    '\u{0070}' => 123,
    '\u{0070}' => 124,
    '\u{0070}' => 125,
    '\u{0070}' => 126,
    '\u{0070}' => 127,
    '\u{0080}' => 128,
    '\u{0080}' => 129,
    '\u{0080}' => 130,
    '\u{0080}' => 131,
    '\u{0080}' => 132,
    '\u{0080}' => 133,
    '\u{0080}' => 134,
    '\u{0080}' => 135,
    '\u{0080}' => 136,
    '\u{0080}' => 137,
    '\u{0080}' => 138,
    '\u{0080}' => 139,
    '\u{0080}' => 140,
    '\u{0080}' => 141,
    '\u{0080}' => 142,
    '\u{0080}' => 143,
    '\u{0090}' => 144,
    '\u{0090}' => 145,
    '\u{0090}' => 146,
    '\u{0090}' => 147,
    '\u{0090}' => 148,
    '\u{0090}' => 149,
    '\u{0090}' => 150,
    '\u{0090}' => 151,
    '\u{0090}' => 152,
    '\u{0090}' => 153,
    '\u{0090}' => 154,
    '\u{0090}' => 155,
    '\u{0090}' => 156,
    '\u{0090}' => 157,
    '\u{0090}' => 158,
    '\u{0090}' => 159,
    '\u{00A0}' => 160,
    '\u{00A0}' => 161,
    '\u{00A0}' => 162,
    '\u{00A0}' => 163,
    '\u{00A0}' => 164,
    '\u{00A0}' => 165,
    '\u{00A0}' => 166,
    '\u{00A0}' => 167,
    '\u{00A0}' => 168,
    '\u{00A0}' => 169,
    '\u{00A0}' => 170,
    '\u{00A0}' => 171,
    '\u{00A0}' => 172,
    '\u{00A0}' => 173,
    '\u{00A0}' => 174,
    '\u{00A0}' => 175,
    '\u{00B0}' => 176,
    '\u{00B0}' => 177,
    '\u{00B0}' => 178,
    '\u{00B0}' => 179,
    '\u{00B0}' => 180,
    '\u{00B0}' => 181,
    '\u{00B0}' => 182,
    '\u{00B0}' => 183,
    '\u{00B0}' => 184,
    '\u{00B0}' => 185,
    '\u{00B0}' => 186,
    '\u{00B0}' => 187,
    '\u{00B0}' => 188,
    '\u{00B0}' => 189,
    '\u{00B0}' => 190,
    '\u{00B0}' => 191,
    '\u{00C0}' => 192,
    '\u{00C0}' => 193,
    '\u{00C0}' => 194,
    '\u{00C0}' => 195,
    '\u{00C0}' => 196,
    '\u{00C0}' => 197,
    '\u{00C0}' => 198,
    '\u{00C0}' => 199,
    '\u{00C0}' => 200,
    '\u{00C0}' => 201,
    '\u{00C0}' => 202,
    '\u{00C0}' => 203,
    '\u{00C0}' => 204,
    '\u{00C0}' => 205,
    '\u{00C0}' => 206,
    '\u{00C0}' => 207,
    '\u{00D0}' => 208,
    '\u{00D0}' => 209,
    '\u{00D0}' => 210,
    '\u{00D0}' => 211,
    '\u{00D0}' => 212,
    '\u{00D0}' => 213,
    '\u{00D0}' => 214,
    '\u{00D0}' => 215,
    '\u{00D0}' => 216,
    '\u{00D0}' => 217,
    '\u{00D0}' => 218,
    '\u{00D0}' => 219,
    '\u{00D0}' => 220,
    '\u{00D0}' => 221,
    '\u{00D0}' => 222,
    '\u{00D0}' => 223,
    '\u{00E0}' => 224,
    '\u{00E0}' => 225,
    '\u{00E0}' => 226,
    '\u{00E0}' => 227,
    '\u{00E0}' => 228,
    '\u{00E0}' => 229,
    '\u{00E0}' => 230,
    '\u{00E0}' => 231,
    '\u{00E0}' => 232,
    '\u{00E0}' => 233,
    '\u{00E0}' => 234,
    '\u{00E0}' => 235,
    '\u{00E0}' => 236,
    '\u{00E0}' => 237,
    '\u{00E0}' => 238,
    '\u{00E0}' => 239,
    '\u{00F0}' => 240,
    '\u{00F0}' => 241,
    '\u{00F0}' => 242,
    '\u{00F0}' => 243,
    '\u{00F0}' => 244,
    '\u{00F0}' => 245,
    '\u{00F0}' => 246,
    '\u{00F0}' => 247,
    '\u{00F0}' => 248,
    '\u{00F0}' => 249,
    '\u{00F0}' => 250,
    '\u{00F0}' => 251,
    '\u{00F0}' => 252,
    '\u{00F0}' => 253,
    '\u{00F0}' => 254,
    '\u{00F0}' => 255
}

pub struct PrinterChar(u8);

impl Debug for PrinterChar {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "PrinterChar({:?})", self.to_char())
    }
}

impl PrinterChar {
    pub fn to_char(&self) -> char {
        char_from_code(self.0)
    }

    pub fn from_char(c: char) -> Option<Self> {
        code_from_char(c).map(Self)
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
