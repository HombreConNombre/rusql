use crate::models::data_model::Data;

#[derive(Debug, Clone)]
pub struct Row{
    pub contents: Vec<Data>
}

impl Row {
    pub fn new(contents: Vec<Data>) -> Self {
        Self { contents }
    }
}