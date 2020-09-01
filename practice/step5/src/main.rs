fn main() {
    let number = 3;

    if number < 5 {
        println!("The condition is true!");
    } else {
        println!("The condition is false");
    }

    let x = 2;

    if x % 3 == 0 {
        println!("The number is divisible by 3");
    } else if x % 4 == 0 {
        println!("The number is divisible by 4");
    } else {
        println!("No");
    }

    let condition = true;

    let y = if condition {
        5
    } else {
        6
    };

    println!("The value of y is {}", y);

    let a = [1, 2, 3, 4, 5];

    for element in a.iter() {
        println!("the value is {}", element);
    }
}
