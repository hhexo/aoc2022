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

fn aabb_intersect(a1: i32, a2: i32, b1: i32, b2: i32) -> bool {
    return a1 >= b1 && a1 <= b2 ||
           b1 >= a1 && b1 <= a2
}

fn aabb_contained(a1: i32, a2: i32, b1: i32, b2: i32) -> bool {
    return a1 >= b1 && a1 <= b2 && a2 >= b1 && a2 <= b2 ||
           b1 >= a1 && b1 <= a2 && b2 >= a1 && b2 <= a2
}

fn main() -> Result<(), AoCError> {
    let input = InputText::new(4)?;

    // Read lines to a vector where each element is a vector of 4 numbers:
    // a1 a2 b1 b2
    let four_number_vectors = input.lines()?
    .map(|line| line.unwrap_or_default())
    .map(|actual_line| {
        actual_line.split(",").fold(Vec::new(), |mut acc, pair| {
            let nums: Vec<i32> = pair.split("-").map(|num| i32::from_str_radix(num, 10).unwrap_or(-1)).collect();
            if nums.len() >= 2 {
                acc.push(nums[0]);
                acc.push(nums[1]);
            }
            acc
        })
    }).collect::<Vec<Vec<i32>>>();

    let part1 = four_number_vectors.iter().fold(0, |acc, tuple| {
        acc + i32::from(tuple.len() == 4 && aabb_contained(tuple[0], tuple[1], tuple[2], tuple[3]))
    });
    let part2 = four_number_vectors.iter().fold(0, |acc, tuple| {
        acc + i32::from(tuple.len() == 4 && aabb_intersect(tuple[0], tuple[1], tuple[2], tuple[3]))
    });

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
