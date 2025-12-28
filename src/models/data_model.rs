
#[derive(Debug, Clone, Copy)]
pub enum ColumnType {
    Integer,
    Text,
    Float,
}
#[derive(Debug, Clone)]
pub enum VariableData{
    Integer(i32),
    Text(String),
    Float(f32),
    Boolean(bool)
}

#[derive(Debug, Clone)]
pub struct Data{
    data_type: ColumnType,
    content: VariableData,
}

#[derive(Debug)]
pub enum DataError {
    TypeMismatch {
        expected: ColumnType,
        found: VariableData,
    },
    InvalidRowLength {
        expected: usize,
        found: usize,
    },
}

impl Data{
    pub fn new(data_type: ColumnType, content: VariableData) -> Result<Self, DataError> {
        let valid = match (&data_type, &content) {
            (ColumnType::Integer, VariableData::Integer(_)) => true,
            (ColumnType::Text,    VariableData::Text(_))    => true,
            (ColumnType::Float,   VariableData::Float(_))   => true,
            _ => false,
        };

        if valid {
            Ok(Self { data_type, content })
        } else {
            Err(DataError::TypeMismatch {
                expected: data_type,
                found: content,
            })
        }
    }
}
