use crate::Position;
use crate::Row;
use std::fs;
use std::io::{Error, Write};
#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub filename: Option<String>,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for value in contents.lines() {
            rows.push(Row::from(value));
        }
        Ok(Self {
            rows,
            filename: Some(filename.to_string()),
        })
    }
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
    pub fn len(&self) -> usize {
        self.rows.len()
    }
    pub fn insert(&mut self, at_pos: &Position, chr: char) {
        if chr == '\n' {
            self.insert_newline(at_pos);
            return;
        }
        if at_pos.y == self.len() {
            let mut row = Row::default();
            row.insert(0, chr);
            self.rows.push(row);
        } else if at_pos.y < self.len() {
            let row = self.rows.get_mut(at_pos.y).unwrap();
            row.insert(at_pos.x, chr);
        }
    }
    pub fn delete(&mut self, at_pos: &Position) {
        let len = self.len();
        if at_pos.y >= len {
            return;
        }
        if at_pos.x == self.rows.get_mut(at_pos.y).unwrap().len() && at_pos.y < len - 1 {
            let next_row = self.rows.remove(at_pos.y + 1);
            let row = self.rows.get_mut(at_pos.y).unwrap();
            row.append(&next_row);
        } else {
            let row = self.rows.get_mut(at_pos.y).unwrap();
            row.delete(at_pos.x);
        }
    }
    pub fn insert_newline(&mut self, at_pos: &Position) {
        if at_pos.y > self.len() {
            return;
        }
        if at_pos.y == self.len() {
            self.rows.push(Row::default());
            return;
        }
        let new_row = self.rows.get_mut(at_pos.y).unwrap().split(at_pos.x);
        self.rows.insert(at_pos.y + 1, new_row);
    }
    pub fn save(&self) -> Result<(), Error> {
        if let Some(filename) = &self.filename {
            let mut file = fs::File::create(filename)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
        }
        Ok(())
    }
}
