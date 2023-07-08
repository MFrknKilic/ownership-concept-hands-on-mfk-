fn concatenate_strings(string_first_one: &str, string_second_one: &str) -> String {
    let mut result = String::new();
    result.push_str(string_first_one);
    result.push_str(string_second_one);
    result
}

fn main() {
    let string_first_one = "Furkan ";
    let string_second_one = "try to learn Rust";

    let concatenated_string = concatenate_strings(string_first_one,
        string_second_one);

    println!("{}", concatenated_string);
}