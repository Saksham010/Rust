fn sum(num1:i32, num2:i32)->i32{

    //num1+num2  //Implicit return
               //If num1+num2; then error as ; returns() so we must use return when using ;
    return num1+num2;
}    

fn main(){
    
    println!("The sum is: {}",sum(10,20));

    //Block
    let mynum = {

        let secondnum = 30;
            secondnum //returning from the block
    };
    println!("Mynumber: {}", mynum); 
}
