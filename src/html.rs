use crate::dom;
use std::vec::Vec;

pub struct Parser {
    input: String,
    pos: usize,
}

impl Parser {
    fn new(input: &str) -> Self {
        return Self {
            input: String::from(input),
            pos: 0,
        };
    }

    fn parse(input: &str) -> dom::Nodes {
        let mut parser = Self::new(input);

        return parser.parse_nodes();
    }

    fn parse_nodes(&mut self) -> dom::Nodes {
        let mut result = Vec::new();

        while !self.eof() && !self.starts_with("</") {
            result.push(self.parse_node());
        }

        return result;
    }

    fn parse_node(&mut self) -> dom::Node {
        if self.next_char() == '<' {
            return self.parse_element();
        }

        return self.parse_text();
    }

    fn parse_element(&mut self) -> dom::Node {
        // Opening tag
        assert!(self.next_char() == '<');
        self.consume_char();
        let opening_tag =
            self.consume_while(&|next_char| next_char != '>' && !next_char.is_whitespace());

        // Attributes
        let mut attrs = dom::AttrMap::new();

        self.consume_whitespace();

        while self.next_char() != '>' && !self.starts_with("/>") {
            let (name, value) = self.parse_attribute();
            attrs.insert(name, value);

            self.consume_whitespace();
        }

        println!("{}", self.next_char());

        // Void element
        if self.starts_with("/>") {
            return dom::element(&opening_tag, dom::Nodes::new(), attrs);
        }

        assert!(self.next_char() == '>');
        self.consume_char();

        // Child nodes
        let children = self.parse_nodes();

        // Closing tag
        assert!(self.next_char() == '<');
        self.consume_char();
        assert!(self.next_char() == '/');
        self.consume_char();
        let closing_tag = self.consume_while(&|next_char| next_char != '>');
        assert!(self.next_char() == '>');
        self.consume_char();

        assert!(opening_tag == closing_tag);

        return dom::element(&opening_tag, children, attrs);
    }

    fn parse_attribute(&mut self) -> (String, String) {
        assert!(self.next_char().is_alphabetic());

        let name = self.consume_while(&|next_char| next_char != '=');

        assert!(self.next_char() == '=');
        self.consume_char();
        assert!(self.next_char() == '"');
        self.consume_char();

        let value = self.consume_while(&|next_char| next_char != '"');

        assert!(self.next_char() == '"');
        self.consume_char();

        return (name, value);
    }

    fn parse_text(&mut self) -> dom::Node {
        assert!(self.next_char() != '<');

        let text = self.consume_while(&|next_char| next_char != '<');

        return dom::text(&text);
    }

    fn consume_whitespace(&mut self) -> String {
        return self.consume_while(&|next_char| next_char.is_whitespace());
    }

    fn consume_while(&mut self, cond: &dyn Fn(char) -> bool) -> String {
        let mut result = String::new();

        while !self.eof() && cond(self.next_char()) {
            result.push(self.consume_char());
        }

        return result;
    }

    fn consume_char(&mut self) -> char {
        let current_char = self.input[self.pos..].chars().next().unwrap();

        self.pos += 1;

        while !self.input.is_char_boundary(self.pos) {
            self.pos += 1;
        }

        return current_char;
    }

    fn next_char(&self) -> char {
        return self.input[self.pos..].chars().next().unwrap();
    }

    fn starts_with(&self, prefix: &str) -> bool {
        return self.input[self.pos..].starts_with(prefix);
    }

    fn eof(&self) -> bool {
        return self.pos >= self.input.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_parse() {
        let nodes = Parser::parse("<p>Hello World!</p>");
        assert!(nodes.len() == 1);
        assert!(
            nodes[0] == dom::element("p", vec![dom::text("Hello World!")], dom::AttrMap::new())
        );
    }

    #[test]
    fn test_parser_parse_nodes() {
        let mut parser = Parser::new("<html>Hello World!<p>Lorem ipsum</p></html>");
        parser.pos = 6;

        let nodes = parser.parse_nodes();
        assert!(nodes.len() == 2);
        assert!(nodes[0] == dom::text("Hello World!"));
        assert!(nodes[1] == dom::element("p", vec![dom::text("Lorem ipsum")], dom::AttrMap::new()));

        assert!(parser.pos == 36); // before the closing html tag
    }

    #[test]
    fn test_parser_parse_node() {
        let mut parser = Parser::new("Hello World!<p>Lorem ipsum</p>");

        let node = parser.parse_node();
        assert!(node == dom::text("Hello World!"));

        let node = parser.parse_node();
        assert!(node == dom::element("p", vec![dom::text("Lorem ipsum")], dom::AttrMap::new()));
    }

    #[test]
    fn test_parser_parse_element_void() {
        let mut parser = Parser::new("<br />");
        let node = parser.parse_element();

        assert!(node == dom::element("br", dom::Nodes::new(), dom::AttrMap::new()));

        let mut parser = Parser::new("<img src=\"./cat.jpg\" alt=\"Cat\" />");
        let node = parser.parse_element();

        let children = dom::Nodes::new();
        let attrs = dom::AttrMap::from([
            ("src".to_owned(), "./cat.jpg".to_owned()),
            ("alt".to_owned(), "Cat".to_owned()),
        ]);

        assert!(node == dom::element("img", children, attrs));
    }

    #[test]
    fn test_parser_parse_element_with_attributes() {
        let mut parser = Parser::new("<div id=\"foo\" class=\"bar\"></div>");
        let node = parser.parse_element();

        let children = dom::Nodes::new();
        let attrs = dom::AttrMap::from([
            ("id".to_owned(), "foo".to_owned()),
            ("class".to_owned(), "bar".to_owned()),
        ]);

        assert!(node == dom::element("div", children, attrs));
    }

    #[test]
    fn test_parser_parse_element() {
        let mut parser = Parser::new("<html>Child node</html>");
        let node = parser.parse_element();

        assert!(node == dom::element("html", vec![dom::text("Child node")], dom::AttrMap::new()));
    }

    #[test]
    fn test_parser_parse_attribute() {
        let mut parser = Parser::new("id=\"foobar\"");
        let (name, value) = parser.parse_attribute();

        assert!(name == "id");
        assert!(value == "foobar");
    }

    #[test]
    fn test_parser_parse_text() {
        let mut parser = Parser::new("Hello <strong>World</strong>!");

        let node = parser.parse_text();
        assert!(node == dom::text("Hello "));
    }

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
