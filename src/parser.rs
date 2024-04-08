use nom::{
    character::complete::{char, one_of},
    combinator::{map, recognize},
    complete::take,
    multi::{many0, many1},
    sequence::terminated,
    IResult,
};

enum TokenType {
    Number,
    Plus,
    Minus,
    LParen,
    RParen,
}

struct Token {
    kind: TokenType,
    lexeme: String,
}



struct SExpr {
    function: TokenType,
    args: Vec<SExpr>,
}

type TokenizerResult<'source> = IResult<&'source str, Token>;

fn number<'source>(source: &'source str) -> TokenizerResult<'source> {
    map(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        |lexeme: &str| Token {
            kind: TokenType::Number,
            lexeme: lexeme.to_string(),
        },
    )(source)
}

fn operator(source: &str) -> TokenizerResult {
    let build_token = |(_, lexeme: &str)| Token {
        kind: match lexeme {
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '(' => TokenType::LParen,
            ')' => TokenType::RParen,
            _ => unreachable!(),
        },
        lexeme: lexeme.to_string(),
    };
    let mut parser = map(take(1usize), build_token);
    parser(source)
}

fn expression(source: &str) -> TokenizerResult {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::parser::Token;

    #[test]
    fn number() {
        use super::number;
        assert!(matches!(
            number("123"),
            Ok((
                "",
                Token {
                    kind: super::TokenType::Number,
                    lexeme: _
                }
            ))
        ));
    }
}
