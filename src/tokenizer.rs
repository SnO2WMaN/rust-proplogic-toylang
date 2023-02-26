use anyhow::{bail, Ok, Result};

#[derive(Debug, PartialEq)]
enum LogicFn0 {
  TAUTO,
  FALSO,
}

#[derive(Debug, PartialEq)]
enum LogicFn1 {
  NOT,
}

#[derive(Debug, PartialEq)]
enum LogicFn2 {
  AND,
  OR,
  IMP,
  EQ,
}

#[derive(Debug, PartialEq)]
enum Symbol {
  Variable { name: char },
  LogicFn0(LogicFn0),
  LogicFn1(LogicFn1),
  LogicFn2(LogicFn2),
  ParenL,
  ParenR,
  Comma,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Range(usize, usize);
impl Range {
  fn by_pos(start: usize, end: usize) -> Range {
    return Range(start, end);
  }

  fn by_size(start: usize, size: usize) -> Range {
    return Range(start, start + size);
  }

  fn start(self) -> usize {
    let Range(start, _end) = self;
    start
  }

  fn end(self) -> usize {
    let Range(_start, end) = self;
    end
  }
}

#[derive(Debug, PartialEq)]
pub struct Token {
  symbol: Symbol,
  range: Range,
}

impl Token {
  fn make(symbol: Symbol, range: Range) -> Token {
    return Token { symbol, range };
  }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>> {
  let chars: Vec<char> = input.chars().collect();

  let mut tokens: Vec<Token> = vec![];
  let mut pos: usize = 0;

  macro_rules! lex_token {
    ($lexer:expr) => {{
      let (symbol, range) = $lexer;
      tokens.push(Token::make(symbol, range));
      pos = range.end();
    }};
  }

  while pos < chars.len() {
    match chars[pos] {
      '(' => {
        lex_token!(lex_paren_l(pos));
      }
      ')' => {
        lex_token!(lex_paren_r(pos));
      }
      ',' => {
        lex_token!(lex_comma(pos));
      }
      '⊤' => {
        lex_token!(lex_tauto(pos));
      }
      '⊥' => {
        lex_token!(lex_falso(pos));
      }
      '¬' => {
        lex_token!(lex_not(pos));
      }
      '∧' => {
        lex_token!(lex_and(pos));
      }
      '∨' => {
        lex_token!(lex_or(pos));
      }
      '→' => {
        lex_token!(lex_imp(pos));
      }
      '↔' => {
        lex_token!(lex_eq(pos));
      }
      c if is_variable(c) => {
        lex_token!(lex_variable(pos, c));
      }
      c => bail!("Invalid char: '{}' at {:?}", c, Range::by_size(pos, 1)),
    }
  }
  Ok(tokens)
}

fn is_variable(c: char) -> bool {
  matches! {c, 'p'|'q'|'r'|'s'|'t'}
}

fn lex_variable(pos: usize, name: char) -> (Symbol, Range) {
  return ((Symbol::Variable { name: name }), Range::by_size(pos, 1));
}

fn lex_paren_l(pos: usize) -> (Symbol, Range) {
  return (Symbol::ParenL, Range::by_size(pos, 1));
}

fn lex_paren_r(pos: usize) -> (Symbol, Range) {
  return (Symbol::ParenR, Range::by_size(pos, 1));
}

fn lex_comma(pos: usize) -> (Symbol, Range) {
  return (Symbol::Comma, Range::by_size(pos, 1));
}

fn lex_tauto(pos: usize) -> (Symbol, Range) {
  return (Symbol::LogicFn0(LogicFn0::TAUTO), Range::by_size(pos, 1));
}

fn lex_falso(pos: usize) -> (Symbol, Range) {
  return (Symbol::LogicFn0(LogicFn0::FALSO), Range::by_size(pos, 1));
}

fn lex_not(pos: usize) -> (Symbol, Range) {
  return (Symbol::LogicFn1(LogicFn1::NOT), Range::by_size(pos, 1));
}

fn lex_and(pos: usize) -> (Symbol, Range) {
  return (Symbol::LogicFn2(LogicFn2::AND), Range::by_size(pos, 1));
}

fn lex_or(pos: usize) -> (Symbol, Range) {
  return (Symbol::LogicFn2(LogicFn2::OR), Range::by_size(pos, 1));
}

fn lex_imp(pos: usize) -> (Symbol, Range) {
  return (Symbol::LogicFn2(LogicFn2::IMP), Range::by_size(pos, 1));
}

fn lex_eq(pos: usize) -> (Symbol, Range) {
  return (Symbol::LogicFn2(LogicFn2::EQ), Range::by_size(pos, 1));
}

#[cfg(test)]
mod test {
  use crate::tokenizer::{tokenize, LogicFn0, LogicFn1, LogicFn2, Range, Symbol, Token};

  #[test]
  fn tokenize_tauto() {
    let actual = tokenize("⊤()").expect("must be returned vec");

    assert_eq!(actual.len(), 3);
    assert_eq!(
      actual[0],
      Token::make(Symbol::LogicFn0(LogicFn0::TAUTO), Range::by_pos(0, 1))
    );
    assert_eq!(actual[1], Token::make(Symbol::ParenL, Range::by_pos(1, 2)));
    assert_eq!(actual[2], Token::make(Symbol::ParenR, Range::by_pos(2, 3)));
  }

