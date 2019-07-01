use super::*;
use std::io::{self, Read, Write};
use std::str::{FromStr};
use std::fmt;
use indoc::indoc;

#[derive(Debug, PartialEq)]
pub struct WDBTable {
    schema: WDBSchema,
    rows: Vec<WDBRow>,
}

impl WDBTable {
    pub fn new(schema: WDBSchema) -> WDBTable {
        WDBTable {
            schema: schema,
            rows: Vec::new(),
        }
    }

    pub fn add(&mut self, row: WDBRow) -> Result<&mut Self, WDBError> {
        self.schema.validate(&row)?;
        self.rows.push(row);
        Ok(self)
    }

    pub fn load_from_csv<R: Read>(schema: &WDBSchema, reader: R) -> Result<WDBTable, WDBError> {
        let mut rows = Vec::new();
        let mut csv_reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(reader);
        for record_result in csv_reader.records() {
            let record = record_result.map_err(|_| WDBError::BadCSV)?;
            let mut fields = Vec::with_capacity(schema.columns().len());
            for (t, s) in schema.columns().iter().zip(record.iter()) {
                let s = s.trim();
                match t {
                    WDBType::Text => {
                        fields.push(WDBValue::Text(s.to_owned()));
                    }
                    WDBType::Integer => {
                        let i = u64::from_str(s.trim()).
                            map_err(|_| WDBError::BadCSV)?;
                        fields.push(WDBValue::Integer(i));
                    }
                }
            }
            rows.push(WDBRow::new(fields));
        }
        Ok(WDBTable {
            schema: schema.clone(),
            rows: rows,
        })
    }

    pub fn dump_as_csv<W: Write>(&self, writer: W) -> Result<(), WDBError> {
        let mut csv_writer = csv::WriterBuilder::new()
            .has_headers(false)
            .from_writer(writer);
        for row in &self.rows {
            let fields_as_str = row.fields().iter()
                .map(|field| field.to_string());
            csv_writer.write_record(fields_as_str)
                .map_err(|_| WDBError::BadIO);
        }
        csv_writer.flush().map_err(|_| WDBError::BadIO)?;
        Ok(())
    }
}

impl fmt::Display for WDBTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output_csv = String::new();
        let mut buf_writer = io::BufWriter::new(unsafe {
            output_csv.as_mut_vec()
        });
        self.dump_as_csv(buf_writer).unwrap();
        write!(f, "{}", &output_csv)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // This csv about bio statistics has five columns:
    //      Name, Sex, Age, Height (in), and Weight (lbs)
    static BIOSTATS_CSV : &str = indoc!(r#"
        Alex,M,41,74,170
        Bert,M,42,68,166
        Carl,M,32,70,155
        Dave,M,39,72,167
        Elly,F,30,66,124
        Fran,F,33,66,115
        Gwen,F,26,64,121
        Hank,M,30,71,158
        Ivan,M,53,72,175
        Jake,M,32,69,143
        Kate,F,47,69,139
        Luke,M,34,72,163
        Myra,F,23,62,98
        Neil,M,36,75,160
        Omar,M,38,70,145
        Page,F,31,67,135
        Quin,M,29,71,176
        Ruth,F,28,65,131
    "#);

    fn get_biostats_table() -> Result<WDBTable, WDBError> {
        let input_csv = BIOSTATS_CSV;
        let mut buf_reader = io::BufReader::new(input_csv.as_bytes());
        let schema = WDBSchema::new({
            let cols = vec![
                WDBType::Text,      // Name
                WDBType::Text,      // Sex
                WDBType::Integer,   // Age
                WDBType::Integer,   // Height
                WDBType::Integer,   // Weight
            ];
            cols
        });
        WDBTable::load_from_csv(&schema, buf_reader)
    }

    #[test]
    fn load_from_and_dump_as_csv() -> Result<(), WDBError> {
        let input_csv = BIOSTATS_CSV;
        let biostats_table = get_biostats_table()?;
        let output_csv = biostats_table.to_string();
        assert_eq!(&input_csv, &output_csv);
        Ok(())
    }

    #[test]
    fn select() -> Result<(), WDBError> {
        Ok(())
    }

    #[test]
    fn project() -> Result<(), WDBError> {
        Ok(())
    }
}
