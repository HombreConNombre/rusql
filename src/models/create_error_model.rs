use crate::models::data_model::DataError;
use crate::models::column_model::ColumnError;
use crate::models::schema_model::SchemaError;

#[derive(Debug)]
pub enum CreateErrors{
    Data(DataError),
    Column(ColumnError),
    Schema(SchemaError)
}
impl From<SchemaError> for CreateErrors {
    fn from(err: SchemaError) -> Self {
        CreateErrors::Schema(err)
    }
}

impl From<DataError> for CreateErrors {
    fn from(err: DataError) -> Self {
        CreateErrors::Data(err)
    }
}

impl From<ColumnError> for CreateErrors{
    fn from(err: ColumnError) -> Self{
        CreateErrors::Column(err)
    }
}