use crate::{FileType, Pos, Row, SearchDirection};
use std::io::{Error, Write};
use std::{cmp::Ordering, fs};

#[derive(Default)]
pub struct Document {
    pub file_name: Option<String>,
    rows: Vec<Row>,
    dirty: bool,
    file_type: FileType,
}

impl Document {
    pub fn open(file_name: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(file_name)?;
        let mut rows = Vec::new();
        for value in contents.lines() {
            let mut row = Row::from(value);
            row.highlight(None);
            rows.push(row);
        }
        Ok(Self {
            rows,
            dirty: false,
            file_name: Some(file_name.to_string()),
            file_type: FileType::default(),
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
            return;
        }

        match at.y.cmp(&self.len()) {
            Ordering::Equal => {
                let mut row = Row::default();
                row.insert(0, c);
                row.highlight(None);
                self.rows.push(row);
            }
            Ordering::Less => {
                let row = &mut self.rows[at.y];
                row.insert(at.x, c);
                row.highlight(None);
            }
            _ => (),
        }
    }

    pub fn insert_newline(&mut self, at: &Pos) {
        if at.y > self.len() {
            return;
        }
        if at.y == self.len() {
            self.rows.push(Row::default());
        }
        let current_row = &mut self.rows[at.y];
        let mut new_row = current_row.split(at.x);
        current_row.highlight(None);
        new_row.highlight(None);
        self.rows.insert(at.y + 1, new_row);
    }

    pub fn find(&self, query: &str, at: &Pos, direction: SearchDirection) -> Option<Pos> {
        if at.y >= self.rows.len() {
            return None;
        }
        let mut pos = at.clone();

        let start = if direction == SearchDirection::Forward {
            at.y
        } else {
            0
        };
        let end = if direction == SearchDirection::Forward {
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
                if direction == SearchDirection::Forward {
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
            row.highlight(None);
        } else {
            let row = &mut self.rows[at.y];
            row.delete(at.x);
            row.highlight(None);
        }
    }

    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(filename) = &self.file_name {
            let mut file = fs::File::create(filename)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
            self.dirty = false;
        }
        Ok(())
    }

    pub fn highlight(&mut self, word: Option<&str>) {
        for row in &mut self.rows {
            row.highlight(word);
        }
    }
}
