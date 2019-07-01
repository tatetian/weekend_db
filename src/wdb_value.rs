use super::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum WDBValue {
    Integer(u64),
    Text(String),
}

impl From<u64> for WDBValue {
    fn from(val: u64) -> WDBValue {
        WDBValue::Integer(val)
    }
}

impl From<&str> for WDBValue {
    fn from(val: &str) -> WDBValue {
        WDBValue::Text(val.to_owned())
    }
}

impl fmt::Display for WDBValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WDBValue::Integer(i) => write!(f, "{}", i),
            WDBValue::Text(s) => write!(f, "{}", s),
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum WDBType {
    Integer,
    Text,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_to_wdb_value() {
        let wdb_text : WDBValue = "abc".into();
        assert_eq!(wdb_text, WDBValue::Text("abc".to_owned()));

        let wdb_int : WDBValue = 123.into();
        assert_eq!(wdb_int, WDBValue::Integer(123));
    }
}
