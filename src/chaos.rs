use crate::{Page, types::Vector2};
use crossterm::{
    ExecutableCommand, cursor,
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{self, enable_raw_mode},
};
use std::io::{self, Write};

/// The primary struct of chaos-engine.
///
/// This struct must be instantiated once to start using chaos-engine and its features.
///
/// # Examples
///
/// ```no_run
/// use chaos_engine::{Chaos, ChaosOptions};
///
/// let stdout = std::io::stdout();
/// let options = ChaosOptions::default();
///
/// let mut chaos = Chaos::new(stdout, options);
/// ```
pub struct Chaos<'a> {
    paddings: ChaosPaddings,
    stdout: io::Stdout,
    input_label: &'a str,
    dimensions: Vector2<u16>,
    position: Vector2<u16>,
}

impl<'a> Chaos<'a> {
    /// Instantiate the chaos engine with specified options.
    ///
    /// It enables raw mode where input must be handled manually.
    pub fn new(stdout: io::Stdout, options: ChaosOptions<'a>) -> Self {
        enable_raw_mode().unwrap();

        Self {
            stdout,
            input_label: options.input_label,
            dimensions: Self::get_dimensions(),
            position: Self::get_position(),
            paddings: ChaosPaddings {
                input: Vector2::new(options.input_padding.x, options.input_padding.y),
                buffer: Vector2::new(options.buffer_padding.x, options.buffer_padding.y),
            },
        }
    }

    /// Completely clears the terminal screen of any visible text.
    ///
    /// # Panics
    ///
    /// Panics in the case of a terminal error.
    pub fn clear_terminal(&mut self) {
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();
    }

    /// Returns the X position of the last character in the input.
    fn last_character_pos(&self, input_len: usize) -> u16 {
        // left padding + input label + input length
        self.paddings.input.x + self.input_label.len() as u16 + 1 + input_len as u16
    }

