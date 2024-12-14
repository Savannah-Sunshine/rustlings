#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// * we can provide a reference to the String value. A reference is like a pointer in that 
// * it’s an address we can follow to access the data stored at that address; that data is owned by some other variable. 


// Shouldn't take ownership
fn get_char(data: &String) -> char {
    // * this line creates a reference to the last character in the string
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}
