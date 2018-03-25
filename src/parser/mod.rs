/*
 * Project: rusth
 * File: parser/mod.rs
 * Copyright 2018 Caner Korkmaz (Kausta) <info@canerkorkmaz.com>
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
use std;
use std::str::Chars;
use std::borrow::Cow;

type ParserIterPeekable<'a> = std::iter::Peekable<Chars<'a>>;

struct Token<'a> {
    pub content: Cow<'a, str>,
}

type OptionalResult<T, E> = Result<Option<T>, E>;

struct Parser<'a> {
    pub line: &'a str,
    pub iter: ParserIterPeekable<'a>,
    pub tokens: Vec<Token<'a>>,
    loc: usize,
}

impl<'a> Parser<'a> {
    pub fn new(line: &'a str) -> Parser {
        return Parser {
            line,
            iter: line.chars().peekable(),
            tokens: Vec::new(),
            loc: 0,
        };
    }

    pub fn parse_tokens<'err>(&mut self) -> Result<(), &'err str> {
        loop {
            let token = self.next_token();
            match token {
                Ok(Some(token)) => self.tokens.push(token),
                Ok(None) => break,
                Err(e) => return Err(e)
            }
        }
        return Ok(());
    }

    fn next_token<'err>(&mut self) -> OptionalResult<Token<'a>, &'err str> {
        self.consume_ws();
        if self.finished() {
            return Ok(None);
        }
        match self.iter.peek() {
            Some(&c) => self.next_token_with(c),
            // Shouldn't happen in reality thanks to self.finished() check above
            None => Err("Unknown Error")
        }
    }

    fn next_token_with<'err>(&mut self, c: char) -> OptionalResult<Token<'a>, &'err str> {
        match c {
            '\"' => {
                self.next();
                self.next_double_quote()
            },
            _ => {
                let current_loc = self.loc;
                let new_loc = self.take_while(|c| !c.is_whitespace());
                Ok(Some(Token {
                    content: self.line[current_loc..new_loc].into()
                }))
            }
        }
    }

    fn next_double_quote<'err>(&mut self) -> OptionalResult<Token<'a>, &'err str> {
        let mut val = String::new();
        'quot: while let Some(c) = self.next() {
            match c {
                '\\' => {
                    if let Some(c) = self.next() {
                        match c {
                            '"' | '\\' => {
                                val.push(c);
                            },
                            'n' => {
                                val.push('\n');
                            },
                            _ => {
                                val.push('\\');
                                val.push(c);
                            }
                        }
                    } else {
                        break 'quot;
                    }
                },
                '\"' => {
                    return Ok(Some( Token{
                        content: val.into()
                    }));
                },
                _ => {
                    val.push(c);
                }
            }
        }
        return Err("Cannot find closing \"");
    }

    fn consume_ws(&mut self) -> usize {
        self.take_while(|c| c.is_whitespace())
    }

    fn take_while(&mut self, pred: fn(char) -> bool) -> usize {
        while let Some(&c) = self.peek() {
            if !pred(c) {
                break;
            }
            self.next();
        }
        return self.loc;
    }

    fn peek(&mut self) -> Option<&char> {
        self.iter.peek()
    }
    fn next(&mut self) -> Option<char> {
        self.loc += 1;
        self.iter.next()
    }

    fn finished(&self) -> bool {
        self.loc == self.line.len()
    }

    pub fn collect(self) -> Vec<Cow<'a, str>> {
        let items = &mut Vec::new();
        for token in self.tokens {
            items.push(token.content);
        }
        return items.to_vec();
    }
}

pub fn parse<'a>(line: &'a str) -> Result<Vec<Cow<'a, str>>, String> {
    let mut parser = Parser::new(line);
    if let Err(e) = parser.parse_tokens() {
        return Err(e.into());
    };
    return Ok(parser.collect());
    // Temporary until actual parsing
    // Just split from whitespaces for now
    //let split = line.split_whitespace();
    //return Ok(split.collect());
}