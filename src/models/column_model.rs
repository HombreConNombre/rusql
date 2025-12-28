use crate::models::data_model::ColumnType;

#[derive(Debug)]
pub enum ColumnError {
    PrimaryKeyCannotBeNullable,
}

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub col_type: ColumnType,
    pub nullable: bool,
    pub is_primary_key: bool,
}

impl Column {
    pub fn new(name:String, col_type: ColumnType, 
        nullable: bool, is_primary_key: bool) 
    -> Result<Self, ColumnError>{
        if nullable && is_primary_key{
            return Err(ColumnError::PrimaryKeyCannotBeNullable);
        }
        Ok( Self { name,
            col_type,
            nullable,
            is_primary_key
        })
    }
}
