// Copyright 2022 Dario Domizioli
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fs::File;
use std::io::{BufReader,BufRead,Lines};

#[derive(Debug)]
pub enum AoCError {
    IOError(std::io::Error),
    GenericError(String),
}

impl std::convert::From<std::io::Error> for AoCError {
    fn from(e: std::io::Error) -> Self {
        AoCError::IOError(e)
    }
}

pub struct InputText {
    path: String,
}

impl InputText {
    pub fn new(problem: i32) -> Result<InputText, AoCError> {
        let path = format!("input/{:02}/input.txt", problem);
        Ok(InputText{ path: path })
    }

    pub fn lines(&self) -> Result<Lines<BufReader<File>>, AoCError> {
        Ok(BufReader::new(File::open(&self.path)?).lines())
    }
}
