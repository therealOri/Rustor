use std::fs::OpenOptions;
use std::io;
use std::io::{Read, Seek, Write};

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
    // Add words here that you want to add to the list.
    // It will let you know if the words already exist.
    // If they don't, it will add the word(s) to the .txt file and keep everything alphabetical.
    // Try to keep words lowercase.
    let words = vec!["foo", "bar", "baz"];
    if words.is_empty() || (words.len() == 1 && words[0].is_empty()) {
        return Err(io::Error::new(io::ErrorKind::Other, "Words list is empty.."));
    }

    let mut contents = String::new();

    println!("Checking words.txt for listed words...");

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

    for word in words {
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


