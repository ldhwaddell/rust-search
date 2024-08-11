use std::str::CharIndices;

// Inspired by the https://eli.thegreenplace.net/2022/rewriting-the-lexer-benchmark-in-rust/#footnote-1
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token<'a> {
    EOF,
    Error,
    Number(&'a str),
    Word(&'a str),
    Symbol(&'a str),
}

/// A `Lexer` struct holds a reference to some content.
/// The lifetime parameter `'a` ensures that the reference to `content` is valid
/// for as long as the `Lexer` instance exists.
pub struct Lexer<'a> {
    // the course text
    input: &'a str,
    iter: CharIndices<'a>,

    // Last char seen by iter
    c: char,
    // Offset in the input
    ci: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lex = Self {
            input,
            iter: input.char_indices(),
            c: '\x00', // null character
            ci: 0,
        };

        lex.scan_char();
        lex
    }

    fn skip_nontokens(&mut self) {
        while self.c == ' ' || self.c == '\t' || self.c == '\r' || self.c == '\n' {
            self.scan_char()
        }
    }

    fn next_token(&mut self) -> Token<'a> {
        self.skip_nontokens();

        // Ensure we have not reached end
        if self.is_at_end() {
            return Token::EOF;
        }

        // If digit, return token with number
        if self.c.is_numeric() {
            return self.scan_number();
        }

        if self.c.is_alphabetic() {
            return self.scan_word();
        }

        if self.is_symbol(self.c) {
            return self.scan_symbol();
        }

        Token::Error
    }

    fn scan_word(&mut self) -> Token<'a> {
        let startpos = self.ci;
        while self.c.is_alphanumeric() || self.c == '\'' || self.c == '-' {
            self.scan_char();
        }

        Token::Word(&self.input[startpos..self.ci])
    }

    fn scan_number(&mut self) -> Token<'a> {
        let startpos = self.ci;
        while self.c.is_numeric() {
            self.scan_char();
        }

        Token::Number(&self.input[startpos..self.ci])
    }

    fn scan_symbol(&mut self) -> Token<'a> {
        let startpos = self.ci;
        while self.is_symbol(self.c) {
            self.scan_char();
        }

        Token::Symbol(&self.input[startpos..self.ci])
    }

    fn is_symbol(&self, c: char) -> bool {
        match c {
            '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '(' | ')' | '-' | '_' | '+' | '='
            | '{' | '}' | '[' | ']' | ':' | ';' | '"' | '\'' | '<' | '>' | ',' | '.' | '?'
            | '/' | '\\' | '|' => true,
            _ => false,
        }
    }

    fn scan_char(&mut self) {
        // If there is a next character, use it
        if let Some((ci, c)) = self.iter.next() {
            self.ci = ci;
            self.c = c;
        // Otherwise, set c to null character
        } else {
            self.ci = self.input.len();
            self.c = '\x00'
        }
    }

    fn is_at_end(&self) -> bool {
        self.ci >= self.input.len()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let tok = self.next_token();

        match tok {
            Token::EOF => None,
            Token::Error => None,
            _ => Some(tok),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Should skip leading spaces until it finds a letter or character
    #[test]
    fn skip_leading_spaces() {
        let leading_spaces = "   \t\t   \n\r  test";
        let target = "test";

        let lexer = Lexer::new(&leading_spaces);
        let toks: Vec<Token> = lexer.collect();

        assert_eq!(toks.len(), 1);
        assert_eq!(toks[0], Token::Word(target));
    }

    /// Should skip everything
    #[test]
    fn skip_all() {
        let spaces = "   \r     \n    \t\t ";

        let lexer = Lexer::new(spaces);
        let toks: Vec<Token> = lexer.collect();

        assert_eq!(toks.len(), 0);
    }

    /// Should extract numbers
    #[test]
    fn scan_nums() {
        let nums = "123 456 789";

        let mut lexer = Lexer::new(nums);

        assert_eq!(lexer.next_token(), Token::Number("123"));
        assert_eq!(lexer.next_token(), Token::Number("456"));
        assert_eq!(lexer.next_token(), Token::Number("789"));
        assert_eq!(lexer.next_token(), Token::EOF);
    }

    /// Should extract words
    #[test]
    fn scan_word() {
        let words = "test they're WhaT";

        let mut lexer = Lexer::new(words);

        assert_eq!(lexer.next_token(), Token::Word("test"));
        assert_eq!(lexer.next_token(), Token::Word("they're"));
        assert_eq!(lexer.next_token(), Token::Word("WhaT"));
        assert_eq!(lexer.next_token(), Token::EOF);
    }

    /// Should scan 'symbols' from text
    #[test]
    fn scan_symbol() {
        let symbols = "?;' \\^%  $#@ *()-=+";

        let mut lexer = Lexer::new(symbols);

        assert_eq!(lexer.next_token(), Token::Symbol("?;'"));
        assert_eq!(lexer.next_token(), Token::Symbol("\\^%"));
        assert_eq!(lexer.next_token(), Token::Symbol("$#@"));
        assert_eq!(lexer.next_token(), Token::Symbol("*()-=+"));
        assert_eq!(lexer.next_token(), Token::EOF);
    }

    /// Should get words, symbols, and numbers!
    #[test]
    fn scan_everything() {
        let text = "1 billion 45$ please";

        let mut lexer = Lexer::new(text);

        assert_eq!(lexer.next_token(), Token::Number("1"));
        assert_eq!(lexer.next_token(), Token::Word("billion"));
        assert_eq!(lexer.next_token(), Token::Number("45"));
        assert_eq!(lexer.next_token(), Token::Symbol("$"));
        assert_eq!(lexer.next_token(), Token::Word("please"));
        assert_eq!(lexer.next_token(), Token::EOF);
    }
}
