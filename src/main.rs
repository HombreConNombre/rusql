mod models;
mod controllers;

use crate::models::data_model::ColumnType;
use crate::models::data_model::VariableData;
use crate::models::table_model::Table;

fn insert_row(table: &mut Table, row: Vec<VariableData>){
    let result = table.insert(row);

    match &result {
        Ok(_) => println!("Fila insertada correctamente"),
        Err(e) => println!("Error al insertar fila: {:?}", e),
    }
}
fn main() {
    let schema = vec![
        ColumnType::Integer,
        ColumnType::Text,
        ColumnType::Float,
    ];

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