    /// Gets input from the user.
    ///
    /// It takes a page to resize in the event of a terminal resize, then writes the input
    /// prompt on the last line of the terminal, overwriting anything on that line.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use chaos_engine::{Chaos, ChaosOptions, Page};
    ///
    /// let mut page = Page::new();
    /// let mut chaos = Chaos::new(std::io::stdout(), ChaosOptions::default());
    ///
    /// loop {
    ///     chaos.clear_terminal();
    ///     chaos.print(&mut page);
    ///
    ///     let input = chaos.get_input(&mut page).unwrap();
    ///     if input == "exit" {
    ///         chaos.alternate_screen(false);
    ///         break;
    ///     }
    ///
    ///     // do stuff here
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// This can panic when it fails to read the terminal events.
    pub fn get_input(&mut self, page: &mut Page) -> Result<String, io::Error> {
        let mut input = String::new();
        self.prepare_input();

        loop {
            match event::read()? {
                Event::Resize(_, _) => {
                    self.update_dimensions();
                    page.align(&self);
                    self.clear_terminal();
                    self.print(page);
                    self.prepare_input();

                    let last_character_pos = self.last_character_pos(input.len());

                    if last_character_pos < self.dimensions.x {
                        print!("{input}");
                        self.move_cursor(last_character_pos, self.dimensions.y - 1);
                        self.update_position();
                    } else {
                        input = String::new();
                    }
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                }) if !input.is_empty() => {
                    self.move_cursor(self.position.x - 1, self.position.y);
                    print!(" ");
                    self.move_cursor(self.position.x - 1, self.position.y);
                    self.update_position();
                    input.pop();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    ..
                }) if c.is_ascii()
                    && self.dimensions.x - 1 > self.last_character_pos(input.len()) =>
                {
                    print!("{c}");
                    self.move_cursor(self.position.x + 1, self.position.y);
                    self.update_position();
                    input.push(c);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    ..
                }) => break,
                _ => (),
            }
        }

        Ok(input)
    }

    /// Prints the input prompt on the last line, and moves the cursor to the right position.
    fn prepare_input(&mut self) {
        self.move_cursor(self.paddings.input.x, self.dimensions.y - 1);
        print!("{}", self.input_label);
        self.move_cursor(
            self.paddings.input.x + self.input_label.len() as u16 + 1,
            self.dimensions.y - 1,
        );
        self.update_position();
    }

    /// Moves the cursor to the specified X and Y positions.
    ///
    /// # Panics
    ///
    /// Panics in the case of a terminal error.
    pub fn move_cursor(&mut self, x: u16, y: u16) {
        self.stdout.execute(cursor::MoveTo(x, y)).unwrap();
        self.stdout.flush().unwrap();
    }

    /// Enables and disables the terminal's alternate screen.
    ///
    /// An alternate screen is a separate buffer. On entering an alternate screen,
    /// the terminal gets completely cleared to allow for program output, and once
    /// the screen is disabled, the original buffer is restored.
    ///
    /// # Panics
    ///
    /// Panics in the case of a terminal error.
    pub fn alternate_screen(&mut self, on: bool) {
        if on {
            self.stdout.execute(terminal::EnterAlternateScreen).unwrap();
        } else {
            self.stdout.execute(terminal::LeaveAlternateScreen).unwrap();
        }
    }

    /// Prints the given `Page` onto the screen, respecting the paddings and word wrapping.
    ///
    /// Calls `Page::align()` on the given `Page` to apply the word wrapping before
    /// printing it to the output.
    pub fn print(&mut self, page: &mut Page) {
        let mut starting_line = self.paddings.buffer.y - 1;
        self.move_cursor(starting_line, 0);
        page.align(&self);

        for index in 0..page.text().len() {
            let string = &page.text()[index];
            if index >= self.dimensions.y as usize - 1 {
                continue;
            }
            starting_line += 1;
            self.move_cursor(self.paddings.buffer.x / 2, starting_line);
            print!("{string}");
        }
    }

    /// Returns the last stored position of the cursor.
    pub fn position(&self) -> &Vector2<u16> {
        &self.position
    }

    /// Returns the current cursor position.
    fn get_position() -> Vector2<u16> {
        let (pos_x, pos_y) = cursor::position().unwrap();
        Vector2::new(pos_x, pos_y)
    }

    /// Updates the stored cursor position to the current one.
    fn update_position(&mut self) {
        self.position = Self::get_position();
    }

    /// Returns the last stored dimensions of the terminal.
    pub fn dimensions(&self) -> &Vector2<u16> {
        &self.dimensions
    }

    /// Returns the current terminal dimensions.
    ///
    /// # Panics
    ///
    /// Panics in the case of a terminal error.
    fn get_dimensions() -> Vector2<u16> {
        let (dim_x, dim_y) = terminal::size().unwrap();
        Vector2::new(dim_x, dim_y)
    }

    /// Updates the stored dimensions of the terminal.
    fn update_dimensions(&mut self) {
        self.dimensions = Self::get_dimensions();
    }

    /// Returns the current paddings.
    pub fn paddings(&self) -> &ChaosPaddings {
        &self.paddings
    }

    /// Updates the paddings to new values. Any active page must be printed again to take effect.
    pub fn update_paddings(&mut self, padding: PaddingType, new_padding: Vector2<u16>) {
        match padding {
            PaddingType::Input => self.paddings.input = new_padding,
            PaddingType::Buffer => self.paddings.buffer = new_padding,
        }
    }
}

/// A helper struct to set some options for a [`Chaos`] instance.
///
/// # Examples
///
/// ```
/// use chaos_engine::{ChaosOptions, types::Vector2};
///
/// let options = ChaosOptions {
///     input_label: "Input:", // The input label
///     input_padding: Vector2::new(1, 1), // Input paddings (bottom line where input is written)
///     buffer_padding: Vector2::new(4, 2), // Buffer paddings (main text output area)
/// };
/// ```
pub struct ChaosOptions<'a> {
    pub input_padding: Vector2<u16>,
    pub buffer_padding: Vector2<u16>,
    pub input_label: &'a str,
}

impl<'a> Default for ChaosOptions<'a> {
    fn default() -> Self {
        ChaosOptions {
            input_label: "Input:",
            input_padding: Vector2::new(1, 0),
            buffer_padding: Vector2::new(8, 2),
        }
    }
}

/// A struct that stores paddings for the input and buffer sections of the terminal.
pub struct ChaosPaddings {
    pub input: Vector2<u16>,
    pub buffer: Vector2<u16>,
}

/// An enum containing all possible types of paddings.
pub enum PaddingType {
    Input,
    Buffer,
}
