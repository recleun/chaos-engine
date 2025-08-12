#[cfg(feature = "test")]
mod tests {
    use chaos_engine::ChaosTestOptions;
    use chaos_engine::{Chaos, Page, types};

    #[test]
    fn instantiation() {
        let page = Page::new();
        let text_should_expect: Vec<String> = Vec::new();
        let raw_text_should_expect: Vec<String> = Vec::new();

        assert_eq!(page.text(), &text_should_expect);
        assert_eq!(page.raw_text(), &raw_text_should_expect);
    }

    #[test]
    fn pushing_and_popping() {
        let text0 = "FIRST TEXT";
        let text1 = "SECOND TEXT";
        let text2 = "THIRD TEXT";

        let mut page = Page::new();

        // test pushing
        page.push(text0);
        let mut expected = vec![text0];
        assert_eq!(page.raw_text(), &expected);

        // test pushing with one element already pushed
        page.push(text1);
        page.push(text2);
        expected.push(text1);
        expected.push(text2);
        assert_eq!(page.raw_text(), &expected);

        // test popping one element
        page.pop();
        expected.pop();
        assert_eq!(page.raw_text(), &expected);

        // test clearing the page
        page.clear();
        assert_eq!(page.raw_text(), &Vec::<String>::new());
    }

    #[test]
    fn word_wrapping() {
        let options = ChaosTestOptions {
            stdout: std::io::stdout(),
            input_label: "",
            dimensions: types::Vector2::new(40, 40),
            buffer_padding: types::Vector2::new(0, 0),
            input_padding: types::Vector2::new(0, 0),
            position: types::Vector2::new(0, 0),
        };

        let chaos = Chaos::test_setup(options);
        let mut page = Page::new();
        page.push("This is a string that is enough to wrap into a new line.");
        page.align(&chaos);

        // string push should get wrapped onto a new line
        assert_eq!(page.text()[0], "This is a string that is enough to wrap ");
        assert_eq!(page.text()[1], "into a new line. ");

        page.push("This is a string that shouldn't wrap.");
        page.align(&chaos);

        // string that fits on one line should use only one line
        assert_eq!(page.text()[2], "This is a string that shouldn't wrap. ");

        page.push("000000000000000000000000000000000000000000000000000000000000");
        page.align(&chaos);

        // single words that are longer than a line should soft-break
        assert_eq!(page.text()[3], "0000000000000000000000000000000000000000");
        assert_eq!(page.text()[4], "00000000000000000000");
    }
}
