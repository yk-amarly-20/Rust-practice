fn main() {
    // 借用
    let mut s = String::from("hello");

    // println!("{}", change(&s))
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world")
}
