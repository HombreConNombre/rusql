use crate::models::column_model::Column;

#[derive(Debug)]
pub enum SchemaError {
    MultiplePrimaryKeys,
    NoPrimaryKey, 
}

#[derive(Debug, Clone)]
pub struct Schema {
    pub columns: Vec<Column>,
}

impl Schema {
    pub fn new(columns: Vec<Column>) -> Result<Self, SchemaError> {
        let pk_columns_count: i8 = Self::count_pks(&columns);

        match pk_columns_count{
            0 => Err( SchemaError::NoPrimaryKey),
            1 => Ok( Self{ columns}),
            _ => Err( SchemaError::MultiplePrimaryKeys)
        }
    }

    fn count_pks(columns: &Vec<Column>) -> i8 {
        let mut pk_columns_count = 0;

        for column in columns {
            if column.is_primary_key {
                pk_columns_count += 1;
            }
        }
        pk_columns_count
    }

    pub fn len(&self) -> usize {
        self.columns.len()
    }
}
