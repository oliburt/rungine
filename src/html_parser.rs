#[derive(Debug, PartialEq)]
pub struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    pub fn new(input: String) -> Self {
        Self { input, pos: 0 }
    }

    /// Read the current character at the position of the parser without consuming it.
    pub fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }

    /// Returns true if character/set of characters staring from the parser position
    /// start with the provided str
    pub fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    /// Returns true if all input is consumed.
    pub fn eof(&self) -> bool {
        self.current_char().is_none()
    }

    /// Return the current character, and advance self.pos to the next character.
    pub fn consume_char(&mut self) -> Option<char> {
        let cur_char = self.input.chars().nth(self.pos);
        self.pos += 1;
        cur_char
    }

    pub fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.current_char().unwrap()) {
            result.push(self.consume_char().unwrap());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume_while() {
        let mut p = Parser::new("test".into());

        let res = p.consume_while(|_| true);
        assert_eq!(res, "test".to_string());

        let mut p = Parser::new("abcdefgh".into());
        let res = p.consume_while(|character| character < 'd');
        assert_eq!(res, "abc".to_string());
    }

    #[test]
    fn test_consume_char() {
        let mut p = Parser::new("test".to_string());
        assert_eq!(p.consume_char(), Some('t'));
        assert_eq!(p.consume_char(), Some('e'));
        assert_eq!(p.consume_char(), Some('s'));
        assert_eq!(p.consume_char(), Some('t'));
        assert_eq!(p.consume_char(), None);
        assert_eq!(p.consume_char(), None);

        let mut p1 = Parser {
            input: "test".into(),
            pos: 2,
        };

        assert_eq!(p1.consume_char(), Some('s'));
        assert_eq!(p1.consume_char(), Some('t'));
        assert_eq!(p1.consume_char(), None);
    }

    #[test]
    fn test_new() {
        let input = "test".to_string();
        assert_eq!(
            Parser::new(input),
            Parser {
                input: "test".into(),
                pos: 0
            }
        )
    }

    #[test]
    fn test_eof() {
        let p0 = Parser {
            input: "test".to_string(),
            pos: 1,
        };

        assert_eq!(p0.eof(), false);

        let p1 = Parser {
            input: "test".to_string(),
            pos: 4,
        };
        assert_eq!(p1.eof(), true);
        let p2 = Parser {
            input: "test".to_string(),
            pos: 5,
        };
        assert_eq!(p2.eof(), true);

        let p3 = Parser::new("".into());
        assert_eq!(p3.eof(), true);
    }

    #[test]
    fn test_current_char() {
        let p = Parser::new("test".into());
        assert_eq!(p.current_char(), Some('t'));
        assert_eq!(p.current_char(), Some('t'));

        let p1 = Parser::new("".into());

        assert_eq!(p1.current_char(), None);
    }

    #[test]
    fn test_next_starts_with() {
        let p = Parser::new("test".into());
        assert_eq!(p.starts_with("t"), true);
        assert_eq!(p.starts_with("test"), true);
        assert_eq!(p.starts_with("tes"), true);
        assert_eq!(p.starts_with(""), true);

        assert_eq!(p.starts_with("r"), false);
        assert_eq!(p.starts_with("rand"), false);
        assert_eq!(p.starts_with("est"), false);

        let p1 = Parser::new("".into());
        assert_eq!(p1.starts_with("t"), false);
        assert_eq!(p1.starts_with(""), true);

        let p2 = Parser {
            input: "testing".into(),
            pos: 2,
        };

        assert_eq!(p2.starts_with(""), true);
        assert_eq!(p2.starts_with("s"), true);
        assert_eq!(p2.starts_with("sting"), true);
        assert_eq!(p2.starts_with("t"), false);
        assert_eq!(p2.starts_with("test"), false);
    }
}
