use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){

    println!("Hello World");
    //Println! is a macro not a function

    //By default rust is immutable
    let var = 5;
    let mut var= 10;
    

    let randomNum = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();

    loop{



        io::stdin().read_line(&mut guess).expect("Error while reading a new line");
    
        let guess: u32 = guess.trim().parse().expect("Please type a number");

        println!("The number you guessed is: {guess}");
    
        match guess.cmp(&randomNum){
    
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("You won");
                break;
            }
        }
    }
}
