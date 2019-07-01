use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct WDBSchema {
    columns: Vec<WDBType>,
}

impl WDBSchema {
    pub fn new(columns: Vec<WDBType>) -> WDBSchema {
        WDBSchema {
            columns,
        }
    }

    pub fn columns(&self) -> &[WDBType] {
        &self.columns
    }

    pub fn validate(&self, row: &WDBRow) -> Result<(), WDBError> {
        Ok(())
    }
}
