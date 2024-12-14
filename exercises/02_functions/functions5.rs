// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i32 {
    // Implicitly returns result bc it doesn't have a semicolon `;` at the end of the line.
    num * num
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}
