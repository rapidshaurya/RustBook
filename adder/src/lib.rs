//program 1

// pub struct Guess {
//     value: i32,
// }

// // --snip--
// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value > 1 {
//             panic!(
//                 "Guess value must be greater than or equal to 1, got {}.",
//                 value
//             );
//         } else if value < 100 {
//             panic!(
//                 "Guess value must be less than or equal to 100, got {}.",
//                 value
//             );
//         }

//         Guess { value }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     #[should_panic(expected = "Guess value must be less than or equal to 100")]
//     fn greater_than_100() {
//         Guess::new(200);
//     }
// }




// Program 2



// pub struct Guess {
//     value: i32,
// }

// // --snip--
// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value > 1 {
//             panic!(
//                 "Guess value must be greater than or equal to 1, got {}.",
//                 value
//             );
//         } else if value < 100 {
//             panic!(
//                 "Guess value must be less than or equal to 100, got {}.",
//                 value
//             );
//         }

//         Guess { value }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     #[should_panic(expected = "Guess value must be less than or equal to 100")]
//     fn greater_than_100() {
//         Guess::new(200);
//     }
// }




// pub struct num {
//     value: i32,
// }

// impl num {
//     pub fn check(value:i32){
//         if value >= 23 && value < 24 {
//             println!("No problem");
//         }
//         else {
//             panic!("Value must be in between 23 & 24");
//         }
//     }
// }

// #[cfg(test)]

// mod test{
//     use super::*;

//     #[test]
//      fn test1() {
//          num::check(23);
//      }
//      #[test]
//      fn test2(){
//          num::check(24);
//      }

// }



// chapter 11.2

//Program 1

// fn prints_and_returns_10(a: i32) -> i32 {
//     println!("I got the value {}", a);
//     10
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn this_test_will_pass() {
//         let value = prints_and_returns_10(4);
//         assert_eq!(10, value);
//     }

//     #[test]
//     fn this_test_will_fail() {
//         let value = prints_and_returns_10(8);
//         assert_eq!(5, value);
//     }
// }


// Program 2


// pub fn add_two(a: i32) -> i32 {
//     a + 2
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn add_two_and_two() {
//         assert_eq!(4, add_two(2));
//     }

//     #[test]
//     fn add_three_and_two() {
//         assert_eq!(5, add_two(3));
//     }

//     #[test]
//     fn one_hundred() {
//         assert_eq!(102, add_two(100));
//     }

//     #[test]
//     fn not(){
//         assert_ne!(2,add_two(2));   
//     }
// }

// Program 3

// #[test]
// fn it_works() {
//     assert_eq!(2 + 2, 4);
// }

// #[test]
// #[ignore]
// fn expensive_test() {
//     // code that takes an hour to run
// }


// chapter 11.3


pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

