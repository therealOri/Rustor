use std::fs::OpenOptions;
use std::io;
use std::io::{Read, Seek, Write};
use std::process::Command;


fn words_sort() -> io::Result<()> {
    let mut contents = String::new();

    let mut file = match OpenOptions::new().read(true).open("words.txt") {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {}", e);
            return Err(e);
        }
    };

    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => {
            println!("Error reading file: {}", e);
            return Err(e);
        }
    };

    let mut lines = contents.lines().collect::<Vec<_>>();
    lines.sort();
    lines.dedup();
    contents = lines.join("\n");

    let mut file = OpenOptions::new().write(true).create(true).open("words_sorted.txt")?;
    file.write_all(contents.as_bytes())?;

    std::fs::rename("words_sorted.txt", "words.txt")?;

    Ok(())
}


fn main() -> io::Result<()> {
    //No idea why this doesn't work..
    Command::new("clear").output().expect("Failed to clear the terminal");
    println!("Enter some words separated by commas: ");

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input)?;

    if user_input.trim().is_empty() {
        println!("user_input can't be an empty string.");
        return Ok(());
    }

    let words = user_input.trim().split(',');
    let word_list: Vec<String> = words.clone()
        .map(|words| words.trim().to_lowercase())
        .filter(|words| !words.is_empty())
        .collect();

    //No idea why this doesn't work..
    Command::new("clear").output().expect("Failed to clear the terminal");
    println!("Checking words.txt for: {:?}", word_list);
    let mut contents = String::new();
    let mut file = match OpenOptions::new().read(true).write(true).open("words.txt") {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening words.txt: {}", e);
            return Err(e);
        }
    };

    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => {
            println!("Error reading words.txt: {}", e);
            return Err(e);
        }
    };

    let lines: Vec<&str> = contents.lines().collect();
    let mut new_lines = Vec::new();
    let mut found_words = Vec::new();

    for word in word_list {
        let word = word.as_str();
        if word.is_empty() {
            continue;
        }

        if !lines.contains(&word) {
            println!("Adding word '{}' to words.txt...", word);
            new_lines.push(word.to_owned());
        } else {
            found_words.push(word.to_owned());
        }
    }

    // Sort the new_lines in alphabetical order. Just in case contents in file aren't already.'
    new_lines.sort();

    // Concatenate the new_lines into a single string with newline characters
    let mut new_contents = new_lines.join("\n");
    new_contents.push('\n');

    // Append the new_contents to the end of the file
    match file.seek(io::SeekFrom::End(0)) {
        Ok(_) => (),
        Err(e) => {
            println!("Error seeking the the end of words.txt: {}", e);
            return Err(e);
        }
    };

    match file.write_all(new_contents.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            println!("Error writing to words.txt: {}", e);
            return Err(e);
        }
    };

    println!("Words already in words.txt: {:?}", found_words);

    words_sort()?;

    Ok(())
}


