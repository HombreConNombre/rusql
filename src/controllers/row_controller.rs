use crate::models::data_model::VariableData;
use crate::models::table_model::Table;

pub fn insert_bulk(table: &mut Table, row: Vec<VariableData>){
    let result = table.insert(row);

    match &result {
        Ok(_) => println!("Fila insertada correctamente"),
        Err(e) => println!("Error al insertar fila: {:?}", e),
    }
}