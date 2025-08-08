![Logo](assets/logo.png)

<div align="center">

[![License](https://img.shields.io/crates/l/chaos-engine?style=square&color=red&label=License)](https://crates.io/crates/chaos-engine)
[![crates.io](https://img.shields.io/crates/v/chaos-engine?style=square&color=red)](https://crates.io/crates/chaos-engine)
[![Downloads](https://img.shields.io/crates/d/chaos-engine?style=square&color=red&label=Downloads)](https://crates.io/crates/chaos-engine)
[![Rustfmt](https://img.shields.io/badge/style-rustfmt-ff69b4?style=square&color=red&label=Style)](https://github.com/rust-lang/rustfmt)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/recleun/chaos-engine/rust.yml?style=square&logo=github&label=Build)
[![docs.rs](https://img.shields.io/docsrs/chaos-engine?style=square&label=docs.rs)](https://docs.rs/chaos-engine)

</div>

# chaos-engine

The ChaosEngine is a useful library that helps CLI-based projects that actively take input from the user to perform actions, which is suitable for text-based games in terminals.

The way it works is you create a few Page instances, which would contain the text to be displayed by the program, then at the bottom the user would have to give some text input. All inputs made by the user must be handled by you, where you could exit the program/display a new page (like progressing in a story), start a mini-game on a different page, etc.

## Features
ChaosEngine includes a few useful features, including:

- Automatic word wrapping, with respect to paddings.
- Automatic re-adjustments to word wrapping when the terminal gets resized.
- Ability to create different pages to easily print whichever one you want depending on your logic.

## Example Usage
Note: Do not be overwhelmed by how big this example is, it's just comments that explain the code. Just 21 lines of code if you exclude the comments.

```rs
use chaos_engine::{Chaos, ChaosOptions, Page, types::Vector2}

fn main() {
    let stdout = std::io::stdout();

    // These are needed options to customize how your program will look.
    let options = ChaosOptions::new(
        // The input label, what text the user will see at the bottom right before the input.
        String::from("Input: "),
        // The X and Y paddings for the input line.
        // X would be how many spaces to the left of the input label.
        Vector2::new(1, 1),
        // The X and Y paddings for the main text output section (buffer).
        // X gets split into left and right (4 each), and Y is how many spaces from the top the text
        // will have.
        Vector2::new(8, 2)
    );

    // Instantiate chaos, passing the needed stdout and options.
    // This is mostly what you will work with, whether it be printing pages, taking input, etc.
    // Alternate screens are isolated terminal buffers, they are useful to restore the terminal
    // state back to how it was, so activating them is very useful.
    let mut chaos = Chaos::new(stdout, options);
    chaos.alternate_screen(true);

    // Instantiate a page where it will contain some text to display.
    // A page by default is empty, therefore you need to push text to it so it gets properly
    // displayed.
    let mut page = Page::new();
    page.push("hello, world!");

    // This loop will contain the logic of the program, as ChaosEngine works by repeatedly
    // taking input from the user.
    // You can check the user input for specific words or patterns yourself, then execute some
    // code.
    loop {
        // Clearing the terminal is necessary for each loop, to prevent printing over existing
        // text.
        chaos.clear_terminal();

        // Get text input from the user.
        let input = chaos.get_input(&mut page).unwrap();

        // Test for what the input could possibly be, if it's "exit", then quit the program.
        match input.as_str() {
            "exit" => {
                // Breaking the loop is how you quit the program.
                // Exiting the alternate screen is necessary before exiting to restore back the
                // text that was on the terminal before starting the program.
                chaos.alternate_screen(false);
                break;
            },
            _ => (),
        }
    }
}
```

## Related Projects

[chaos-engine-contain](https://crates.io/crates/chaos-engine-contain) - A really useful binary crate where you can containerize your ChaosEngine program, and reload it in-place by exiting the program with a specific exit code.
