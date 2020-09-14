pub mod RpnCalculator {

    pub struct RpnCalculator(bool);

    impl RpnCalculator {
        pub fn init(verbose: bool) -> Self {
            Self(verbose)
        }

        pub fn eval(&self, formula: &str) -> i32 {
            let mut tokens = formula.split_ascii_whitespace().rev().collect::<Vec<_>>();
            self.eval_inner(&mut tokens)
        }

        pub fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32 {
            let mut stack = Vec::new();

            while let Some(token) = tokens.pop() {
                if let Ok(x) = token.parse::<i32>() {
                    stack.push(x);
                } else {
                    let y = stack.pop().expect("invalid syntax");
                    let x = stack.pop().expect("invalid systax");
                    let res = match token {
                        "+" => x + y,
                        "-" => x - y,
                        "*" => x * y,
                        "/" => x / y,
                        "%" => x % y,
                        _ => panic!("invalid token"),
                    };

                    stack.push(res);
                }

                if self.0 {
                    println!("{:?} {:?}", tokens, stack);
                }
            }


            if stack.len() == 1 {
                stack[0]
            } else {
                panic!("invalid syntax")
            }
        }
    }
}
