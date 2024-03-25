use crate::{highlight, Direction};
use std::cmp;
use termion::color;
use unicode_segmentation::UnicodeSegmentation;

type HlOpts = highlight::Options;
type HlType = highlight::Type;

#[derive(Default)]
pub struct Row {
    highlighted: bool,
    string: String,
    highlighting: Vec<HlType>,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: String::from(slice),
            highlighting: Vec::new(),
            highlighted: false,
            len: slice.graphemes(true).count(),
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        const TAB_SPACE: usize = 4;
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        let mut result = String::new();
        let mut current_highlighting = &HlType::None;
        for (index, grapheme) in self.string[..]
            .graphemes(true)
            .enumerate()
            .skip(start)
            .take(end - start)
        {
            if let Some(c) = grapheme.chars().next() {
                let hl_type = self.highlighting.get(index).unwrap_or(&HlType::None);

                if hl_type != current_highlighting {
                    current_highlighting = hl_type;
                    let start_highlight = format!("{}", color::Fg(hl_type.to_color()));
                    result.push_str(&start_highlight[..]);
                }

                if c == '\t' {
                    result.push_str(" ".repeat(TAB_SPACE).as_str());
                } else {
                    result.push(c);
                }
            }
        }
        let end_highlight = format!("{}", color::Fg(color::Reset));
        result.push_str(&end_highlight[..]);
        result
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.string.push(c);
            self.len += 1;
            return;
        }
        let mut result: String = String::new();
        let mut length = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            length += 1;
            if index == at {
                length += 1;
                result.push(c);
            }
            result.push_str(grapheme);
        }
        self.len = length;
        self.highlighted = false;
        self.string = result;
    }

    pub fn find(&self, query: &str, at: usize, direction: Direction) -> Option<usize> {
        if at > self.len || query.is_empty() {
            return None;
        }
        let start = if direction == Direction::Forward {
            at
        } else {
            0
        };
        let end = if direction == Direction::Forward {
            self.len
        } else {
            at
        };

        let substring: String = self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .collect();
        let matching_byte_index = if direction == Direction::Forward {
            substring.find(query)
        } else {
            substring.rfind(query)
        };

        if let Some(matching_byte_index) = matching_byte_index {
            return substring[..]
                .grapheme_indices(true)
                .position(|(byte_index, _)| byte_index == matching_byte_index)
                .map(|index| index + start);
        }
        None
    }

    pub fn delete(&mut self, at: usize) {
        if at >= self.len() {
            return;
        }

        let mut result: String = String::new();
        let mut length = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            if index != at {
                length += 1;
                result.push_str(grapheme);
            }
        }
        self.len = length;
        self.string = result;
    }

    pub fn append(&mut self, new: &Self) {
        self.string = format!("{}{}", self.string, new.string);
        self.len += new.len;
    }

    pub fn split(&mut self, at: usize) -> Self {
        let mut row: String = String::new();
        let mut length = 0;
        let mut splitted_row: String = String::new();
        let mut splitted_length = 0;
        for (index, grapheme) in self.string[..].graphemes(true).enumerate() {
            if index < at {
                length += 1;
                row.push_str(grapheme);
            } else {
                splitted_length += 1;
                splitted_row.push_str(grapheme);
            }
        }
        self.string = row;
        self.len = length;
        Self {
            string: splitted_row,
            highlighting: Vec::new(),
            len: splitted_length,
            highlighted: false,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }

    pub fn unhighlight(&mut self) {
        self.highlighted = false;
    }

    pub fn highlight(
        &mut self,
        opts: &HlOpts,
        word: &Option<String>,
        start_with_comment: bool,
    ) -> bool {
        let chars: Vec<char> = self.string.chars().collect();
        if self.highlighted && word.is_none() {
            if let Some(hl_type) = self.highlighting.last() {
                if *hl_type == HlType::MultilineComment
                    && self.string.len() > 1
                    && self.string[self.string.len() - 2..] == *"*/"
                {
                    return true;
                }
            }
            return false;
        }
        self.highlighting = Vec::new();
        let mut index = 0;
        let mut in_ml_comment = start_with_comment;
        if in_ml_comment {
            let closing_index = if let Some(closing_index) = self.string.find("*/") {
                closing_index + 2
            } else {
                chars.len()
            };
            for _ in 0..closing_index {
                self.highlighting.push(HlType::MultilineComment);
            }
            index = closing_index;
        }
        while let Some(c) = chars.get(index) {
            if opts.multiline_comments() && self.highlight_multiline_comment(&mut index, *c, &chars)
            {
                in_ml_comment = true;
                continue;
            }
            in_ml_comment = false;
            if (opts.chars() && self.highlight_char(&mut index, *c, &chars))
                || (opts.comments() && self.highlight_comment(&mut index, *c, &chars))
                || (self.highlight_primary_keywords(&mut index, opts, &chars))
                || (self.highlight_secondary_keywords(&mut index, opts, &chars))
                || (opts.strings() && self.highlight_string(&mut index, *c, &chars))
                || (opts.numbers() && self.highlight_number(&mut index, *c, &chars))
            {
                continue;
            }
            self.highlighting.push(HlType::None);
            index += 1;
        }
        self.highlight_match(word);
        if in_ml_comment && &self.string[self.string.len().saturating_sub(2)..] != "*/" {
            return true;
        }
        self.highlighted = true;
        false
    }

    fn highlight_match(&mut self, word: &Option<String>) {
        let s = "".to_string();
        let word = word.as_ref().unwrap_or(&s);
        if word.is_empty() {
            return;
        }
        let mut index = 0;
        while let Some(search_match) = self.find(word, index, Direction::Forward) {
            if let Some(next_index) = search_match.checked_add(word[..].graphemes(true).count()) {
                for i in index.saturating_add(search_match)..next_index {
                    self.highlighting[i] = HlType::Match;
                }
                index = next_index;
            } else {
                break;
            }
        }
    }

    fn highlight_char(&mut self, index: &mut usize, c: char, chars: &[char]) -> bool {
        // check opening char and len
        if c != '\'' || chars.len() <= *index + 1 {
            return false;
        }

        // escape char
        let next_char = chars[*index + 1];
        let closing_index = if next_char == '\\' {
            *index + 3
        } else {
            *index + 2
        };

        // check closing char and len
        if chars.len() <= closing_index || chars[closing_index] != '\'' {
            return false;
        }

        let len = closing_index.saturating_sub(*index) + 1;
        self.highlighting
            .extend(std::iter::repeat(HlType::Character).take(len));
        *index += len;
        true
    }

    fn highlight_comment(&mut self, index: &mut usize, c: char, chars: &[char]) -> bool {
        // check opening char and len
        if c != '/' || *index + 1 >= chars.len() || chars[*index + 1] != '/' {
            return false;
        }

        let len = chars.len().saturating_sub(*index);
        self.highlighting
            .extend(std::iter::repeat(HlType::Comment).take(len));
        *index += len;
        true
    }

    fn highlight_multiline_comment(&mut self, index: &mut usize, c: char, chars: &[char]) -> bool {
        // check opening char and len
        if c != '/' || *index + 1 >= chars.len() || chars[*index + 1] != '*' {
            return false;
        }

        let closing_index = self.string[*index + 2..]
            .find("*/")
            .map(|i| i + *index + 4)
            .unwrap_or(chars.len());

        let len = closing_index.saturating_sub(*index);
        self.highlighting
            .extend(std::iter::repeat(HlType::MultilineComment).take(len));
        *index += len;
        true
    }

    fn highlight_string(&mut self, index: &mut usize, c: char, chars: &[char]) -> bool {
        if c != '"' {
            return false;
        }
        loop {
            self.highlighting.push(HlType::String);
            *index += 1;
            if let Some(next_char) = chars.get(*index) {
                if *next_char == '"' {
                    break;
                }
            } else {
                break;
            }
        }
        self.highlighting.push(HlType::String);
        *index += 1;
        true
    }

    fn highlight_number(&mut self, index: &mut usize, c: char, chars: &[char]) -> bool {
        if !c.is_ascii_digit() {
            return false;
        }
        if *index > 0 && !is_seperator(chars[*index - 1]) {
            return false;
        }

        loop {
            self.highlighting.push(HlType::Number);
            *index += 1;
            if let Some(next_char) = chars.get(*index) {
                if *next_char != '.' && !next_char.is_ascii_digit() {
                    break;
                }
            } else {
                break;
            }
        }
        true
    }

    fn highlight_str(
        &mut self,
        index: &mut usize,
        substring: &str,
        chars: &[char],
        hl_type: HlType,
    ) -> bool {
        if substring.is_empty() || substring.len() + *index > chars.len() {
            return false;
        }

        let chars_iter = chars.iter().skip(*index);
        let substring_chars_iter = substring.chars();

        let matches = substring_chars_iter.zip(chars_iter).all(|(a, b)| a == *b);
        if matches {
            self.highlighting
                .extend(std::iter::repeat(hl_type).take(substring.len()));
            *index += substring.len();
        }

        matches
    }

    fn highlight_keywords(
        &mut self,
        index: &mut usize,
        chars: &[char],
        keywords: &[String],
        hl_type: HlType,
    ) -> bool {
        if *index > 0 {
            let prev_chars = chars[*index - 1];
            if !is_seperator(prev_chars) {
                return false;
            }
        }

        for word in keywords {
            if *index + word.len() < chars.len() {
                let next_char = chars[*index + word.len()];
                if !is_seperator(next_char) {
                    continue;
                }
            }
            if self.highlight_str(index, word, chars, hl_type) {
                return true;
            }
        }
        false
    }

    fn highlight_primary_keywords(
        &mut self,
        index: &mut usize,
        opts: &HlOpts,
        chars: &[char],
    ) -> bool {
        self.highlight_keywords(
            index,
            chars,
            opts.primary_keywords(),
            HlType::PrimaryKeywords,
        )
    }

    fn highlight_secondary_keywords(
        &mut self,
        index: &mut usize,
        opts: &HlOpts,
        chars: &[char],
    ) -> bool {
        self.highlight_keywords(
            index,
            chars,
            opts.secondary_keywords(),
            HlType::SecondaryKeywords,
        )
    }
}

fn is_seperator(c: char) -> bool {
    c.is_ascii_punctuation() || c.is_ascii_whitespace()
}
