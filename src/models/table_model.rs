use crate::models::data_model::{ColumnType, 
    DataError, 
    VariableData,
    Data
};
use crate::models::row_model::Row;

#[derive(Debug, Clone)]
pub struct Table{
    schema: Vec<ColumnType>,
    rows: Vec<Row>
}

impl Table{
    pub fn new(schema: Vec<ColumnType>) -> Self{
        Self { schema,
            rows: Vec::new() 
        }
    }

    pub fn insert(&mut self, values: Vec<VariableData>) -> Result<(), DataError> {
        if values.len() != self.schema.len() {
            return Err(DataError::InvalidRowLength{
                expected: self.schema.len(),
                found: values.len(),
            });
        }
        let mut row_values = Vec::new();

        for (col_type, value) in self.schema.iter().zip(values) {
            let data = Data::new(*col_type, value)?;
            row_values.push(data);
        }

        self.rows.push(Row { contents: row_values });
        Ok(())
    }
}
