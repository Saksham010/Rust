fn main(){

    // i32 => 32 bit default for number
    // u8 => 256 bit which holds similar to char so can be typecasted to char easily

    let number:i32 = 100;
    println!("The value is: {}", number as u8 as char);

    //String
    let name = "Saksham";
    println!("The size in bytes:{} ",name.len());
    println!("The number of char is: {}",name.chars().count());

}
