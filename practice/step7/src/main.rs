fn main() {
    // 所有権
    let s = String::from("hello");
    let x = 5;

    takes_ownership(s);
    // ここではsはもう使えない

    make_copy(x);
    // ここではxは使える
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
}
