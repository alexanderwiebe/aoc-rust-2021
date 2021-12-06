/*--------------------------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
 *-------------------------------------------------------------------------------------------------------------*/
 use std::str::FromStr;

fn main() {
    let name = "VS Code Remote - Containers";
    println!("Hello, {}!", name);
    println!("{:?}", std::env::current_dir());
    day_1();
    
}

fn read_all<T: FromStr>(file_name: &str) -> Vec<Result<T, <T as FromStr>::Err>> {
    std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse())
        .collect()
}

fn day_1() {
    let input = read_all::<i32>("day1input.txt");
    println!("{:?}", input[0]);
}