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
        assert!(self.parser.next_char().is_alphabetic());

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
        let text = self.parser.consume_while(&|next_char| {
            return next_char != ',' && next_char != '{';
        });

        return cssom::Selector::new(&text.trim());
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
        assert!(rulesets[0].selectors[0] == cssom::Selector::new("ul"));
        assert!(rulesets[1].selectors[0] == cssom::Selector::new("p"));
        assert!(parser.parser.eof());
    }

    #[test]
    fn test_parser_parse_ruleset() {
        let mut parser = Parser::new("ul { padding-left: 1rem; list-style: square; }");
        let ruleset = parser.parse_ruleset();

        assert!(ruleset.selectors == cssom::Selectors::from([cssom::Selector::new("ul")]));
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
        assert!(selectors[0] == cssom::Selector::new("ul"));
        assert!(selectors[1] == cssom::Selector::new("ol"));

        assert!(
            selectors
                == cssom::Selectors::from(
                    [cssom::Selector::new("ul"), cssom::Selector::new("ol"),]
                )
        );
    }

    #[test]
    fn test_parser_parse_selector() {
        let mut parser = Parser::new("ul, ol { padding-left: 1rem; }");
        let selector = parser.parse_selector();

        assert!(selector == cssom::Selector::new("ul"));

        // Trims whitespace
        let mut parser = Parser::new("ul { padding-left: 1rem; }");
        let selector = parser.parse_selector();

        assert!(selector == cssom::Selector::new("ul"));
    }
}
