
// Opgave 1: Introduktion til Referencer
fn opgave_1() {
    let number = 10;
    let ref_to_number = &number;
    println!("Opgave 1 - Tallet via reference: {}", ref_to_number);
    println!("Opgave 1 - Tallet via dereference: {}", *ref_to_number);
}

// Opgave 2: Muterbare Referencer
fn opgave_2() {
    let mut number = 5;
    let ref_to_number = &mut number;
    *ref_to_number += 10;
    println!("Opgave 2 - Ændret tal: {}", number);
}

// Opgave 3: Fejlhåndtering med Result
use std::fs::File;

fn opgave_3() {
    let file_result = File::open("data.txt");

    match file_result {
        Ok(file) => println!("Opgave 3 - Filen blev åbnet succesfuldt: {:?}", file),
        Err(error) => println!("Opgave 3 - Fejl ved åbning af fil: {:?}", error),
    }
}

// Opgave 4: Fejlhåndtering med Option
fn find_user(users: Vec<&str>, user_to_find: &str) -> Option<usize> {
    users.iter().position(|&u| u == user_to_find)
}

fn opgave_4() {
    let users = vec!["Alice", "Bob", "Charlie"];
    match find_user(users, "Bob") {
        Some(index) => println!("Opgave 4 - Brugeren blev fundet på indeks: {}", index),
        None => println!("Opgave 4 - Brugeren blev ikke fundet"),
    }
}

// Opgave 5: Kombinering af Result og Option
use std::io::{self, BufRead, BufReader};

fn read_first_line(filename: &str) -> Result<Option<String>, io::Error> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let bytes = reader.read_line(&mut line)?;
    if bytes == 0 {
        Ok(None)
    } else {
        Ok(Some(line))
    }
}

fn opgave_5() {
    match read_first_line("test.txt") {
        Ok(Some(line)) => println!("Opgave 5 - Første linje: {}", line),
        Ok(None) => println!("Opgave 5 - Filen er tom"),
        Err(e) => println!("Opgave 5 - Fejl ved læsning af fil: {}", e),
    }
}

// Opgave 6: Reference-problemer og ejerskab (dette vil ikke kompilere)
// fn dangle() -> &String {
//     let s = String::from("Hej");
//     &s // Fejl! 's' vil ikke eksistere, når funktionen slutter
// }

fn main() {
    // Kør alle opgaver
    opgave_1();
    opgave_2();
    opgave_3();
    opgave_4();
    opgave_5();
}
