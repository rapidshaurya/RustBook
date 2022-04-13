/*

fn main() {
    let dice_roll = 3;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

  
}
fn add_fancy_hat() { println!("add_fancy_hat")}
fn remove_fancy_hat() {println!("remove_fancy_hat")}


*/

fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

