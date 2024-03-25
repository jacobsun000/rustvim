use crate::{Direction, FileType, Pos, Row};
use std::fs;
use std::io::{Error, Write};

#[derive(Default)]
pub struct Document {
    pub file_name: Option<String>,
    file_type: FileType,
    rows: Vec<Row>,
    dirty: bool,
}

impl Document {
    pub fn open(file_name: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(file_name)?;
        Ok(Self {
            rows: contents.lines().map(Row::from).collect(),
            dirty: false,
            file_name: Some(file_name.to_string()),
            file_type: FileType::from(file_name),
        })
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn file_type(&self) -> String {
        self.file_type.name()
    }

    pub fn insert(&mut self, at: &Pos, c: char) {
        if at.y > self.len() {
            return;
        }
        self.dirty = true;
        if c == '\n' {
            self.insert_newline(at);
        } else if at.y == self.rows.len() {
            self.rows.push(Row::from(c.to_string().as_str()));
        } else {
            self.rows[at.y].insert(at.x, c);
        }
        self.unhighlight_rows(at.y);
    }

    pub fn insert_newline(&mut self, at: &Pos) {
        if at.y > self.len() {
            return;
        }
        if at.y == self.len() {
            self.rows.push(Row::default());
            return;
        }
        let current_row = &mut self.rows[at.y];
        let new_row = current_row.split(at.x);
        self.rows.insert(at.y + 1, new_row);
    }

    pub fn find(&self, query: &str, at: &Pos, direction: Direction) -> Option<Pos> {
        if at.y >= self.rows.len() {
            return None;
        }
        let mut pos = *at;

        let start = if direction == Direction::Forward {
            at.y
        } else {
            0
        };
        let end = if direction == Direction::Forward {
            self.rows.len()
        } else {
            at.y + 1
        };

        for _ in start..end {
            if let Some(row) = self.rows.get(pos.y) {
                if let Some(x) = row.find(query, pos.x, direction) {
                    pos.x = x;
                    return Some(pos);
                }
                if direction == Direction::Forward {
                    pos.y += 1;
                    pos.x = 0;
                } else {
                    pos.y -= 1;
                    pos.x = self.rows[pos.y].len();
                }
            } else {
                return None;
            }
        }
        None
    }

    pub fn delete(&mut self, at: &Pos) {
        let len = self.len();
        if at.y >= len {
            return;
        }
        self.dirty = true;
        if at.x == self.rows[at.y].len() && at.y < len - 1 {
            let next_row = self.rows.remove(at.y + 1);
            let row = &mut self.rows[at.y];
            row.append(&next_row);
        } else {
            let row = &mut self.rows[at.y];
            row.delete(at.x);
        }
        self.unhighlight_rows(at.y);
    }

    pub fn save(&mut self) -> Result<(), Error> {
        if self.file_name.is_none() {
            return Err(Error::new(std::io::ErrorKind::InvalidInput, "No file name"));
        }

        let file_name = self.file_name.as_ref().unwrap();
        let mut file = fs::File::create(file_name)?;
        self.file_type = FileType::from(file_name);
        for row in &mut self.rows {
            file.write_all(row.as_bytes())?;
            file.write_all(b"\n")?;
        }
        self.dirty = false;
        Ok(())
    }

    pub fn highlight(&mut self, word: &Option<String>, until: Option<usize>) {
        let mut start_with_comment = false;
        let until = until
            .map(|u| u + 1)
            .unwrap_or(self.rows.len())
            .min(self.rows.len());

        for row in &mut self.rows[..until] {
            start_with_comment =
                row.highlight(self.file_type.highlight_options(), word, start_with_comment);
        }
    }

    fn unhighlight_rows(&mut self, start: usize) {
        let start = start.saturating_sub(1);
        for row in self.rows.iter_mut().skip(start) {
            row.unhighlight();
        }
    }
}
