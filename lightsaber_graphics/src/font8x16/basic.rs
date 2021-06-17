use core::{
    fmt,
    slice
};

use crate::font8x16::{
    legacy::BASIC_LATIN,
    unicode::{
        UnicodeFont,
        UnicodeFonts,
    }
};

pub const BASIC_UNICODE: [UnicodeFont; 128] = [
    UnicodeFont('\u{0000}', BASIC_LATIN[0]),
    UnicodeFont('\u{0001}', BASIC_LATIN[1]),
    UnicodeFont('\u{0002}', BASIC_LATIN[2]),
    UnicodeFont('\u{0003}', BASIC_LATIN[3]),
    UnicodeFont('\u{0004}', BASIC_LATIN[4]),
    UnicodeFont('\u{0005}', BASIC_LATIN[5]),
    UnicodeFont('\u{0006}', BASIC_LATIN[6]),
    UnicodeFont('\u{0007}', BASIC_LATIN[7]),
    UnicodeFont('\u{0008}', BASIC_LATIN[8]),
    UnicodeFont('\u{0009}', BASIC_LATIN[9]),
    UnicodeFont('\u{000A}', BASIC_LATIN[10]),
    UnicodeFont('\u{000B}', BASIC_LATIN[11]),
    UnicodeFont('\u{000C}', BASIC_LATIN[12]),
    UnicodeFont('\u{000D}', BASIC_LATIN[13]),
    UnicodeFont('\u{000E}', BASIC_LATIN[14]),
    UnicodeFont('\u{000F}', BASIC_LATIN[15]),
    UnicodeFont('\u{0010}', BASIC_LATIN[16]),
    UnicodeFont('\u{0011}', BASIC_LATIN[17]),
    UnicodeFont('\u{0012}', BASIC_LATIN[18]),
    UnicodeFont('\u{0013}', BASIC_LATIN[19]),
    UnicodeFont('\u{0014}', BASIC_LATIN[20]),
    UnicodeFont('\u{0015}', BASIC_LATIN[21]),
    UnicodeFont('\u{0016}', BASIC_LATIN[22]),
    UnicodeFont('\u{0017}', BASIC_LATIN[23]),
    UnicodeFont('\u{0018}', BASIC_LATIN[24]),
    UnicodeFont('\u{0019}', BASIC_LATIN[25]),
    UnicodeFont('\u{001A}', BASIC_LATIN[26]),
    UnicodeFont('\u{001B}', BASIC_LATIN[27]),
    UnicodeFont('\u{001C}', BASIC_LATIN[28]),
    UnicodeFont('\u{001D}', BASIC_LATIN[29]),
    UnicodeFont('\u{001E}', BASIC_LATIN[30]),
    UnicodeFont('\u{001F}', BASIC_LATIN[31]),
    UnicodeFont('\u{0020}', BASIC_LATIN[32]),
    UnicodeFont('\u{0021}', BASIC_LATIN[33]),
    UnicodeFont('\u{0022}', BASIC_LATIN[34]),
    UnicodeFont('\u{0023}', BASIC_LATIN[35]),
    UnicodeFont('\u{0024}', BASIC_LATIN[36]),
    UnicodeFont('\u{0025}', BASIC_LATIN[37]),
    UnicodeFont('\u{0026}', BASIC_LATIN[38]),
    UnicodeFont('\u{0027}', BASIC_LATIN[39]),
    UnicodeFont('\u{0028}', BASIC_LATIN[40]),
    UnicodeFont('\u{0029}', BASIC_LATIN[41]),
    UnicodeFont('\u{002A}', BASIC_LATIN[42]),
    UnicodeFont('\u{002B}', BASIC_LATIN[43]),
    UnicodeFont('\u{002C}', BASIC_LATIN[44]),
    UnicodeFont('\u{002D}', BASIC_LATIN[45]),
    UnicodeFont('\u{002E}', BASIC_LATIN[46]),
    UnicodeFont('\u{002F}', BASIC_LATIN[47]),
    UnicodeFont('\u{0030}', BASIC_LATIN[48]),
    UnicodeFont('\u{0031}', BASIC_LATIN[49]),
    UnicodeFont('\u{0032}', BASIC_LATIN[50]),
    UnicodeFont('\u{0033}', BASIC_LATIN[51]),
    UnicodeFont('\u{0034}', BASIC_LATIN[52]),
    UnicodeFont('\u{0035}', BASIC_LATIN[53]),
    UnicodeFont('\u{0036}', BASIC_LATIN[54]),
    UnicodeFont('\u{0037}', BASIC_LATIN[55]),
    UnicodeFont('\u{0038}', BASIC_LATIN[56]),
    UnicodeFont('\u{0039}', BASIC_LATIN[57]),
    UnicodeFont('\u{003A}', BASIC_LATIN[58]),
    UnicodeFont('\u{003B}', BASIC_LATIN[59]),
    UnicodeFont('\u{003C}', BASIC_LATIN[60]),
    UnicodeFont('\u{003D}', BASIC_LATIN[61]),
    UnicodeFont('\u{003E}', BASIC_LATIN[62]),
    UnicodeFont('\u{003F}', BASIC_LATIN[63]),
    UnicodeFont('\u{0040}', BASIC_LATIN[64]),
    UnicodeFont('\u{0041}', BASIC_LATIN[65]),
    UnicodeFont('\u{0042}', BASIC_LATIN[66]),
    UnicodeFont('\u{0043}', BASIC_LATIN[67]),
    UnicodeFont('\u{0044}', BASIC_LATIN[68]),
    UnicodeFont('\u{0045}', BASIC_LATIN[69]),
    UnicodeFont('\u{0046}', BASIC_LATIN[70]),
    UnicodeFont('\u{0047}', BASIC_LATIN[71]),
    UnicodeFont('\u{0048}', BASIC_LATIN[72]),
    UnicodeFont('\u{0049}', BASIC_LATIN[73]),
    UnicodeFont('\u{004A}', BASIC_LATIN[74]),
    UnicodeFont('\u{004B}', BASIC_LATIN[75]),
    UnicodeFont('\u{004C}', BASIC_LATIN[76]),
    UnicodeFont('\u{004D}', BASIC_LATIN[77]),
    UnicodeFont('\u{004E}', BASIC_LATIN[78]),
    UnicodeFont('\u{004F}', BASIC_LATIN[79]),
    UnicodeFont('\u{0050}', BASIC_LATIN[80]),
    UnicodeFont('\u{0051}', BASIC_LATIN[81]),
    UnicodeFont('\u{0052}', BASIC_LATIN[82]),
    UnicodeFont('\u{0053}', BASIC_LATIN[83]),
    UnicodeFont('\u{0054}', BASIC_LATIN[84]),
    UnicodeFont('\u{0055}', BASIC_LATIN[85]),
    UnicodeFont('\u{0056}', BASIC_LATIN[86]),
    UnicodeFont('\u{0057}', BASIC_LATIN[87]),
    UnicodeFont('\u{0058}', BASIC_LATIN[88]),
    UnicodeFont('\u{0059}', BASIC_LATIN[89]),
    UnicodeFont('\u{005A}', BASIC_LATIN[90]),
    UnicodeFont('\u{005B}', BASIC_LATIN[91]),
    UnicodeFont('\u{005C}', BASIC_LATIN[92]),
    UnicodeFont('\u{005D}', BASIC_LATIN[93]),
    UnicodeFont('\u{005E}', BASIC_LATIN[94]),
    UnicodeFont('\u{005F}', BASIC_LATIN[95]),
    UnicodeFont('\u{0060}', BASIC_LATIN[96]),
    UnicodeFont('\u{0061}', BASIC_LATIN[97]),
    UnicodeFont('\u{0062}', BASIC_LATIN[98]),
    UnicodeFont('\u{0063}', BASIC_LATIN[99]),
    UnicodeFont('\u{0064}', BASIC_LATIN[100]),
    UnicodeFont('\u{0065}', BASIC_LATIN[101]),
    UnicodeFont('\u{0066}', BASIC_LATIN[102]),
    UnicodeFont('\u{0067}', BASIC_LATIN[103]),
    UnicodeFont('\u{0068}', BASIC_LATIN[104]),
    UnicodeFont('\u{0069}', BASIC_LATIN[105]),
    UnicodeFont('\u{006A}', BASIC_LATIN[106]),
    UnicodeFont('\u{006B}', BASIC_LATIN[107]),
    UnicodeFont('\u{006C}', BASIC_LATIN[108]),
    UnicodeFont('\u{006D}', BASIC_LATIN[109]),
    UnicodeFont('\u{006E}', BASIC_LATIN[110]),
    UnicodeFont('\u{006F}', BASIC_LATIN[111]),
    UnicodeFont('\u{0070}', BASIC_LATIN[112]),
    UnicodeFont('\u{0071}', BASIC_LATIN[113]),
    UnicodeFont('\u{0072}', BASIC_LATIN[114]),
    UnicodeFont('\u{0073}', BASIC_LATIN[115]),
    UnicodeFont('\u{0074}', BASIC_LATIN[116]),
    UnicodeFont('\u{0075}', BASIC_LATIN[117]),
    UnicodeFont('\u{0076}', BASIC_LATIN[118]),
    UnicodeFont('\u{0077}', BASIC_LATIN[119]),
    UnicodeFont('\u{0078}', BASIC_LATIN[120]),
    UnicodeFont('\u{0079}', BASIC_LATIN[121]),
    UnicodeFont('\u{007A}', BASIC_LATIN[122]),
    UnicodeFont('\u{007B}', BASIC_LATIN[123]),
    UnicodeFont('\u{007C}', BASIC_LATIN[124]),
    UnicodeFont('\u{007D}', BASIC_LATIN[125]),
    UnicodeFont('\u{007E}', BASIC_LATIN[126]),
    UnicodeFont('\u{007F}', BASIC_LATIN[127])
];

pub const BASIC_FONTS: BasicUnicodeFonts = BasicUnicodeFonts(BASIC_UNICODE);

pub struct BasicUnicodeFonts([UnicodeFont; 128]);

impl BasicUnicodeFonts {
    pub fn new() -> Self {
        BasicUnicodeFonts(BASIC_UNICODE)
    }
}

impl fmt::Debug for BasicUnicodeFonts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, stringify!(BASIC_UNICODE))
    }
}

impl Default for BasicUnicodeFonts {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for BasicUnicodeFonts {
    fn eq(&self, other: &Self) -> bool {
        self.0
            .iter()
            .zip(other.0.iter())
            .fold(true, |eq, (a, b)| eq && a == b)
    }
}

impl UnicodeFonts for BasicUnicodeFonts {
    fn get(&self, key: char) -> Option<[u8; 16]> {
        match self.get_font(key) {
            Some(font) => Some(font.into()),
            None => None
        }
    }

    fn get_font(&self, key: char) -> Option<UnicodeFont> {
        match self.0.binary_search_by_key(&key, |&f| f.char()) {
            Ok(index) => Some(self.0[index]),
            _ => None
        }
    }

    fn iter(&self) -> slice::Iter<UnicodeFont> {
        self.0.iter()
    }
}
