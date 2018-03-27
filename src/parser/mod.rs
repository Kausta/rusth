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
mod lexer;
#[allow(module_inception)]
mod parser;

use self::lexer::Lexer;
use self::parser::Parser;

use runner::command::Runnable;

pub fn parse(line: &str) -> Result<Runnable, String> {
    let mut lexer = Lexer::new(line);
    if let Err(e) = lexer.lex_tokens() {
        return Err(e.into());
    };
    let parser = Parser::new(lexer.collect());
    Ok(parser.collect())
    // Temporary until actual parsing
    // Just split from whitespaces for now
    //let split = line.split_whitespace();
    //return Ok(split.collect());
}