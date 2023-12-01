use thiserror::Error;

#[derive(Error, Debug)]
enum ParseError {
    #[error("expected {expected:?}, found {found:?}")]
    Expected { expected: String, found: Token },
    #[error("unexpected end of input")]
    UnexpectedEndOfInput,
    #[error("invalid token {0:?}")]
    InvalidToken(char),
}

#[derive(Debug, Copy, Clone)]
enum Token {
    LParen,
    RParen,
    Digit(u32),
    Plus,
    Minus,
}

/* grammars

expr   = term { ("+" | "-"), expr };
term   = "(", expr, ")" | number;
number = digit, { digit };
digit  = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"

*/

#[derive(Debug)]
enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Number(i32),
}

fn parse_expr(tokens: &[Token]) -> Result<(Expr, &[Token]), ParseError> {
    let (term, remaining) = parse_term(tokens)?;

    match remaining.first() {
        // addition
        Some(Token::Plus) => {
            let (other, remaining) = parse_expr(&remaining[1..])?;
            Ok((Expr::Add(Box::new(term), Box::new(other)), remaining))
        }

        // subtraction
        Some(Token::Minus) => {
            let (other, remaining) = parse_expr(&remaining[1..])?;
            Ok((Expr::Sub(Box::new(term), Box::new(other)), remaining))
        }

        // only a term
        Some(_) => Ok((term, remaining)),

        // done parsing
        None => Ok((term, remaining)),
    }
}

fn parse_term(tokens: &[Token]) -> Result<(Expr, &[Token]), ParseError> {
    match tokens.first() {
        // parenthesis
        Some(Token::LParen) => {
            let (expr, remaining) = parse_expr(&tokens[1..])?;

            match remaining.first() {
                Some(Token::RParen) => Ok((expr, &remaining[1..])),
                Some(token) => Err(ParseError::Expected {
                    expected: "right parenthesis".to_string(),
                    found: *token,
                }),
                None => Err(ParseError::UnexpectedEndOfInput),
            }
        }
        Some(_) => parse_number(tokens),
        None => Err(ParseError::UnexpectedEndOfInput),
    }
}

fn parse_number(tokens: &[Token]) -> Result<(Expr, &[Token]), ParseError> {
    match tokens.first() {
        Some(Token::Digit(_)) => {
            let mut num_digits = 1;

            let number = tokens
                .iter()
                .take_while(|&token| match token {
                    Token::Digit(_) => true,
                    _ => false,
                })
                .map(|token| match token {
                    Token::Digit(d) => *d,
                    _ => unreachable!(),
                })
                .reduce(|total, digit| {
                    num_digits += 1;
                    total * 10 + digit
                })
                .unwrap();

            Ok((Expr::Number(number as i32), &tokens[num_digits..]))
        }

        // bad token
        Some(token) => Err(ParseError::Expected {
            expected: "digit".to_string(),
            found: *token,
        }),

        // end of line
        None => Err(ParseError::UnexpectedEndOfInput),
    }
}

fn tokenize(input: &String) -> Result<Vec<Token>, ParseError> {
    input
        .chars()
        .enumerate()
        .filter(|(_, c)| !c.is_whitespace())
        .map(|(_, c)| match c {
            '(' => Ok(Token::LParen),
            ')' => Ok(Token::RParen),
            '+' => Ok(Token::Plus),
            '-' => Ok(Token::Minus),
            _ => Ok(Token::Digit(
                c.to_digit(10).ok_or(ParseError::InvalidToken(c))?,
            )),
        })
        .collect::<Result<Vec<Token>, ParseError>>()
}

fn parse(input: &String) -> Result<i32, ParseError> {
    let tokens = tokenize(input)?;
    let (expr, remaining) = parse_expr(&tokens)?;

    dbg!(expr);

    if !remaining.is_empty() {
        dbg!(remaining);
        return Err(ParseError::UnexpectedEndOfInput);
    }

    Ok(0)
}

fn main() {
    // get all chars after the program name
    let input = std::env::args().skip(1).collect::<Vec<String>>().join(" ");

    println!("input: {}", input);

    match parse(&input) {
        // Ok(result) => println!("{}", result),
        Err(err) => println!("error: {:?}", err),
        _ => {}
    }
}
