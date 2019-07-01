use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum WDBError {
    BadCSV,
    BadIO,
}
