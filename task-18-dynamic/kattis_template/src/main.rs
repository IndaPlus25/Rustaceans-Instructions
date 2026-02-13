/***
 * Template to a Kattis solution.
 * 
 * Course litterature.
 * Course: DD1337 Programming
 * KTH Royal Institute of Technology
 * 
 * See: https://open.kattis.com/help/rust
 * 
 * Author: Viola Söderlund <violaso@kth.se>
 * License: MIT
 * Latest change: 2025-09-04
 */

use std::io;
use std::io::prelude::*;

/// Kattis calls main function to run your solution.
fn main() {
    // Get standard input stream.
    let input = io::stdin();

    // Get input lines as strings.
    // See: https://doc.rust-lang.org/std/io/trait.BufRead.html
    let mut lines = input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap())
        .collect::<Vec<String>>();

    /* add code here ... */

    // Debug print statement.
    eprintln!("Kattis print this comment to an irrelevant stream!");
    
    // Print result to Kattis.
    println!("Print to standard output.");
}