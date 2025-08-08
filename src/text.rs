use crate::Chaos;

/// A page is a struct containing text to be printed by ChaosEngine. Pages are the building blocks
/// for the programs made by the engine, they must be used to print text to the output.
///
/// # Examples
///
/// ```no_run
/// use chaos_engine::Page;
///
/// let mut page = Page::new();
/// page.push("hello, world!");
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Page {
    text: Vec<String>,
    raw_text: Vec<String>,
}

impl Page {
    /// Instantiate a new page to store text.
    pub fn new() -> Self {
        Self {
            text: Vec::new(),
            raw_text: Vec::new(),
        }
    }

    /// Push some string to the page. Each push will start on its own line.
    pub fn push(&mut self, text: &str) {
        self.raw_text.push(text.to_string());
    }

    /// Pop the last string pushed to the page.
    pub fn pop(&mut self) -> Option<String> {
        self.raw_text.pop()
    }

    /// Get the stored aligned text.
    ///
    /// Aligned text puts paddings and word-wrapping into consideration.
    pub fn text(&self) -> &Vec<String> {
        &self.text
    }

    /// Get the stored raw text.
    pub fn raw_text(&self) -> &Vec<String> {
        &self.raw_text
    }

    /// Align the stored raw text, in other words, convert it to a properly formatted text,
    /// respecting paddings and word-wrapping.
    pub fn align(&mut self, chaos: &Chaos) {
        if self.raw_text.is_empty() {
            return;
        }

        let buffer_padding_x = chaos.paddings().buffer.x;
        let dimensions = &chaos.dimensions();
        self.text = Vec::new();

        for string in &self.raw_text {
            let words: Vec<&str> = string.split_whitespace().collect();
            let mut left_chars = dimensions.x as i32 - buffer_padding_x as i32;
            let mut line = String::new();

            for i in 0..words.len() {
                let word = words[i];
                let len = word.len() as i32;
                if len > dimensions.x as i32 - buffer_padding_x as i32 {
                    for c in word.chars() {
                        if left_chars > 1 {
                            line += &format!("{c}");
                            left_chars -= 1;
                        } else {
                            line += &format!("{c}");
                            self.text.push(line);
                            line = String::new();
                            left_chars = dimensions.x as i32 - buffer_padding_x as i32;
                        }
                    }
                } else if left_chars > len {
                    line += &format!("{word} ");
                    left_chars -= len + 1;
                } else {
                    self.text.push(line);
                    line = String::new();
                    line += &format!("{word} ");
                    left_chars = dimensions.x as i32 - buffer_padding_x as i32 - len;
                }
            }
            self.text.push(line);
        }
    }
}
