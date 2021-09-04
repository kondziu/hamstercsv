use unicode_segmentation::UnicodeSegmentation;

pub const ELIPSIS: &'static str = "…";
pub const PAGE: &'static str = "⤶"; //"▼";
pub const PADDING: &'static str = " ";

pub trait MaleableUnicode<'a>: Sized {
    type Into;
    type Dimensions;
    fn cut_or_pad_to(&'a self, dimensions: Self::Dimensions, pad_with: &'static str) -> Self::Into;
}

impl<'a> MaleableUnicode<'a> for String {
    type Into = Vec<&'a str>;
    type Dimensions = usize;
    fn cut_or_pad_to(&'a self, width: Self::Dimensions, pad_with: &'static str) -> Self::Into {
        log::info!("cut_or_pad_to<String> {:?} {:?} {:?}", self, width, pad_with);
        log::info!("graphemes {:?}", self.graphemes(true).collect::<Vec<&str>>());
        log::info!("graphemes[n    ] {:?}", self.graphemes(true).take(10).collect::<Vec<&str>>());
        log::info!("graphemes[n + 1] {:?}", self.graphemes(true).take(10 + 1).collect::<Vec<&str>>());

        // Grab one extra grapheme to check if there are too many
        let mut graphemes: Vec<&str> = self.graphemes(true)
            .take(width + 1)
            .collect();            

        let grapheme_count = graphemes.len();

        log::info!("graphemes_count {:?}", grapheme_count);
        if grapheme_count > width {

            // Remove the extra
            graphemes.pop();

            // Replace last fitting charactewr with overlong row indicator
            graphemes.pop();
            graphemes.push(ELIPSIS);
        }
        log::info!("graphemes => {:?}", graphemes);

        // Append padding, if needed
        log::info!("w - gc {:?} - {:?}", width, grapheme_count);
        if width > grapheme_count {
            for _ in 0..(width - grapheme_count) {            
                graphemes.push(pad_with);
            }
        }

        log::info!("graphemes => {:?}", graphemes);
        graphemes
    }
}

#[derive(Debug)]
pub struct CSVItem {
    rows: Vec<String>,    
    width: usize,
    height: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct CellDimentions { pub width: usize, pub height: usize } // TODO necessary?

impl<'a> MaleableUnicode<'a> for CSVItem {
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
            last_row.push(PAGE);
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

impl Default for CSVItem {
    fn default() -> Self {
        CSVItem { rows: vec![], width: 0, height: 0 }
    }
}

impl From<String> for CSVItem {
    fn from(string: String) -> Self {
        CSVItem::from(string.as_str())
    }
}

impl From<&str> for CSVItem {
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
        if width > max_width {
            max_width = width;
        }

        CSVItem {            
            width: max_width,
            height: rows.len(),
            rows,
        }
    }
}

#[derive(Debug)]
pub struct CSVColumn {
    header: String,
    values: Vec<CSVItem>,
    max_width: usize,
    max_height: usize,
    // type?
}

impl Default for CSVColumn {
    fn default() -> Self {
        CSVColumn { header: String::default(), values: Vec::new(), max_width: 0, max_height: 0 }
    }
}

impl CSVColumn {
    pub fn from_header(header: String) -> Self {
        CSVColumn { header, values: Vec::new(), max_width: 0, max_height: 0 }
    }
    pub fn header(&self) -> &str {
        self.header.as_str()
    }
    fn set_value(&mut self, index: usize, value: CSVItem) {
        while index > self.values.len() {
            self.values.push(CSVItem::default());            
        } 
        self.values.push(value);
    }
    // fn push_value(&mut self, value: CSVItem) {        
    //     self.values.push(value);
    // }
    pub fn value(&self, index: usize) -> Option<&CSVItem> {
        self.values.get(index)
    }    
    pub fn values(&self) -> impl Iterator<Item=&CSVItem> {
        self.values.iter()
    }
    pub fn row_count(&self) -> usize {
        self.values.len()
    }
}

#[derive(Debug)]
pub struct CSVFile {
    columns: Vec<CSVColumn>,
}

impl CSVFile {
    pub fn new() -> Self {
        CSVFile { columns: Vec::new() }
    }

    pub fn new_column(&mut self, header: String) {        
        self.columns.push(CSVColumn::from_header(header));
    }

    pub fn get_column(&self, column_index: usize) -> Option<&CSVColumn> {
        self.columns.get(column_index)
    }

    fn get_column_mut(&mut self, column_index: usize) -> &mut CSVColumn {
        while column_index >= self.columns.len() {
            self.columns.push(CSVColumn::default());            
        } 
        &mut self.columns[column_index]
    }

    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    pub fn row_count(&mut self) -> usize {
        // TODO: thius is inefficient for large column counts.
        self.columns.iter().map(|column| column.row_count()).max().unwrap_or(0)        
    }
}

impl<R> From<csv::Reader<R>> for CSVFile where R: std::io::Read {
    fn from(mut reader: csv::Reader<R>) -> Self { 

        let mut csv = CSVFile::new();

        if reader.has_headers() {
            let headers = reader.headers()
                .expect("Error reading CSV file");      
            for header in headers {
                log::info!("header: {}", header);
                csv.new_column(header.to_owned());
            }
        }
            
        for (row_index, row) in reader.records().into_iter().enumerate() {
            let row = row.expect(&format!("Error reading row {} in CSV file", row_index));
            for (column_index, value) in row.into_iter().enumerate() {
                log::info!("item col:{}: {}", column_index, value);
                let item = CSVItem::from(value);
                let column = csv.get_column_mut(column_index);
                column.set_value(row_index, item);
            }            
        }

        csv
    }
}