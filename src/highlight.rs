use termion::color;

#[derive(PartialEq, Clone, Copy)]
pub enum Type {
    None,
    Number,
    Match,
    String,
    Character,
    Comment,
    MultilineComment,
    PrimaryKeywords,
    SecondaryKeywords,
}

#[derive(Default)]
pub struct Options {
    numbers: bool,
    strings: bool,
    chars: bool,
    comments: bool,
    multiline_comments: bool,
    primary_keywords: Vec<String>,
    secondary_keywords: Vec<String>,
}

impl Options {
    pub fn numbers(&self) -> bool {
        self.numbers
    }

    pub fn strings(&self) -> bool {
        self.strings
    }

    pub fn chars(&self) -> bool {
        self.chars
    }

    pub fn comments(&self) -> bool {
        self.comments
    }

    pub fn multiline_comments(&self) -> bool {
        self.multiline_comments
    }

    pub fn primary_keywords(&self) -> &Vec<String> {
        &self.primary_keywords
    }

    pub fn secondary_keywords(&self) -> &Vec<String> {
        &self.secondary_keywords
    }
}

impl Type {
    pub fn to_color(self) -> impl color::Color {
        match self {
            Type::Number => color::Rgb(220, 163, 163),
            Type::Match => color::Rgb(38, 139, 210),
            Type::String => color::Rgb(211, 54, 130),
            Type::Character => color::Rgb(108, 113, 196),
            Type::MultilineComment | Type::Comment => color::Rgb(133, 153, 0),
            Type::PrimaryKeywords => color::Rgb(181, 137, 0),
            Type::SecondaryKeywords => color::Rgb(42, 161, 152),
            _ => color::Rgb(255, 255, 255),
        }
    }
}

// Highlight Options for file types
impl From<&str> for Options {
    fn from(s: &str) -> Self {
        match s {
            "rust" => Self {
                numbers: true,
                strings: true,
                chars: true,
                comments: true,
                multiline_comments: true,
                primary_keywords: stringify(vec![
                    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false",
                    "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut",
                    "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait",
                    "true", "type", "unsafe", "use", "where", "while", "dyn", "abstract", "become",
                    "box", "do", "final", "macro", "override", "priv", "typeof", "unsized",
                    "virtual", "yield", "async", "await", "try",
                ]),
                secondary_keywords: stringify(vec![
                    "bool", "char", "i8", "i16", "i32", "i64", "isize", "u8", "u16", "u32", "u64",
                    "usize", "f32", "f64",
                ]),
            },
            _ => Self::default(),
        }
    }
}

fn stringify(iterator: Vec<&str>) -> Vec<String> {
    iterator.iter().map(|s| s.to_string()).collect()
}
