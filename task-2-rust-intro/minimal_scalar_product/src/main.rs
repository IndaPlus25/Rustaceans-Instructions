/***
 * Example solution to Kattis problem Minimal Scalar Product.
 * 
 * Course litterature.
 * Course: DD1337 Programming
 * KTH Royal Institute of Technology
 * 
 * See: https://open.kattis.com/problems/minimumscalar
 * 
 * Author: Viola Söderlund <violaso@kth.se>
 * License: MIT
 * Latest change: 2025-09-04
 */

use std::io;
use std::io::prelude::*;

fn main() {
    let input = io::stdin();

    let mut lines = input
        .lock()                                                 
        .lines()
        .map(|_line| _line.ok().unwrap());

    let number_of_cases = lines 
        .next().unwrap()
        .parse::<u32>().unwrap();

    eprintln!("\nNumber of cases: {}\n", number_of_cases);

    for _case in 0..number_of_cases {

        let length = lines
            .next().unwrap()
            .parse::<usize>().unwrap();

        let mut vector_1 = lines
            .next().unwrap()
            .split(" ") 
            .map(|component| component.parse::<i64>().unwrap()) 
            .collect::<Vec<i64>>();
        vector_1.sort();

        let mut vector_2 = lines
            .next().unwrap()
            .split(" ")
            .map(|component| component.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        vector_2.sort();
        
        eprintln!("\nVector 1: {:?}", vector_1);
        eprintln!("Vector 2: {:?}\n", vector_2);

        // compute scalar product
        let mut scalar_product: i64 = 0;
        for _index in 0..length {
            scalar_product += vector_1[_index] * vector_2[length - 1 - _index];
        }

        eprintln!("Scalar product: {}\n", scalar_product);

        println!("Case #{}: {}", _case + 1, scalar_product);
    }
}