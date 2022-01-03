use crate::cssom;
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

    pub fn parse(input: &str) -> cssom::Rulesets {
        let mut parser = Self::new(input);

        return parser.parse_rulesets();
    }

    fn parse_rulesets(&mut self) -> cssom::Rulesets {
        let mut rulesets = cssom::Rulesets::new();

        while !self.parser.eof() {
            rulesets.push(self.parse_ruleset());
            self.parser.consume_whitespace();
        }

        return rulesets;
    }

    fn parse_ruleset(&mut self) -> cssom::Ruleset {
        let selectors = self.parse_selectors();

        self.parser.consume_whitespace();
        assert!(self.parser.next_char() == '{');
        self.parser.consume_char();
        self.parser.consume_whitespace();

        let declarations = self.parse_declarations();

        self.parser.consume_whitespace();
        assert!(self.parser.next_char() == '}');
        self.parser.consume_char();

        return cssom::Ruleset::new(selectors, declarations);
    }

    fn parse_declarations(&mut self) -> cssom::Declarations {
        let mut declarations = cssom::Declarations::new();

        while self.parser.next_char() != '}' {
            let (property, value) = self.parse_declaration();
            declarations.insert(property, value);
            self.parser.consume_whitespace();
        }

        return declarations;
    }

    fn parse_declaration(&mut self) -> (String, String) {
        let property = self.parser.consume_while(&|next_char| next_char != ':');

        self.parser.consume_char();
        self.parser.consume_whitespace();

        let value = self.parser.consume_while(&|next_char| next_char != ';');
        assert!(self.parser.next_char() == ';');
        self.parser.consume_char();

        return (property.trim().to_owned(), value.trim().to_owned());
    }

    fn parse_selectors(&mut self) -> cssom::Selectors {
        let mut selectors = cssom::Selectors::new();

        while self.parser.next_char() != '{' {
            selectors.push(self.parse_selector());
            self.parser.consume_whitespace();

            if self.parser.next_char() == ',' {
                self.parser.consume_char();
                self.parser.consume_whitespace();
            }
        }

        return selectors;
    }

    fn parse_selector(&mut self) -> cssom::Selector {
        let mut selector = cssom::Selector::new();
        let tag = self.consume_identifier();

        if tag.len() > 0 {
            selector = selector.tag(&tag);
        }

        while !self.parser.eof() && self.parser.next_char() != ',' && self.parser.next_char() != '{'
        {
            match self.parser.next_char() {
                '.' => {
                    self.parser.consume_char();
                    selector = selector.class(&self.consume_identifier());
                }

                '#' => {
                    self.parser.consume_char();
                    selector = selector.id(&self.consume_identifier());
                }

                '[' => {
                    self.parser.consume_char();
                    self.parser.consume_whitespace();

                    let name = self.parser.consume_while(&|next_char| {
                        return next_char != '=' && !next_char.is_whitespace();
                    });

                    self.parser.consume_whitespace();
                    assert!(self.parser.next_char() == '=');
                    self.parser.consume_char();
                    self.parser.consume_whitespace();
                    assert!(self.parser.next_char() == '"');
                    self.parser.consume_char();

                    let value = self.parser.consume_while(&|next_char| next_char != '"');

                    self.parser.consume_char();
                    self.parser.consume_whitespace();
                    println!("{}", self.parser.next_char());
                    assert!(self.parser.next_char() == ']');
                    self.parser.consume_char();

                    selector = selector.attr(&name, &value);
                }

                _ if self.parser.next_char().is_whitespace() => {
                    self.parser.consume_whitespace();
                    assert!(self.parser.next_char() == ',' || self.parser.next_char() == '{');
                }

                _ => assert!(false),
            }
        }

        return selector;
    }

    fn consume_identifier(&mut self) -> String {
        return self.parser.consume_while(&|next_char| {
            // This isn’t spec-compliant, as identifiers must start with an
            // alphabetic char -- we’ll ignore that for now.
            return next_char.is_alphanumeric() || next_char == '-' || next_char == '_';
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_parse_rulesets() {
        let mut parser = Parser::new("ul { padding-left: 1rem; } p { font-family: serif; }");
        let rulesets = parser.parse_rulesets();

        assert!(rulesets.len() == 2);
        assert!(rulesets[0].selectors[0] == cssom::Selector::new().tag("ul"));
        assert!(rulesets[1].selectors[0] == cssom::Selector::new().tag("p"));
        assert!(parser.parser.eof());
    }

    #[test]
    fn test_parser_parse_ruleset() {
        let mut parser = Parser::new("ul { padding-left: 1rem; list-style: square; }");
        let ruleset = parser.parse_ruleset();

        assert!(ruleset.selectors == cssom::Selectors::from([cssom::Selector::new().tag("ul")]));
        assert!(ruleset.declarations.len() == 2);
        assert!(parser.parser.eof());
    }

    #[test]
    fn test_parser_parse_declarations() {
        let mut parser = Parser::new("ul { padding-left: 1rem; list-style: square; }");
        parser.parser.set_pos(4);
        let declarations = parser.parse_declarations();

        assert!(declarations.len() == 2);
        assert!(declarations["padding-left"] == "1rem");
        assert!(declarations["list-style"] == "square");
    }

    #[test]
    fn test_parser_parse_declaration() {
        let mut parser = Parser::new("padding-left: 1rem;");
        let (property, value) = parser.parse_declaration();

        assert!(property == "padding-left");
        assert!(value == "1rem");
    }

    #[test]
    fn test_parser_parse_selectors() {
        let mut parser = Parser::new("ul, ol { padding-left: 1rem; }");
        let selectors = parser.parse_selectors();

        assert!(selectors.len() == 2);
        assert!(selectors[0] == cssom::Selector::new().tag("ul"));
        assert!(selectors[1] == cssom::Selector::new().tag("ol"));

        assert!(
            selectors
                == cssom::Selectors::from([
                    cssom::Selector::new().tag("ul"),
                    cssom::Selector::new().tag("ol"),
                ])
        );
    }

    #[test]
    fn test_parser_parse_selector() {
        let mut parser = Parser::new("ul, ol { padding-left: 1rem; }");
        let selector = parser.parse_selector();

        assert!(selector == cssom::Selector::new().tag("ul"));

        // Trims whitespace
        let mut parser = Parser::new("ul { padding-left: 1rem; }");
        let selector = parser.parse_selector();

        assert!(selector == cssom::Selector::new().tag("ul"));
    }

    #[test]
    fn test_parser_parse_selector_class() {
        let mut parser = Parser::new("p.class1.class2 { color: #333; }");
        let selector = parser.parse_selector();
        let expected = cssom::Selector::new()
            .tag("p")
            .class("class1")
            .class("class2");

        assert!(selector == expected);

        let mut parser = Parser::new(".class-1.class_2 { color: #333; }");
        let selector = parser.parse_selector();
        let expected = cssom::Selector::new().class("class-1").class("class_2");

        assert!(selector == expected);
    }

    #[test]
    fn test_parser_parse_selector_id() {
        let mut parser = Parser::new("p#intro { font-weight: italic; }");
        let selector = parser.parse_selector();
        let expected = cssom::Selector::new().tag("p").id("intro");

        assert!(selector == expected);

        let mut parser = Parser::new("#intro { font-weight: italic; }");
        let selector = parser.parse_selector();
        let expected = cssom::Selector::new().id("intro");

        assert!(selector == expected);
    }

    #[test]
    fn test_parser_parse_selector_attrs() {
        let mut parser = Parser::new("button[aria-expanded=\"true\"]");
        let selector = parser.parse_selector();
        let expected = cssom::Selector::new()
            .tag("button")
            .attr("aria-expanded", "true");

        assert!(selector == expected);

        let mut parser = Parser::new("button[ aria-expanded  =  \"true\" ]");
        let selector = parser.parse_selector();

        assert!(selector == expected);
    }
}
