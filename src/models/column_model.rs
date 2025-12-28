use crate::models::data_model::ColumnType;

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub col_type: ColumnType,
    pub nullable: bool,
    pub is_primary_key: bool,
}
