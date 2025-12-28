mod models;
mod controllers;

use crate::models::data_model::ColumnType;
use crate::models::data_model::VariableData;
use crate::models::schema_model::Schema;
use crate::models::table_model::Table;
use crate::models::column_model::Column;

use crate::models::create_error_model::CreateErrors;

fn main() -> Result<(), CreateErrors>{
    let schema = Schema::new(vec![
        Column::new("id".to_string(), ColumnType::Integer, false, true)?,
        Column::new("name".to_string(), ColumnType::Text, false, false)?,
        Column::new("age".to_string(), ColumnType::Float, true, false)?,
    ])?;

    let mut table = Table::new(schema);
    
    table.insert(vec![
        VariableData::Integer(1),
        VariableData::Text("Alice".into()),
        VariableData::Float(23.5),
    ])?;
    
    println!("\nTabla final:");
    println!("{:#?}", table);
    Ok(())
}