  #[test]
  fn tokenize_falso() {
    let actual = tokenize("⊥()").expect("must be returned vec");

    assert_eq!(actual.len(), 3);
    assert_eq!(
      actual[0],
      Token::make(Symbol::LogicFn0(LogicFn0::FALSO), Range::by_pos(0, 1))
    );
    assert_eq!(actual[1], Token::make(Symbol::ParenL, Range::by_pos(1, 2)));
    assert_eq!(actual[2], Token::make(Symbol::ParenR, Range::by_pos(2, 3)));
  }

  #[test]
  fn tokenize_not() {
    let actual = tokenize("¬(p)").expect("must be returned vec");

    assert_eq!(actual.len(), 4);
    assert_eq!(
      actual[0],
      Token::make(Symbol::LogicFn1(LogicFn1::NOT), Range::by_pos(0, 1))
    );
    assert_eq!(actual[1], Token::make(Symbol::ParenL, Range::by_pos(1, 2)));
    assert_eq!(
      actual[2],
      Token::make(Symbol::Variable { name: 'p' }, Range::by_pos(2, 3))
    );
    assert_eq!(actual[3], Token::make(Symbol::ParenR, Range::by_pos(3, 4)));
  }

  #[test]
  fn tokenize_and() {
    let actual = tokenize("∧(p,q)").expect("must be returned vec");

    assert_eq!(actual.len(), 6);
    assert_eq!(
      actual[0],
      Token::make(Symbol::LogicFn2(LogicFn2::AND), Range::by_pos(0, 1))
    );
    assert_eq!(actual[1], Token::make(Symbol::ParenL, Range::by_pos(1, 2)));
    assert_eq!(
      actual[2],
      Token::make(Symbol::Variable { name: 'p' }, Range::by_pos(2, 3))
    );
    assert_eq!(actual[3], Token::make(Symbol::Comma, Range::by_pos(3, 4)));
    assert_eq!(
      actual[4],
      Token::make(Symbol::Variable { name: 'q' }, Range::by_pos(4, 5))
    );
    assert_eq!(actual[5], Token::make(Symbol::ParenR, Range::by_pos(5, 6)));
  }

  #[test]
  fn tokenize_or() {
    let actual = tokenize("∨(p,q)").expect("must be returned vec");

    assert_eq!(actual.len(), 6);
    assert_eq!(
      actual[0],
      Token::make(Symbol::LogicFn2(LogicFn2::OR), Range::by_pos(0, 1))
    );
    assert_eq!(actual[1], Token::make(Symbol::ParenL, Range::by_pos(1, 2)));
    assert_eq!(
      actual[2],
      Token::make(Symbol::Variable { name: 'p' }, Range::by_pos(2, 3))
    );
    assert_eq!(actual[3], Token::make(Symbol::Comma, Range::by_pos(3, 4)));
    assert_eq!(
      actual[4],
      Token::make(Symbol::Variable { name: 'q' }, Range::by_pos(4, 5))
    );
    assert_eq!(actual[5], Token::make(Symbol::ParenR, Range::by_pos(5, 6)));
  }

  #[test]
  fn tokenize_imp() {
    let actual = tokenize("→(p,q)").expect("must be returned vec");

    assert_eq!(actual.len(), 6);
    assert_eq!(
      actual[0],
      Token::make(Symbol::LogicFn2(LogicFn2::IMP), Range::by_pos(0, 1))
    );
    assert_eq!(actual[1], Token::make(Symbol::ParenL, Range::by_pos(1, 2)));
    assert_eq!(
      actual[2],
      Token::make(Symbol::Variable { name: 'p' }, Range::by_pos(2, 3))
    );
    assert_eq!(actual[3], Token::make(Symbol::Comma, Range::by_pos(3, 4)));
    assert_eq!(
      actual[4],
      Token::make(Symbol::Variable { name: 'q' }, Range::by_pos(4, 5))
    );
    assert_eq!(actual[5], Token::make(Symbol::ParenR, Range::by_pos(5, 6)));
  }

  #[test]
  fn tokenize_eq() {
    let actual = tokenize("↔(p,q)").expect("must be returned vec");

    assert_eq!(actual.len(), 6);
    assert_eq!(
      actual[0],
      Token::make(Symbol::LogicFn2(LogicFn2::EQ), Range::by_pos(0, 1))
    );
    assert_eq!(actual[1], Token::make(Symbol::ParenL, Range::by_pos(1, 2)));
    assert_eq!(
      actual[2],
      Token::make(Symbol::Variable { name: 'p' }, Range::by_pos(2, 3))
    );
    assert_eq!(actual[3], Token::make(Symbol::Comma, Range::by_pos(3, 4)));
    assert_eq!(
      actual[4],
      Token::make(Symbol::Variable { name: 'q' }, Range::by_pos(4, 5))
    );
    assert_eq!(actual[5], Token::make(Symbol::ParenR, Range::by_pos(5, 6)));
  }
}
