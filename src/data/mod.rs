pub mod unicode;
pub mod builder;

pub use unicode::*;
use unicode_segmentation::UnicodeSegmentation;

pub const ELIPSIS: &'static str = "…";
pub const PAGE: &'static str = "⤶"; //"▼";
pub const PADDING: &'static str = " ";

#[derive(Debug)]
pub struct Csv {
    max_rows: usize,
    columns: Vec<CsvColumn>,
}

impl Csv {
    pub fn get_column(&self, column_index: usize) -> Option<&CsvColumn> {
        self.columns.get(column_index)
    }

    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    pub fn row_count(&self) -> usize {
        self.max_rows
    }
}


#[derive(Debug)]
pub struct CsvColumn {
    header: String,
    values: Vec<CsvItem>,
    // max_width: usize,
    // max_height: usize,
    // type?
}

impl Default for CsvColumn {
    fn default() -> Self {
        CsvColumn { header: String::default(), values: Vec::new(), /*max_width: 0, max_height: 0*/ }
    }
}

impl CsvColumn {
    pub fn from_header(header: String) -> Self {
        CsvColumn { header, values: Vec::new(), /*max_width: 0, max_height: 0*/ }
    }
    pub fn header(&self) -> &str {
        self.header.as_str()
    }
    pub fn value(&self, index: usize) -> Option<&CsvItem> {
        self.values.get(index)
    }    
    pub fn values(&self) -> impl Iterator<Item=&CsvItem> {
        self.values.iter()
    }
    pub fn row_count(&self) -> usize {
        self.values.len()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CellDimentions { pub width: usize, pub height: usize } // TODO necessary?

/**
 * A CSV cell consisting of zero, one or more lines of text.
 */
#[derive(Debug)]
pub struct CsvItem {
    rows: Vec<String>,    
    // width: usize,
    // height: usize,
}

impl Default for CsvItem {
    fn default() -> Self {
        CsvItem { rows: vec![], /*width: 0, height: 0*/ }
    }
}

impl From<String> for CsvItem {
    fn from(string: String) -> Self {
        CsvItem::from(string.as_str())
    }
}

impl From<&str> for CsvItem {
    fn from(string: &str) -> Self {
        let mut rows = Vec::new();
        let mut row = String::new();

        let mut max_width = 0;
        let mut width = 0;

        let mut previous = "";

        for current in string.graphemes(true) {
            match (previous, current) {
                ("\n", "\r") | ("\r", "\n") => {
                    // "Consume" sequence.
                    previous = "";
                }
                (_, "\n") | (_, "\r") => {
                    rows.push(row);                    
                    row = String::new();
                    if width > max_width {
                        max_width = width;
                    }
                    width = 0;
                }
                (_, grapheme) => { 
                    row.push_str(grapheme);
                    previous = current;
                    width += 1;
                }
            }            
        }

        rows.push(row);
        // if width > max_width {
        //     max_width = width;
        // }

        CsvItem {            
            // width: max_width,
            // height: rows.len(),
            rows,
        }
    }
}

impl<'a> MaleableUnicode<'a> for CsvItem {
    type Into = Vec<Vec<&'a str>>;
    type Dimensions = CellDimentions;
    fn cut_or_pad_to(&'a self, dimensions: Self::Dimensions, pad_with: &'static str) -> Self::Into {
        log::info!("cut_or_pad_to<CSVItem> {:?} {:?} {:?}", self, dimensions, pad_with);
        let mut rows: Vec<Vec<&str>> = self.rows.iter()
            .map(|row| row.cut_or_pad_to(dimensions.width, pad_with)).take(dimensions.height + 1)
            //.map(|row| Vec::new())
            .collect();
        
        let row_count = rows.len();
        if row_count > dimensions.height {
    
            // Remove the extra
            rows.pop();

            // Replace last fitting row with one that has an overlong indicator
            let mut last_row = rows.pop().unwrap();
            last_row.pop();
            last_row.push(crate::data::PAGE);
            rows.push(last_row);
        }

        if row_count < dimensions.height {
            
            // Create padding vector of appropriate width
            let mut padding: Vec<&str> = Vec::new();
            for _ in 0..dimensions.width {
                padding.push(pad_with);
            }

            // Append padding, as needed
            for _ in 0..(dimensions.height - row_count) {            
                rows.push(padding.clone());
            }
        }

        rows
    }
}