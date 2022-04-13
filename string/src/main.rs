
fn main() {
    /*
    let hello = String::from("السلام عليكم");
    println!("{}",hello);
    let hello = String::from("Dobrý den");
    println!("{}",hello);
    let hello = String::from("Hello");
    println!("{}",hello);
    let hello = String::from("שָׁלוֹם");
    println!("{}",hello);
    let hello = String::from("नमस्ते");
    println!("{}",hello);
    let hello = String::from("こんにちは");
    println!("{}",hello);
    let hello = String::from("안녕하세요");
     println!("{}",hello);
    let hello = String::from("你好");
    println!("{}",hello);
    let hello = String::from("Olá");
    println!("{}",hello);
    let hello = String::from("Здравствуйте");
    println!("{}",hello);
    let hello = String::from("Hola");
    println!("{}",hello);
    */
    
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
        println!("{}",s3)
        

}