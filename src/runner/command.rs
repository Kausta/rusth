/*
 * Project: rusth
 * File: runner/command.rs
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

use std::borrow::Cow;

#[derive(Debug)]
pub struct Command<'a> {
    pub args: Vec<Cow<'a, str>>
}

impl<'a> Command<'a> {
    pub fn new(args: Vec<Cow<'a, str>>) -> Command<'a> {
        return Command {
            args
        };
    }

    pub fn empty(&self) -> bool {
        return self.args.len() == 0;
    }

    pub fn command(&self) -> &str {
        return self.args[0].as_ref();
    }
}


pub type Method = fn(&Command) -> Option<i32>;