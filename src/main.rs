mod models;
mod controllers;

use crate::models::data_model::ColumnType;
use crate::models::data_model::VariableData;
use crate::models::schema_model::Schema;
use crate::models::table_model::Table;
use crate::models::column_model::Column;

use crate::controllers::row_controller::insert_row;

fn main() {
    let schema = Schema::new(vec![
        Column {
            name: "id".to_string(),
            col_type: ColumnType::Integer,
            nullable: false,
            is_primary_key: true,
        },
        Column {
            name: "name".to_string(),
            col_type: ColumnType::Text,
            nullable: false,
            is_primary_key: false,
        },
        Column {
            name: "age".to_string(),
            col_type: ColumnType::Float,
            nullable: true,
            is_primary_key: false,
        },
    ]);

    let mut table = Table::new(schema);

    let row1 = vec![
        VariableData::Integer(1),
        VariableData::Text("Alice".into()),
        VariableData::Float(23.5),
    ];

    let row2 = vec![
        VariableData::Text("ID".into()),
        VariableData::Text("Bob".into()),
        VariableData::Float(30.0),
    ];

    let row3 = vec![
        VariableData::Integer(2),
        VariableData::Text("Charlie".into()),
    ];

    for row in vec![row1, row2, row3]{
        insert_row(&mut table, row);
    }

    println!("\nTabla final:");
    println!("{:#?}", table);
}