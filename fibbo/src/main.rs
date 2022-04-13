use std::io;
fn main() {
    println!("Enter number");
    let mut num=String::new();
    io::stdin().read_line(&mut num).expect("Invalid");
    let num:i32=num.trim().parse().expect("Not a number");
    for int in 0..num+1 {
        print! ( "{}  ",fib(int));
    }
}
fn fib (n: i32) -> i32 {
    if n <= 0 {
          return 0;
    } else if n== 1{
          return 1;
} else {
    return fib (n-1)  + fib(n-2);
 }
}

