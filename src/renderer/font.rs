pub struct Glyph {
    pub rows: [u8; 5], // lower 3 bits used
}

const FONT_0: Glyph = Glyph {
    rows: [0b111, 0b101, 0b101, 0b101, 0b111],
};
const FONT_1: Glyph = Glyph {
    rows: [0b010, 0b110, 0b010, 0b010, 0b111],
};
const FONT_2: Glyph = Glyph {
    rows: [0b111, 0b001, 0b111, 0b100, 0b111],
};
const FONT_3: Glyph = Glyph {
    rows: [0b111, 0b001, 0b011, 0b001, 0b111],
};
const FONT_4: Glyph = Glyph {
    rows: [0b101, 0b101, 0b111, 0b001, 0b001],
};
const FONT_5: Glyph = Glyph {
    rows: [0b111, 0b100, 0b111, 0b001, 0b111],
};
const FONT_6: Glyph = Glyph {
    rows: [0b111, 0b100, 0b111, 0b101, 0b111],
};
const FONT_7: Glyph = Glyph {
    rows: [0b111, 0b001, 0b010, 0b100, 0b100],
};
const FONT_8: Glyph = Glyph {
    rows: [0b111, 0b101, 0b111, 0b101, 0b111],
};
const FONT_9: Glyph = Glyph {
    rows: [0b111, 0b101, 0b111, 0b001, 0b001],
};

const FONT_A: Glyph = Glyph {
    rows: [0b010, 0b101, 0b111, 0b101, 0b101],
};
const FONT_B: Glyph = Glyph {
    rows: [0b110, 0b101, 0b110, 0b101, 0b110],
};
const FONT_C: Glyph = Glyph {
    rows: [0b111, 0b100, 0b100, 0b100, 0b111],
};
const FONT_D: Glyph = Glyph {
    rows: [0b110, 0b101, 0b101, 0b101, 0b110],
};
const FONT_E: Glyph = Glyph {
    rows: [0b111, 0b100, 0b110, 0b100, 0b111],
};
const FONT_F: Glyph = Glyph {
    rows: [0b111, 0b100, 0b110, 0b100, 0b100],
};
const FONT_G: Glyph = Glyph {
    rows: [0b111, 0b100, 0b101, 0b101, 0b111],
};
const FONT_H: Glyph = Glyph {
    rows: [0b101, 0b101, 0b111, 0b101, 0b101],
};
const FONT_I: Glyph = Glyph {
    rows: [0b111, 0b010, 0b010, 0b010, 0b111],
};
const FONT_J: Glyph = Glyph {
    rows: [0b001, 0b001, 0b001, 0b101, 0b111],
};
const FONT_K: Glyph = Glyph {
    rows: [0b101, 0b110, 0b100, 0b110, 0b101],
};
const FONT_L: Glyph = Glyph {
    rows: [0b100, 0b100, 0b100, 0b100, 0b111],
};
const FONT_M: Glyph = Glyph {
    rows: [0b101, 0b111, 0b101, 0b101, 0b101],
};
const FONT_N: Glyph = Glyph {
    rows: [0b110, 0b101, 0b101, 0b101, 0b101],
};
const FONT_O: Glyph = Glyph {
    rows: [0b111, 0b101, 0b101, 0b101, 0b111],
};
const FONT_P: Glyph = Glyph {
    rows: [0b111, 0b101, 0b111, 0b100, 0b100],
};
const FONT_Q: Glyph = Glyph {
    rows: [0b111, 0b101, 0b101, 0b111, 0b001],
};
const FONT_R: Glyph = Glyph {
    rows: [0b111, 0b101, 0b110, 0b101, 0b101],
};
const FONT_S: Glyph = Glyph {
    rows: [0b111, 0b100, 0b111, 0b001, 0b111],
};
const FONT_T: Glyph = Glyph {
    rows: [0b111, 0b010, 0b010, 0b010, 0b010],
};
const FONT_U: Glyph = Glyph {
    rows: [0b101, 0b101, 0b101, 0b101, 0b111],
};
const FONT_V: Glyph = Glyph {
    rows: [0b101, 0b101, 0b101, 0b101, 0b010],
};
const FONT_W: Glyph = Glyph {
    rows: [0b101, 0b101, 0b101, 0b111, 0b101],
};
const FONT_X: Glyph = Glyph {
    rows: [0b101, 0b101, 0b010, 0b101, 0b101],
};
const FONT_Y: Glyph = Glyph {
    rows: [0b101, 0b101, 0b010, 0b010, 0b010],
};
const FONT_Z: Glyph = Glyph {
    rows: [0b111, 0b001, 0b010, 0b100, 0b111],
};

const FONT_COLON: Glyph = Glyph {
    rows: [0b000, 0b010, 0b000, 0b010, 0b000],
};
const FONT_SLASH: Glyph = Glyph {
    rows: [0b001, 0b001, 0b010, 0b100, 0b100],
};
const FONT_SPACE: Glyph = Glyph {
    rows: [0b000, 0b000, 0b000, 0b000, 0b000],
};

pub fn get_glyph(c: char) -> Option<&'static Glyph> {
    match c {
        '0' => Some(&FONT_0),
        '1' => Some(&FONT_1),
        '2' => Some(&FONT_2),
        '3' => Some(&FONT_3),
        '4' => Some(&FONT_4),
        '5' => Some(&FONT_5),
        '6' => Some(&FONT_6),
        '7' => Some(&FONT_7),
        '8' => Some(&FONT_8),
        '9' => Some(&FONT_9),
        'A' => Some(&FONT_A),
        'B' => Some(&FONT_B),
        'C' => Some(&FONT_C),
        'D' => Some(&FONT_D),
        'E' => Some(&FONT_E),
        'F' => Some(&FONT_F),
        'G' => Some(&FONT_G),
        'H' => Some(&FONT_H),
        'I' => Some(&FONT_I),
        'J' => Some(&FONT_J),
        'K' => Some(&FONT_K),
        'L' => Some(&FONT_L),
        'M' => Some(&FONT_M),
        'N' => Some(&FONT_N),
        'O' => Some(&FONT_O),
        'P' => Some(&FONT_P),
        'Q' => Some(&FONT_Q),
        'R' => Some(&FONT_R),
        'S' => Some(&FONT_S),
        'T' => Some(&FONT_T),
        'U' => Some(&FONT_U),
        'V' => Some(&FONT_V),
        'W' => Some(&FONT_W),
        'X' => Some(&FONT_X),
        'Y' => Some(&FONT_Y),
        'Z' => Some(&FONT_Z),
        ':' => Some(&FONT_COLON),
        '/' => Some(&FONT_SLASH),
        ' ' => Some(&FONT_SPACE),
        _ => None,
    }
}
