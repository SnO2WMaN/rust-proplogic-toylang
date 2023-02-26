use anyhow::Result;

#[derive(Debug)]
enum LogicFn0 {
    TRUE,
    FALSE,
}

#[derive(Debug)]
enum LogicFn1 {
    NOT,
}

#[derive(Debug)]
enum LogicFn2 {
    AND,
    OR,
    IMP,
    EQ,
}

#[derive(Debug)]
enum Symbol {
    Variable { name: char },
    LogicFn0(LogicFn0),
    LogicFn1(LogicFn1),
    LogicFn2(LogicFn2),
    ParenL,
    ParenR,
    Comma,
}

#[derive(Debug)]
pub struct Token {
    symbol: Symbol,
    start: i64,
    end: i64,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();
    tokens.push(Token {
        symbol: Symbol::LogicFn2(LogicFn2::AND),
        start: 0,
        end: 1,
    });
    tokens.push(Token {
        symbol: Symbol::ParenL,
        start: 1,
        end: 2,
    });
    tokens.push(Token {
        symbol: Symbol::Variable { name: 'p' },
        start: 2,
        end: 3,
    });
    tokens.push(Token {
        symbol: Symbol::Comma,
        start: 3,
        end: 4,
    });
    tokens.push(Token {
        symbol: Symbol::Variable { name: 'q' },
        start: 4,
        end: 5,
    });
    tokens.push(Token {
        symbol: Symbol::ParenR,
        start: 5,
        end: 6,
    });

    Ok(tokens)
}

#[cfg(test)]
mod test {
    use crate::tokenizer::tokenize;

    #[test]
    fn var() {
        tokenize("p");
        tokenize("q");
        tokenize("r");
        tokenize("s");
        tokenize("t");
    }

    #[test]
    fn tauto() {
        let actual = tokenize("⊤()");
    }

    #[test]
    fn falso() {
        let actual = tokenize("⊥()");
    }

    #[test]
    fn not() {
        let actual = tokenize("¬(p)");
    }

    #[test]
    fn and() {
        let actual = tokenize("∧(p,q)");
    }

    #[test]
    fn or() {
        let actual = tokenize("∨(p,q)");
    }

    #[test]
    fn imp() {
        let actual = tokenize("→(p,q)");
    }

    #[test]
    fn eq() {
        let actual = tokenize("↔(p,q)");
    }
}
