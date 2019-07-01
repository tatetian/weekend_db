use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct WDBRow {
    fields: Vec<WDBValue>,
}

impl WDBRow {
    pub fn new(fields: Vec<WDBValue>) -> WDBRow {
        WDBRow {
            fields
        }
    }

    pub fn fields(&self) -> &[WDBValue] {
        &self.fields
    }
}

// macro for converting a n-ary tuple to a WDBRow
macro_rules! impl_from_tuple_for_wdbrow {
    ($(
        $Tuple:ident {
            $(($idx:tt) -> $T:ident)+
        }
    )+) => {
        $(
            impl<$($T:Into<WDBValue>),+> From<($($T,)+)> for WDBRow {
                fn from(tuple: ($($T,)+)) -> WDBRow {
                    let fields = vec![
                        $(tuple.$idx.into(),)+
                    ];
                    WDBRow::new(fields)
                }
            }
        )+
    }
}

impl_from_tuple_for_wdbrow! {
    Tuple1 {
        (0) -> A
    }
    Tuple2 {
        (0) -> A
        (1) -> B
    }
    Tuple3 {
        (0) -> A
        (1) -> B
        (2) -> C
    }
    Tuple4 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_to_wdb_row() {
        let actual_row : WDBRow = (123, "abc", 456, "edf").into();
        let expected_row = WDBRow::new(vec![
            WDBValue::Integer(123),
            WDBValue::Text("abc".into()),
            WDBValue::Integer(456),
            WDBValue::Text("edf".into()),
        ]);
        assert_eq!(actual_row, expected_row);
    }
}
