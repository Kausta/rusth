/*
 * Project: rusth
 * File: util/prompt.rs
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

use std::env::current_dir;
use std::fmt::Write;

#[allow(unused_imports)]
use ansi_term;

static PROMPT_ANSI: &'static str = "\x1b[1;32m>>\x1b[0m ";
static PROMPT_NORMAL: &'static str = ">> ";

pub struct Prompt<'a> {
    pub prompt_base: &'a str,
    pub have_ansi: bool,
    return_code: Option<i32>,
}

impl<'a> Prompt<'a> {
    pub fn new() -> Prompt<'a> {
        // TODO: Config options etc.
        let have_ansi = Prompt::init_ansi();
        let prompt = if have_ansi { PROMPT_ANSI } else { PROMPT_NORMAL };
        Prompt {
            prompt_base: prompt,
            have_ansi,
            return_code: None,
        }
    }

    /// Tries to initialize ansi support on Windows and return accordingly
    /// Always returns true on linux
    #[cfg(windows)]
    fn init_ansi() -> bool {
        ansi_term::enable_ansi_support().is_ok()
    }
    #[cfg(not(windows))]
    fn init_ansi() -> bool {
        true
    }

    pub fn set_return_code(&mut self, code: Option<i32>) {
        self.return_code = code;
    }

    pub fn make_prompt(&mut self) -> String {
        let cd = current_dir();
        let mut prompt = String::new();
        if let Some(code) = self.return_code {
            write!(&mut prompt, "({}) ", code).expect("Cannot build prompt");
        }
        if let Ok(dir) = cd {
            write!(&mut prompt, "{} ", dir.into_os_string().into_string().unwrap()).expect("Cannot get directory string");
        };
        prompt.push_str(self.prompt_base);
        self.reset_state();
        prompt
    }

    fn reset_state(&mut self) {
        self.return_code = None;
    }
}