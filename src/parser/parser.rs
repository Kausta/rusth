/*
 * Project: rusth
 * File: parser/parser.rs
 * Copyright 2018 Caner Korkmaz (Kausta) [info@canerkorkmaz.com]
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
*/

use super::lexer::Token;
use std::borrow::Cow;

use runner::command::*;

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Parser {
        Parser {
            tokens
        }
    }

    fn collect_items<I>(iter: &mut I) -> (Vec<Cow<'a, str>>, Option<Token<'a>>)
        where I: Iterator<Item=Token<'a>> {
        let mut items = Vec::new();
        for token in iter {
            match token {
                Token::Str(token) => items.push(token.content),
                _ => return (items, Some(token))
            }
        }
        (items, None)
    }

    pub fn collect(self) -> Runnable<'a> {
        let iter = &mut self.tokens.into_iter();
        let res = Parser::collect_items(iter);
        let (items, token) = res;
        match token {
            Some(token) => {
                match token {
                    Token::Pipe => {
                        let res_next = Parser::collect_items(iter);
                        let (items_next, token_next) = res_next;
                        if let Some(token) = token_next {
                            eprintln!("Unexpected token {:?}, commands more than 2 not implemented", token)
                        }
                        Runnable::Pipe(Pipe::new(Command::new(items), Command::new(items_next)))
                    },
                    _ => {
                        panic!("Either unimplemented token type, or collect_items not working")
                    }
                }
            }
            None => {
                Runnable::Cmd(Command::new(items))
            }
        }
    }
}