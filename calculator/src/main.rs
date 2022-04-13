use std::env::{args, Args};

fn main() {
    let mut args: Args=args();
    let first =args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let first_number: f32 = first.parse().unwrap();
    let second_number: f32 = second.parse().unwrap();
    let res = operate(operator, first_number, second_number);
    println!("{:?}  ", format!("{} {} {} = {}", first_number, operator, second_number, res));
    

}
fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {

   /* if operator == '+' {
        return first_number+second_number;     // first way
    }
    else if operator == '*' {
        return first_number*second_number;
    }
    else if operator == '-' {
        return first_number-second_number;
    }
    else if operator == '%' {
        return first_number%second_number;
    }
    else {
        return 0.0
    } */
    match operator {
        '+' => first_number+second_number,
        '-' => first_number-second_number,
        '/' => first_number/second_number,
        '*' => first_number*second_number,
         _ => 0.0

    }

}