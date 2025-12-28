use crate::models::{
    data_model::{Data, DataError, VariableData},
    row_model::Row,
    schema_model::Schema,
};

#[derive(Debug)]
pub struct Table {
    schema: Schema,
    rows: Vec<Row>,
}

impl Table {
    pub fn new(schema: Schema) -> Self {
        Self {
            schema,
            rows: Vec::new(),
        }
    }

    pub fn insert(&mut self, values: Vec<VariableData>) -> Result<(), DataError> {
        if values.len() != self.schema.len() {
            return Err(DataError::InvalidRowLength {
                expected: self.schema.len(),
                found: values.len(),
            });
        }

        let mut row_values = Vec::new();

        for (column, value) in self.schema.columns.iter().zip(values) {
            let data = Data::new(column.col_type, value)?;
            row_values.push(data);
        }

        self.rows.push(Row::new(row_values));
        Ok(())
    }
}
