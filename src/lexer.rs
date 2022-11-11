use chumsky::prelude::*;

use crate::ast::{Span, Token};

pub fn lexer() -> impl Parser<char, Vec<(Token, Span)>, Error = Simple<char>> {
    // numbers
    let frac = just('.').chain(text::digits(10));
    let exp = just('e')
        .or(just('E'))
        .chain(just('+').or(just('-')).or_not())
        .chain::<char, _, _>(text::digits(10));

    let num = just('-')
        .or_not()
        .chain::<char, _, _>(text::int(10))
        .chain::<char, _, _>(frac.or_not().flatten())
        .chain::<char, _, _>(exp.or_not().flatten())
        .collect::<String>()
        .map(Token::Num)
        .labelled("num");

    // strings
    let escape = just('\\').ignore_then(
        just('\\')
            .or(just('/'))
            .or(just('"'))
            .or(just('\''))
            .or(just('b').to('\x08'))
            .or(just('f').to('\x0C'))
            .or(just('n').to('\n'))
            .or(just('r').to('\r'))
            .or(just('t').to('\t'))
            .or(just('u').ignore_then(
                filter(|c: &char| c.is_digit(16))
                    .repeated()
                    .exactly(4)
                    .collect::<String>()
                    .validate(|digits, span, emit| {
                        char::from_u32(u32::from_str_radix(&digits, 16).unwrap()).unwrap_or_else(
                            || {
                                emit(Simple::custom(span, "invalid unicode character"));
                                '\u{FFFD}' // unicode replacement character
                            },
                        )
                    }),
            )),
    );

    // the d stands for double quotes
    let d_string = just('"')
        .ignore_then(filter(|c| *c != '\\' && *c != '"').or(escape).repeated())
        .then_ignore(just('"'))
        .collect::<String>();

    // the s stands for single quotes
    let s_string = just('\'')
        .ignore_then(filter(|c| *c != '\\' && *c != '\'').or(escape).repeated())
        .then_ignore(just('\''))
        .collect::<String>();

    let string = d_string.or(s_string).map(Token::String).labelled("string");

    // operators
    let op = one_of("+-*/!=<>&|")
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map(Token::Op);

    // control characters
    let ctrl = one_of("()[]{};,").map(|c| Token::Ctrl(c));

    // identifiers
    let ident = text::ident().map(|ident: String| match ident.as_str() {
        "if" => Token::If,
        "else" => Token::Else,
        "true" => Token::Bool(true),
        "false" => Token::Bool(false),
        "null" => Token::Null,
        "and" => Token::Op("and".to_owned()),
        "or" => Token::Op("or".to_owned()),
        "in" => Token::Op("in".to_owned()),
        _ => Token::Ident(ident),
    });

    let token = num
        .or(string)
        .or(op)
        .or(ctrl)
        .or(ident)
        .recover_with(skip_then_retry_until([]));

    let comment = just("//").then(take_until(just('\n'))).padded();

    token
        .map_with_span(|token, span| (token, span))
        .padded_by(comment.repeated())
        .padded()
        .repeated()
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::ast::Token;

    use super::lexer;

    #[test]
    fn lex_int() {
        let lexed = lexer().parse("12").unwrap();

        assert_eq!(lexed, vec![(Token::Num("12".to_owned()), 0..2)])
    }

    #[test]
    fn lex_float() {
        let lexed = lexer().parse("32.5383").unwrap();

        assert_eq!(lexed, vec![(Token::Num("32.5383".to_owned()), 0..7)])
    }

    #[test]
    fn lex_exp() {
        let lexed = lexer().parse("3.2928e4").unwrap();

        assert_eq!(lexed, vec![(Token::Num("3.2928e4".to_owned()), 0..8)])
    }

    #[test]
    fn lex_infix() {
        let lexed = lexer().parse("400 + 300").unwrap();

        assert_eq!(
            lexed,
            vec![
                (Token::Num("400".to_owned()), 0..3),
                (Token::Op("+".to_owned()), 4..5),
                (Token::Num("300".to_owned()), 6..9)
            ]
        )
    }

    #[test]
    fn lex_s_string() {
        let lexed = lexer().parse("'cool'").unwrap();

        assert_eq!(lexed, vec![(Token::String("cool".to_owned()), 0..6)])
    }

    #[test]
    fn lex_d_string() {
        let lexed = lexer().parse(r#""cool""#).unwrap();

        assert_eq!(lexed, vec![(Token::String("cool".to_owned()), 0..6)])
    }

    #[test]
    fn lex_extra_whitespace() {
        let lexed = lexer()
            .parse(
                "32     -
		95   +  12",
            )
            .unwrap();

        assert_eq!(
            lexed,
            vec![
                (Token::Num("32".to_owned()), 0..2),
                (Token::Op("-".to_owned()), 7..8),
                (Token::Num("95".to_owned()), 11..13),
                (Token::Op("+".to_owned()), 16..17),
                (Token::Num("12".to_owned()), 19..21)
            ]
        )
    }

    #[test]
    fn lex_control() {
        let lexed = lexer().parse("({10})").unwrap();

        assert_eq!(
            lexed,
            vec![
                (Token::Ctrl('('), 0..1),
                (Token::Ctrl('{'), 1..2),
                (Token::Num("10".to_owned()), 2..4),
                (Token::Ctrl('}'), 4..5),
                (Token::Ctrl(')'), 5..6)
            ]
        )
    }

    #[test]
    fn lex_conditional() {
        let lexed = lexer().parse("if cool { nice } else { bad }").unwrap();

        assert_eq!(
            lexed,
            vec![
                (Token::If, 0..2),
                (Token::Ident("cool".to_owned()), 3..7),
                (Token::Ctrl('{'), 8..9),
                (Token::Ident("nice".to_owned()), 10..14),
                (Token::Ctrl('}'), 15..16),
                (Token::Else, 17..21),
                (Token::Ctrl('{'), 22..23),
                (Token::Ident("bad".to_owned()), 24..27),
                (Token::Ctrl('}'), 28..29)
            ]
        )
    }
}
