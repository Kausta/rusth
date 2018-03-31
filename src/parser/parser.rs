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

    fn collect_single<I>(iter: &mut I) -> (Option<Cow<'a, str>>, Option<Token<'a>>)
        where I: Iterator<Item=Token<'a>> {
        match iter.next() {
            Some(Token::Str(token)) => (Some(token.content), None),
            Some(token) => (None, Some(token)),
            None => (None, None)
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

    pub fn collect(self) -> Result<Runnable<'a>, String> {
        let iter = &mut self.tokens.into_iter();
        let res = Parser::collect_items(iter);
        let (items, token) = res;
        match token {
            Some(Token::Pipe) => {
                let res_next = Parser::collect_items(iter);
                let (items_next, token_next) = res_next;
                match token_next {
                    Some(token) => Err(format!("Unexpected token {:?}, commands more than 2 not implemented, will ignore the rest", token)),
                    None => Ok(Runnable::Pipe(Pipe::new(Command::new(items), Command::new(items_next))))
                }
            }
            Some(Token::Insert) => {
                match Parser::collect_single(iter) {
                    (Some(file_name), None) => Ok(Runnable::Insert(Insert::new(Command::new(items), file_name))),
                    (None, Some(token)) => Err(format!("Unexpected token {:?}", token)),
                    (None, None) => Err("Expected token as filename!".to_string()),
                    (Some(_), Some(_)) => unreachable!()
                }
            }
            Some(Token::Append) => {
                match Parser::collect_single(iter) {
                    (Some(file_name), None) => Ok(Runnable::Append(Append::new(Command::new(items), file_name))),
                    (None, Some(token)) => Err(format!("Unexpected token {:?}", token)),
                    (None, None) => Err("Expected token as filename!".to_string()),
                    (Some(_), Some(_)) => unreachable!()
                }
            }
            Some(Token::From) => {
                Ok(Runnable::From)
            }
            Some(_) => {
                Err("Either unimplemented token type, or collect_items not working".to_string())
            }
            None => {
                Ok(Runnable::Cmd(Command::new(items)))
            }
        }
    }
}