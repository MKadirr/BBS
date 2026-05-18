use crate::lexer::parser::parser::Parser;
use crate::lexer::token::Token;

mod parser;
mod parsing;

#[cfg(test)]
mod test {
    use crate::lexer::parser::parser::Parser;
    use crate::lexer::token::TokenType;

    #[test]
    fn test_empty() {
        let mut parser = Parser::new(String::from("")).unwrap();

        assert!(parser.get().ty == TokenType::EOF);
        assert!(parser.peak().ty == TokenType::EOF);
        parser.peak();
    }

    #[test]
    fn test_space() {
        let mut parser = Parser::new(String::from("   ")).unwrap();

        assert!(parser.get().ty == TokenType::Space);
    }

    #[test]
    fn test_eol() {
        let mut parser = Parser::new(String::from("\n")).unwrap();

        assert!(parser.get().ty == TokenType::Space);
    }

    #[test]
    fn test_tokens1() {
        let mut parser = Parser::new(String::from("\n   \n")).unwrap();

        let mut vec = Vec::new();

        while let t = parser.pop().unwrap() {
            if t.ty == TokenType::EOF {
                break
            }
            println!("{:?}", t.ty);
            vec.push(t.ty);
        }

        assert_eq!(vec, vec![TokenType::EOL, TokenType::Space, TokenType::EOL])
    }
}
