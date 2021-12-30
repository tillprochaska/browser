pub struct Parser {
    input: String,
    pos: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        return Self {
            input: String::from(input),
            pos: 0,
        };
    }

    pub fn consume_whitespace(&mut self) -> String {
        return self.consume_while(&|next_char| next_char.is_whitespace());
    }

    pub fn consume_while(&mut self, cond: &dyn Fn(char) -> bool) -> String {
        let mut result = String::new();

        while !self.eof() && cond(self.next_char()) {
            result.push(self.consume_char());
        }

        return result;
    }

    pub fn consume_char(&mut self) -> char {
        let current_char = self.input[self.pos..].chars().next().unwrap();

        self.pos += 1;

        while !self.input.is_char_boundary(self.pos) {
            self.pos += 1;
        }

        return current_char;
    }

    pub fn next_char(&self) -> char {
        return self.input[self.pos..].chars().next().unwrap();
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        return self.input[self.pos..].starts_with(prefix);
    }

    pub fn eof(&self) -> bool {
        return self.pos >= self.input.len();
    }

    pub fn set_pos(&mut self, pos: usize) -> &Self {
        self.pos = pos;

        return self;
    }

    pub fn pos(&self) -> usize {
        return self.pos;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_consume_whitespace() {
        let mut parser = Parser::new("   Hello World!");

        let result = parser.consume_whitespace();
        assert!(result == "   ");
    }

    #[test]
    fn test_parser_consume_while() {
        let mut parser = Parser::new("Hello World");

        let result = parser.consume_while(&|next_char| next_char != ' ');
        assert!(result == "Hello");

        parser.consume_char();

        // handles EOF
        let result = parser.consume_while(&|next_char| next_char != ' ');
        assert!(result == "World");
    }

    #[test]
    fn test_parser_consume_char() {
        let mut parser = Parser::new("aä");

        let result = parser.consume_char();

        assert!(result == 'a');
        assert!(parser.pos == 1);

        let result = parser.consume_char();

        // handles multi-byte characters
        assert!(result == 'ä');
        assert!(parser.pos == 3);
    }

    #[test]
    fn test_parser_next_char() {
        let mut parser = Parser::new("<html></html>");

        assert!(parser.next_char() == '<');
        parser.consume_char();
        assert!(parser.next_char() == 'h');
    }

    #[test]
    fn test_parser_starts_with() {
        let parser = Parser::new("<html></html>");

        assert!(parser.starts_with("<") == true);
        assert!(parser.starts_with("</") == false);
    }

    #[test]
    fn test_parser_eof() {
        let mut parser = Parser::new("<html></html>");
        assert!(parser.eof() == false);

        parser.pos = 13;
        assert!(parser.eof() == true);
    }
}
