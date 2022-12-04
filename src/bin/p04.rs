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

use std::collections::HashSet;

fn main() -> Result<(), AoCError> {
    let input = InputText::new(4)?;

    let pairs_of_sets = input.lines()?
    .map(|line| line.unwrap_or_default())
    .map(|actual_line| {
        actual_line.split(",").map(|pair| {
            let nums: Vec<i32> = pair.split("-").map(|num| i32::from_str_radix(num, 10).unwrap_or(-1)).collect();
            if nums.len() >= 2 {
                if nums[1] >= nums[0] {
                    (nums[0]..nums[1]+1).collect::<HashSet<i32>>()
                } else {
                    HashSet::new()
                }
            } else {
                HashSet::new()
            }
        }).collect::<Vec<HashSet<i32>>>()
    }).collect::<Vec<Vec<HashSet<i32>>>>();

    let part1 = pairs_of_sets.iter().fold(0, |acc, sets| {
        acc + i32::from(sets.len() >= 2 && {
            let intersection_size = sets[0].intersection(&sets[1]).count();
            intersection_size == sets[0].len() || intersection_size == sets[1].len()
        })
    });
    let part2 = pairs_of_sets.iter().fold(0, |acc, sets| {
        acc + i32::from(sets.len() >= 2 && sets[0].intersection(&sets[1]).next().is_some())
    });

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
