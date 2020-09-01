fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    // println!("The value of y is {}", y);
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);

    let a = 2;
    let b = plus_one(a);

    println!("The value of a is {}", a);
    println!("The value of b is {}", b);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
