use crate::{HamsterCsv, config};

use super::{Csv, CsvColumn, CsvItem};

use std::convert::{TryFrom, TryInto};
use std::fmt::Display;
use std::path::PathBuf;

impl HamsterCsv {
    pub fn read_csv(config: &config::Config) -> Result<Csv, CsvError> {
        let builder = config.build()?;

        let reader = builder
            .from_path(&config.path)
            .map_err(|e| CsvError::CannotOpenFile { 
                path: config.path.clone(), 
                reason: e.to_string() 
            })?;

        let builder = 
            CsvBuilder::try_from(reader)?;

        builder.try_into()
    }
}

trait Build {
    fn build_with(&self, builder: &mut csv::ReaderBuilder) -> Result<(), CsvError>;
    fn build(&self) -> Result<csv::ReaderBuilder, CsvError> {
        let mut builder = csv::ReaderBuilder::new();
        self.build_with(&mut builder)?;
        Ok(builder)
    }
}

impl Build for config::Config {
    fn build_with(&self, builder: &mut csv::ReaderBuilder) -> Result<(), CsvError> {
        self.csv.build_with(builder)?;
        //self.aesthetics.build_with(builder)?;
        Ok(())
    }
}

impl Build for config::Csv {
    fn build_with(&self, builder: &mut csv::ReaderBuilder) -> Result<(), CsvError> {
        builder.has_headers(!self.no_headers);
        builder.delimiter(self.column_delimiter.as_byte());
        builder.terminator((&self.row_teminator).into());
        builder.trim(self.trim.into());
        builder.escape(self.escape.map(|escape| escape.as_byte()));
        builder.comment(self.comment.map(|comment| comment.as_byte()));

        builder.quoting(true);
        builder.double_quote(true);
        builder.flexible(true);

        Ok(())
    }
}

impl From<&config::Terminator> for csv::Terminator {
    fn from(terminator: &config::Terminator) -> Self {
        match terminator {
            config::Terminator::CRLF => csv::Terminator::CRLF,
            config::Terminator::CR => csv::Terminator::Any('\r' as u8),
            config::Terminator::LF => csv::Terminator::Any('\n' as u8),
        }
    }
}

impl From<config::Trim> for csv::Trim {
    fn from(trim: config::Trim) -> Self {
        match trim {
            config::Trim::None => csv::Trim::None,            
            config::Trim::OnlyHeaders => csv::Trim::Headers,
            config::Trim::ExceptHeaders => csv::Trim::Fields,
            config::Trim::All => csv::Trim::All,
        }
    }
}

pub struct CsvBuilder {
    columns: Vec<CsvColumn>,
}

impl CsvBuilder {
    pub fn new() -> Self {
        CsvBuilder { columns: Vec::new() }
    }

    fn new_column(&mut self, header: String) {        
        self.columns.push(CsvColumn::from_header(header));
    }

    pub fn row_count(&self) -> usize {
        self.columns.iter().map(|column| column.row_count()).max().unwrap_or(0)        
    }

    fn get_column_mut(&mut self, column_index: usize) -> &mut CsvColumn {
        while column_index >= self.columns.len() {
            self.columns.push(CsvColumn::default());            
        } 
        &mut self.columns[column_index]
    }
}

impl TryFrom<CsvBuilder> for Csv {
    type Error = CsvError;
    fn try_from(value: CsvBuilder) -> Result<Self, Self::Error> {
        Ok(Csv {
            max_rows: value.row_count(),
            columns: value.columns
        })
    }
}



// impl Into<csv::ReaderBuilder> for &config::Csv {    
//     fn into(self) -> csv::ReaderBuilder {
//         //fn try_from(&self, value: config::Csv) -> Result<Self, Self::Error> {
//         let mut builder = csv::ReaderBuilder::new();

//         builder.has_headers(!self.no_headers);
//         builder.delimiter(self.column_delimiter.as_byte());
//         builder.terminator(self.row_teminator.into());
//         builder.trim(self.trim.into());

//         builder.escape(self.escape.map(|escape| escape.as_byte()));
//         builder.comment(self.comment.map(|comment| comment.as_byte()));

//         // .quote(self.quote.as_u8())
//         // .quoting(!self.ignore_quotes)                        
//         // .double_quote(!self.ignore_double_quotes)        
//         // .flexible(!self.each_row_same_length);

//         builder

//         // builder
//         // .from_path(&self.path)
//         // .expect(&format!("Cannot open CSV file {:?}.", self.path))
//     }
// }

/*
pub fn build_reader(&self) -> csv::Reader<File> {
    let mut builder = csv::ReaderBuilder::new();

    builder
        .has_headers(!self.no_headers)
        .delimiter(self.column_delimiter.as_u8())
        .terminator(self.row_teminator.as_csv_terminator())
        .escape(self.escape.as_ref().map(|c| c.as_u8()))
        .comment(self.comment.as_ref().map(|c| c.as_u8()))
        .quote(self.quote.as_u8())
        .quoting(!self.ignore_quotes)                        
        .double_quote(!self.ignore_double_quotes)
        .trim(self.trim_whitespace.as_csv_trim())
        .flexible(!self.each_row_same_length);

    builder
        .from_path(&self.path)
        .expect(&format!("Cannot open CSV file {:?}.", self.path))
}
*/

impl<R> TryFrom<csv::Reader<R>> for CsvBuilder where R: std::io::Read {
    type Error = CsvError;

    fn try_from(mut reader: csv::Reader<R>) -> Result<Self, Self::Error> { 

        let mut builder = CsvBuilder::new();

        if reader.has_headers() {
            let headers = reader.headers()
                .expect("Error reading CSV file");      
            for header in headers {
                log::info!("header: {}", header);
                builder.new_column(header.to_owned());
            }
        }
            
        for (row_index, row) in reader.records().into_iter().enumerate() {
            let row = row.expect(&format!("Error reading row {} in CSV file", row_index));
            for (column_index, value) in row.into_iter().enumerate() {
                log::info!("item col:{}: {}", column_index, value);
                let item = CsvItem::from(value);
                let column = builder.get_column_mut(column_index);
                column.set_value(row_index, item);
            }            
        }

        Ok(builder)
    }
}

trait MutableColumn {

        fn set_value(&mut self, index: usize, value: CsvItem);
    
}

impl MutableColumn for CsvColumn {
        fn set_value(&mut self, index: usize, value: CsvItem) {
            while index > self.values.len() {
                self.values.push(CsvItem::default());            
            } 
            self.values.push(value);
        }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CsvError {
    CannotOpenFile{ path: PathBuf, reason: String }
}

impl std::error::Error for CsvError {}

impl Display for CsvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CsvError::CannotOpenFile {path, reason}=> write!(f, "Cannot open CSV file {:?} for reading: {}", path, reason),
        }
    }
}
