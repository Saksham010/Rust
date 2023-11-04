fn copy(country_name:String){
    //Country_name owns the passed value
    println!("{}",country_name);
}
fn refcopy(country_name: &String){
    println!("{}",country_name);
}

fn mutrefcopy(country_name: &mut String){
    country_name.push_str("Harty");
    println!("{}",country_name);

}

fn mutcopy(mut country_name: String){
    country_name.push_str("changed");
    println!("{}",country_name);
}

fn main() {
        
    //Country owns the value
    let mut country = String::from("Austria");
    
    //copy(country); //Runs
    //copy(country); //Does not run because:
                   //In first call the country value is owned by country_name
                   //country_name is destroyed after func end so nothing exits

    refcopy(&country);
    mutrefcopy(&mut country);
    mutcopy(country);


}
