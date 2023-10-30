[![Clippy](https://github.com/tommymmcguire/Individproj2/actions/workflows/lint.yml/badge.svg)](https://github.com/tommymmcguire/Individproj2/actions/workflows/lint.yml)
[![Tests](https://github.com/tommymmcguire/Individproj2/actions/workflows/tests.yml/badge.svg)](https://github.com/tommymmcguire/Individproj2/actions/workflows/tests.yml)

# IDS 706 Individual Project 2

---
**Walk Through Youtube Video**
[YouTube](https://youtu.be/Pu5Zv3aWvwE)

---

## Purpose
This project serves as a practical example of how to manipulate an SQLite database and perform CRUD operations using Rust. In specific, this project extracts information from a csv file in GitHub, connects to a database, and creates and manipulates a table within that database. 
While developing this project, I used GitHub copilot to develop and polish my code. This enhanced the efficiency and effectiveness of the overall project. 
The combination of Rust and Copilot makes this project computationally efficient, robust, and reduces programming time. 

## Details
The dependencies used in this project include reqwest, csv, rusqlite, and clap. The dependencies can be found in Cargo.toml.

To use this project you will first need to build it using `cargo build`, then run `cargo run` to execute.

## Output

The first function that executes is 'query_top5' which prints the top 5 rated movies.

<img width="695" alt="Screenshot 2023-10-29 at 3 23 01 PM" src="https://github.com/tommymmcguire/Individproj2/assets/141086024/ac84d789-0bbb-4d9d-9e05-eee77502ccf8">

The second function to execute is 'query_best_genre' which prints the top 3 movies based on the genre. The genre can be changed in the main function. 

<img width="687" alt="Screenshot 2023-10-29 at 3 23 19 PM" src="https://github.com/tommymmcguire/Individproj2/assets/141086024/e18c71d4-7f7f-4d7c-8de2-3ba0597da26e">

Next, the program executes CRUD operations. This is an example of a row created and read. You can also update the row by including "update_record('rank you want to update', 'the new rating you want to update to')" in the main function after "read_record". To delete the row include "delete_record('the rank of the row you want to delete')".

<img width="694" alt="Screenshot 2023-10-29 at 3 23 31 PM" src="https://github.com/tommymmcguire/Individproj2/assets/141086024/b265d190-0778-4a04-80f0-7d485606195e">


