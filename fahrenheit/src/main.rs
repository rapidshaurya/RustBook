use std::io;
fn main() {
    println!("Enter fahrenheit");
    let mut f=String::new();
    io::stdin().read_line(&mut f).expect("Invalid");
    let f:f64=f.trim().parse().expect("Not a number");
   
    let ans=f2c(f);
    println!("Celsius is {}",ans);

}

fn f2c(f:f64)->f64{
    (f-32.0)*(5.0/9.0)
}
