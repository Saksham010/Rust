fn main(){

    //String are of two types
    //1. &str => & points to str whose size is not known by the compiler
    //compiler knows the size of pointer (Data is not owned)
    //
    //2. String => String is a pointer that points to heap where data is variable size (Owned)
    let myname = "Hello"; //&s
    let othername = String::from("Hello"); //String
                                           //
    //Conversion from &str to String
    //1. String::from()
    //2. to_string()
    let mystr = "Hello".to_string();

    //3. into()
    let mydata:String = "Hello".into();

    //4. format! macro
    let data = format!("Hello");
        



}
