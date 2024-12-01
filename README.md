# Advent of Code 2024 - Rust Solutions

This repository contains my solutions to the Advent of Code 2024 challenges, implemented in Rust. Each day, I will tackle a new puzzle and share the clean and efficient code that solves it, following best practices and making use of Rust’s powerful features.

## Table of Contents

- [About](#about)
- [How to Run](#how-to-run)
- [Structure](#structure)

## About

Advent of Code is an annual event that presents a new coding challenge every day from December 1st to December 25th. The goal is to solve these challenges using the programming language of your choice. In this repository, I’m solving the puzzles using Rust, a systems programming language known for its performance and safety.

You can follow along as I work through the daily puzzles, or use my solutions as inspiration to solve the challenges yourself.

## How to Run

To run the solutions for each day, you need to have Rust installed. You can install it from the official Rust website.

1.	Clone this repository to your local machine:
   
```
git clone https://github.com/andrea-deluca/advent-of-code-2024.git
cd advent-of-code-2024
```

2.	Each day’s solution is in its own folder. Navigate to the folder for the day you’re interested in:

```
cd day-1-historian-hysteria -- resources/puzzle # for example
```

3.	Run the solution for that day:

```
cargo run -- resources/puzzle
```

**Note**: Some solutions may require you to pass args through the terminal.

**Note**: You can also run a solution from root project folder using the package name. For example:

```
cargo run --package day-1-historian-hysteria -- day-1-historian-hysteria/resources/puzzle
```

## Structure

Each day’s solution is located in its respective folder within the repository. The structure looks like this:

```
advent-of-code-2024/
├── day-1/
│   ├── resources/    # input files for that specific day are placed here
│   │   └── example
│   │   └── puzzle
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
├── day-2/
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
├── day-3/
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
└── README.md
```

Each folder has its own Cargo.toml file, so you can build and run the individual day solutions independently.
