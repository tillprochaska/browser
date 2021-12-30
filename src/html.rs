use crate::dom;
use crate::parser;

pub struct Parser {
    parser: parser::Parser,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        return Self {
            parser: parser::Parser::new(input),
        };
    }

    pub fn parse(input: &str) -> dom::Nodes {
        let mut parser = Self::new(input);

        return parser.parse_nodes();
    }

    pub fn parse_nodes(&mut self) -> dom::Nodes {
        let mut result = dom::Nodes::new();

        while !self.parser.eof() && !self.parser.starts_with("</") {
            result.push(self.parse_node());
        }

        return result;
    }

    fn parse_node(&mut self) -> dom::Node {
        if self.parser.next_char() == '<' {
            return self.parse_element();
        }

        return self.parse_text();
    }

    fn parse_element(&mut self) -> dom::Node {
        // Opening tag
        assert!(self.parser.next_char() == '<');
        self.parser.consume_char();
        let opening_tag = self
            .parser
            .consume_while(&|next_char| next_char != '>' && !next_char.is_whitespace());

        // Attributes
        let mut attrs = dom::AttrMap::new();

        self.parser.consume_whitespace();

        while self.parser.next_char() != '>' && !self.parser.starts_with("/>") {
            let (name, value) = self.parse_attribute();
            attrs.insert(name, value);

            self.parser.consume_whitespace();
        }

        println!("{}", self.parser.next_char());

        // Void element
        if self.parser.starts_with("/>") {
            return dom::element(&opening_tag, dom::Nodes::new(), attrs);
        }

        assert!(self.parser.next_char() == '>');
        self.parser.consume_char();

        // Child nodes
        let children = self.parse_nodes();

        // Closing tag
        assert!(self.parser.next_char() == '<');
        self.parser.consume_char();
        assert!(self.parser.next_char() == '/');
        self.parser.consume_char();
        let closing_tag = self.parser.consume_while(&|next_char| next_char != '>');
        assert!(self.parser.next_char() == '>');
        self.parser.consume_char();

        assert!(opening_tag == closing_tag);

        return dom::element(&opening_tag, children, attrs);
    }

    fn parse_attribute(&mut self) -> (String, String) {
        assert!(self.parser.next_char().is_alphabetic());

        let name = self.parser.consume_while(&|next_char| next_char != '=');

        assert!(self.parser.next_char() == '=');
        self.parser.consume_char();
        assert!(self.parser.next_char() == '"');
        self.parser.consume_char();

        let value = self.parser.consume_while(&|next_char| next_char != '"');

        assert!(self.parser.next_char() == '"');
        self.parser.consume_char();

        return (name, value);
    }

    fn parse_text(&mut self) -> dom::Node {
        assert!(self.parser.next_char() != '<');

        let text = self.parser.consume_while(&|next_char| next_char != '<');

        return dom::text(&text);
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
        parser.parser.set_pos(6);

        let nodes = parser.parse_nodes();
        assert!(nodes.len() == 2);
        assert!(nodes[0] == dom::text("Hello World!"));
        assert!(nodes[1] == dom::element("p", vec![dom::text("Lorem ipsum")], dom::AttrMap::new()));

        assert!(parser.parser.pos() == 36); // before the closing html tag
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
}
