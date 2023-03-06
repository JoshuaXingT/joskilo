use crate::Position;
use crate::Row;
use std::fs;
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
        if at_pos.y == self.len() {
            return;
        }
        let row = self.rows.get_mut(at_pos.y).unwrap();
        row.delete(at_pos.x);
    }
}
