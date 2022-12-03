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

use aoc2022::{AoCError, InputText};

fn main() -> Result<(), AoCError> {
    let input = InputText::new(1)?;

    let mut sums = input.lines()?.fold(vec![0,], |mut sums, line| {
        match i32::from_str_radix(&line.unwrap_or_default(), 10) {
            Ok(i) => { *sums.last_mut().unwrap() += i; },
            _ => { sums.push(0); }
        };
        sums
    });
    sums.sort();

    println!("Part 1: {}", sums.last().unwrap_or(&0));
    println!("Part 2: {}", sums.iter().rev().take(3).fold(0, |acc, i| acc+i));

    Ok(())
}
