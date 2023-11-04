fn main(){
    let mut number = 8;
    let numref = &mut number;//Mutable reference
    *numref += 1; //Dereference mutable reference
    let normalref = &number; //Normal reference
                             //
    //Rules: 
    //1. If only immutable reference then many can exist
    //2. immutable refernece followed by mutable not allowed
    //3. Mutable reference followed by immutable allowed
    

    //To overcome 2 we can do shadowing
    let country = "Nepal";
    let imcountry = &country;
    let country = 8; //Shadowing
    println!("Ref: {}, country: {}",imcountry,country);








}
