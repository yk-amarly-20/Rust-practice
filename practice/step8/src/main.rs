fn main() {
    // 参照
    let s1 = String::from("hello");

    let length = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, length);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
