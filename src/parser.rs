#[derive(Debug)]
pub enum Ast {
    Literal { value: isize },
    BinOp { leftexp: Box<Ast>, op: BinOp, rightexp: Box<Ast> },
    UnOp { op: UnOp, exp: Box<Ast> },
}


#[derive(Debug)]
pub enum BinOp {
    Plus,
    Minus,
    Mult,
    Div,
    Modulo
}


#[derive(Debug)]
pub enum UnOp {
    Neg
}

#[derive(Debug)]
enum Token<'a> {
    Keyword(&'a str),
    Literal(isize)
}

const KEYWORDS: [&'static str; 7] = ["+", "-", "/", "*", "%", "(", ")"];

pub fn parse(string: &str) -> Ast {
    let tokens = lexer(string);
    println!("{:?}", tokens);
    parse_tokens(&tokens[..])
}

fn parse_tokens(tokens: &[Token]) -> Ast {
    let mut strongest = None;
    let mut strongest_index = None;
    let mut indent = 0;
    tokens.iter().enumerate().for_each(|(idx, token)|{
            match token {
                Token::Keyword("(") => indent += 1,
                Token::Keyword(")") => indent -= 1,
                _ => {
                    if indent == 0 {
                        match strongest {
                            None => {
                                strongest = Some(token);
                                strongest_index = Some(idx);
                            }
                            Some(t) => {
                                if prescedence(token, t) {
                                    strongest = Some(token);
                                    strongest_index = Some(idx);
                                }
                            }
                        }
                    }
                }
            }
        });
    match strongest {
        None => {
            parse_tokens(&tokens[1..tokens.len() - 1])
        }
        Some(Token::Keyword(keyword)) => {
            if strongest_index == Some(0) {
                Ast::UnOp { op: match *keyword {
                    "-" => UnOp::Neg,
                    _ => panic!("Invalid UnOp {}", keyword)
                }, exp: Box::new(parse_tokens(&tokens[1..])) }
            }
            else {
                Ast::BinOp { leftexp: Box::new(parse_tokens(&tokens[0..strongest_index.unwrap()])), op: match *keyword {
                    "-" => BinOp::Minus,
                    "+" => BinOp::Plus,
                    "*" => BinOp::Mult,
                    "/" => BinOp::Div,
                    "%" => BinOp::Modulo,
                    _ => panic!("Invalid BinOp {}", keyword)
                }, rightexp: Box::new(parse_tokens(&tokens[strongest_index.unwrap() + 1..])) }
            }
        }
        Some(Token::Literal(c)) => {
            Ast::Literal { value: *c }
        }
    }
}

fn lexer<'a>(string: &'a str) -> Vec<Token<'a>> {
    string.split_ascii_whitespace().map(|token_string| {
            if KEYWORDS.iter().any(|keyword| keyword == &token_string) {
                Token::Keyword(token_string)
            }
            else if token_string.chars().all(|char| char.is_numeric()) {
                Token::Literal(isize::from_str_radix(token_string, 10).unwrap())
            }
            else {
                panic!("{} is not a valid token", token_string);
            }
        })
        .collect()
}

fn prescedence(a: &Token, b: &Token) -> bool {
    !match (a, b) {
        (&Token::Keyword("%"), _) => false,
        (_, &Token::Keyword("%")) => true,
        (&Token::Keyword("+") | &Token::Keyword("-"), _) => false,
        (_, &Token::Keyword("+") | &Token::Keyword("-")) => true,
        (&Token::Keyword("*") | &Token::Keyword("/"), _) => false,
        (_, &Token::Keyword("*") | &Token::Keyword("/")) => true,
        (_, _) => false
    }
}

