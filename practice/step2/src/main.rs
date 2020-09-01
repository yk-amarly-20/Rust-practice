fn main() {
    // 変数について
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    const MAX_POINTS: u32 = 100_000;

    println!("{}", MAX_POINTS);

    println!("シャドーイング");

    let y = 1;
    let y = y + 2;
    let y = y + 3;

    println!("Final y value is {}", y);

    let space = "   ";
    let space = space.len();
    println!("The space's length is {}", space);
}
