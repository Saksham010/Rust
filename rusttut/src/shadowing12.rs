fn main(){
    
    //Shadowing refers to redefining previous variable
    

    let number:i32 = 10;
    println!("Unshadowed: {}",number);

    let number:f64 = 10.2;
    println!("Shadowed: {}",number);


    let myvalue = {
        
        let data = 10;
        let data = "saksham";
        data 
    };

    println!("DATA: {}",myvalue);

}
