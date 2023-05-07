use unicode_segmentation::UnicodeSegmentation;

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
            graphemes.push(crate::data::ELIPSIS);
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
