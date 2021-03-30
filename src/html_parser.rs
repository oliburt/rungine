#[derive(Debug, PartialEq)]
pub struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    pub fn new(input: String) -> Self {
        Self { input, pos: 0 }
    }

    pub fn next_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    pub fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    pub fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_next_char() {
        let p = Parser::new("test".into());
        assert_eq!(p.next_char(), Some('t'));
        assert_eq!(p.next_char(), Some('t'));

        let p1 = Parser::new("".into());

        assert_eq!(p1.next_char(), None);
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
    }
}
