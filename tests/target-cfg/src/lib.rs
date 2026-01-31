#![no_std]

use logos::{Lexer, Logos};

/// Combined K=V tokens
#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"\s+")]
pub enum KeyValueToken<'s> {
    #[regex("([A-Z]+)", |lex| lex.slice())]
    MaybeKey(&'s str),
    #[regex("([0-9]+)", |lex| lex.slice())]
    MaybeValue(&'s str),
}

/// Key only tokens
#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"\s+")]
pub enum KeyToken<'s> {
    #[regex("([A-Z]+)", |lex| lex.slice())]
    MaybeKey(&'s str),
}

/// Value only tokens
#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"\s+")]
pub enum ValueToken<'s> {
    #[regex("([0-9]+)", |lex| lex.slice())]
    MaybeValue(&'s str),
}

/// Morphed K=V entry
#[inline]
pub fn parse_kv_morphed<'raw, U, F: FnMut(&mut U, KeyToken<'raw>, ValueToken<'raw>)>(
    user: &mut U,
    input: &'raw str,
    mut f: F,
) {
    let mut lexer: Lexer<'raw, KeyToken<'raw>> = KeyToken::lexer(input);

    while let Some(Ok(key_token)) = lexer.next() {
        let mut val_lexer: Lexer<'raw, ValueToken<'raw>> = lexer.morph();
        if let Some(Ok(val_token)) = val_lexer.next() {
            f(user, key_token, val_token);
        }
        lexer = val_lexer.morph();
    }
}

/// Non-Morphed K=V entry
#[inline]
pub fn parse_kv<'raw, U, F: FnMut(&mut U, KeyValueToken<'raw>)>(
    user: &mut U,
    input: &'raw str,
    mut f: F,
) {
    let mut lexer: Lexer<'raw, KeyValueToken<'raw>> = KeyValueToken::lexer(input);

    while let Some(Ok(key_val_token)) = lexer.next() {
        f(user, key_val_token);
    }
}

#[cfg(test)]
mod self_test {
    use super::*;

    #[test]
    fn morphed() {
        let mut u = 0;
        parse_kv_morphed(&mut u, "AAA000", |u, k, v| {
            *u += 1;
            assert_eq!(k, KeyToken::MaybeKey("AAA"));
            assert_eq!(v, ValueToken::MaybeValue("000"));
        });
        assert_eq!(u, 1);
    }

    #[test]
    fn not_morphed() {
        #[derive(Debug, PartialEq)]
        struct T<'raw> {
            inner: [Option<KeyValueToken<'raw>>; 2],
            idx: usize,
        }
        let mut u = T {
            inner: [const { None }; 2],
            idx: 0,
        };
        parse_kv(&mut u, "AAA000", |u, kv| {
            u.inner[u.idx] = Some(kv);
            u.idx += 1;
        });
        assert_eq!(
            u,
            T {
                inner: [
                    Some(KeyValueToken::MaybeKey("AAA")),
                    Some(KeyValueToken::MaybeValue("000"))
                ],
                idx: 2
            }
        );
    }
}
