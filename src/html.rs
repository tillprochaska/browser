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

        self.parser.consume_whitespace();

        while !self.parser.eof() && !self.parser.starts_with("</") {
            result.push(self.parse_node());
            self.parser.consume_whitespace();
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

        let mut element = dom::Element::new(&opening_tag);

        self.parser.consume_whitespace();

        while self.parser.next_char() != '>' && !self.parser.starts_with("/>") {
            let (name, value) = self.parse_attribute();
            element = element.attr(&name, &value);

            self.parser.consume_whitespace();
        }

        // Void element
        if self.parser.starts_with("/>") {
            return dom::Node::Element(element);
        }

        assert!(self.parser.next_char() == '>');
        self.parser.consume_char();

        // Child nodes
        element = element.children(self.parse_nodes());

        // Closing tag
        assert!(self.parser.next_char() == '<');
        self.parser.consume_char();
        assert!(self.parser.next_char() == '/');
        self.parser.consume_char();
        let closing_tag = self.parser.consume_while(&|next_char| next_char != '>');
        assert!(self.parser.next_char() == '>');
        self.parser.consume_char();

        assert!(opening_tag == closing_tag);

        return dom::Node::Element(element);
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

        return dom::Node::Text(text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_parse() {
        let nodes = Parser::parse("<p>Hello World!</p>");
        let expected = dom::Node::Element(
            dom::Element::new("p").child(dom::Node::Text("Hello World!".to_owned())),
        );

        assert!(nodes.len() == 1);
        assert!(nodes[0] == expected);
    }

    #[test]
    fn test_parser_parse_nodes() {
        let mut parser = Parser::new("<html>Hello World!<p>Lorem ipsum</p></html>");
        parser.parser.set_pos(6);

        let nodes = parser.parse_nodes();
        assert!(nodes.len() == 2);
        assert!(nodes[0] == dom::Node::Text("Hello World!".to_owned()));

        let expected = dom::Node::Element(
            dom::Element::new("p").child(dom::Node::Text("Lorem ipsum".to_owned())),
        );

        assert!(nodes[1] == expected);

        assert!(parser.parser.pos() == 36); // before the closing html tag
    }

    #[test]
    fn test_parser_parse_nodes_whitespace() {
        // Ignores leading whitespace
        assert!(Parser::new("  <html></html>").parse_nodes().len() == 1);

        // Ignores whitespace between elements
        assert!(Parser::new("<div></div>  <div></div>").parse_nodes().len() == 2);

        // Ignores tailing whitespace
        assert!(Parser::new("<html></html>  ").parse_nodes().len() == 1);
    }

    #[test]
    fn test_parser_parse_node() {
        let mut parser = Parser::new("Hello World!<p>Lorem ipsum</p>");

        let node = parser.parse_node();
        assert!(node == dom::Node::Text("Hello World!".to_owned()));

        let node = parser.parse_node();
        let expected = dom::Node::Element(
            dom::Element::new("p").child(dom::Node::Text("Lorem ipsum".to_owned())),
        );

        assert!(node == expected);
    }

    #[test]
    fn test_parser_parse_element_void() {
        let mut parser = Parser::new("<br />");
        let node = parser.parse_element();

        assert!(node == dom::Node::Element(dom::Element::new("br")));

        let mut parser = Parser::new("<img src=\"./cat.jpg\" alt=\"Cat\" />");
        let node = parser.parse_element();

        let expected = dom::Node::Element(
            dom::Element::new("img")
                .attr("src", "./cat.jpg")
                .attr("alt", "Cat"),
        );

        assert!(node == expected);
    }

    #[test]
    fn test_parser_parse_element_with_attributes() {
        let mut parser = Parser::new("<div id=\"foo\" class=\"bar\"></div>");
        let node = parser.parse_element();

        let expected = dom::Node::Element(
            dom::Element::new("div")
                .attr("id", "foo")
                .attr("class", "bar"),
        );

        assert!(node == expected);
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
        assert!(node == dom::Node::Text("Hello ".to_owned()));
    }
}
