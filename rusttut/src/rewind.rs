// Variable bindings
// '''
// fn main(){

//     // let x = 5; // x:i32

//     let (x,y) = (1,2);
//     let a:i32 = 26;
//     let mut b:i32 = 40;

//     println!("This  is b original: {}",b);

//     b = 30;
//     println!("This is b after mutation: ",b);

//     {
//         let b:i32 = 100; //Shadowing
//         let z:i32 = 500;
//         println!("The value of x is: {}, and the value of z is :{} and b is: {}",x,z,b);
//     }

//     println!("Z is undefined here: {}",z);

// }
// '''


//Functions
// '''
// fn main(){
//     let value:i32 = sum(10,20);
//     println!("The sum is: {}",value);

//     //Expressions(returns a value) and declarations(does not return value)
//     let mut a = 100; //Declaration that start with a 

//     a = 40; //Expression

//     let v = (a = 20); // a is 20 and v is () cause a=20; returns empty

//     println!("The value of a is: {}, v is : {:?}",a,v);

//     //Diverging function can be used as any type
//     let anyvalue:i32 = diverges();
//     let anystring:String = diverges();

//     println!("After panic does not run");


//     // Function pointer 
//     let f:fn(i32)->i32 = showfn; //Without type inference
//     let f = showfn; //With type inference
// }

// fn sum(x:i32, y:i32) -> i32{
//     x+y //No semicolon on return

//     //Rust also has return statement but it is considered as a bad practice;
// }

// //Rust is expression based language

// // Diverging function (It does not return anything)
// fn diverges() -> ! {
//     panic!("This function diverges and does not return anything");

// }

// fn showfn(i:i32) -> i32{
//     i
// }
// '''

// Primitive types
// fn main(){

//     // Array
//     let arr:[i32;3] = [1,2,3];
//     // let arr = [0;2]; // [0,0]
//     let size = arr.len();

//     //Slice (Reference to other data structure)
//     let s = &arr[..]; //A slice of all element
//     let s2 = &arr[0..3]; // 0,1,2 index elements


//     //Tuples
//     let tuple:(i32,&str) = (1,"Saksham");
//     let name = tuple.0;
//     //Destructuring tuple
//     let (x,y,z) = (1,2,3);



//     // Loops
//     loop{
//         // Infinite
//         break;
//     }

//     while true {
//         // Infinte
//         break;
//     }

//     //For loop
//     for x in 0..10 {
//         println!("X: {}",x);
//     }

//     // We can use enumerate to keep track of index
//     for (index,value) in (5..10).enumerate() {
//         println!("Index: {} and value: {}",index,value);

//     }

//     //Loops are labelled

//     'outer: for x in 0..3 {
//         'inner: for y in 2..4{
//             if y == 3{
//                 break 'outer;
//             }else if y == 2{
//                 continue 'inner;
//             }
//         }
//     }
// }


// Vectors
// fn main(){

//     let mut v: Vec<i32> = vec![1,2,3,4,5];

//     //Index must be of usize
//     let i:usize = 0;
//     println!("Usize indexed value: {}",v[i]);

//     for i in &v {
//         println!("A reference to {}", i);
//     }
    
//     for i in &mut v {
//         // *i = 20; //Change variable using mutable reference
//         println!("A mutable reference to {}", i);
//     }
    
//     for i in v {
//         println!("Take ownership of the vector and its element {}", i);
//     }


// }

// Ownership

// fn main(){

//     // Vector case 
//     let v = vec![1,2,3]; // v(data pointer) is stored in stack and [1,2,3] is stored in heap
//     let mut v2 = v; //v2 is created on stack that points to data on heap that v is pointing

//     //Removing some value
//     v2.truncate();

//     //But v still thinks there are 3 elements so, rust does not allow v to be accessed 

//     //This is the concept of ownership

//     takeownership(v2);

//     //Now v2 is unaccessible
//     let v3 = vec![1,2,3];
//     v3 = take_give_ownership(v3);
//     //Here v3 is accessible


//     //Normal data types that are stored in stack only implement copy trait, which means variable binding are copied on stack
//     let a = 10;
//     let b = a;
//     //Both a and b are accessible, ownership is not transferred, but copy is implemented



// }

// fn takeownership(v: Vec<i32>) -> !{
//     println!("Ownership taken");
// }

// fn take_give_ownership(v: Vec<i32>) -> Vec<i32>{
//     v
// }


//Borrowing (Take ownership and give back)

// fn main(){

//     let mut x:i32 =20;
//     let y = &x; //Immmuatble reference
//     let a = &x; //No issue

//     println!("{} {} ",x,y); //No issue in immutable cause data cant be changed

//     // For mutable reference
//     let z = &mut x;
//     println!("{} ",z);


// }

//Structs 
//No field level mutability is allowed by reference are allowed
// #[derive(Debug)]
// struct Position{
//     x:i32,
//     y:i32,
//     z:i32
// }

// struct Person<'a>{
//     name:&'a str,
//     age:i32
// }

// struct Color(i32,i32,i32);
// struct Inches(i32);

// fn main(){

//     let mut point = Position{x:0,y:0,z:1};
//     point.x = 20;
//     println!("Point: {:?}",point);

//     let point2 = Position{y:2, ..point};
//     println!("Point2: {:?}",point2);

//     let employee = Person{name:"Hari",age:30};
//     println!("Employee name: {} age: {}",employee.name, employee.age);


//     //Tuple struct

//     let rgb = Color(0,0,0);
//     println!("White: {} {} {} ",rgb.0, rgb.1, rgb.2);

//     let Inches(length) = Inches(10); //Destructring 
//     println!("Length: {}",length);
    
// }

//Enum (Contains optional fields), :: is used to scope access enum variants
// #[derive(Debug)]
// enum Message{
//     Quit,
//     Move{x:i32,y:i32},
//     Write(String),
// }

// fn main(){

//     let m:Message = Message::Write("Hello".to_string());
//     println!("{:?}",m);

//     let m2:Message = foo("World".to_string());
//     println!("{:?}",m2);

//     let q:Message = Message::Quit;
//     println!("Quit: {:?}",q);

// }

// fn foo(x:String) -> Message{
//     Message::Write(x)

// }

//Match (Similar to switch)

// #[derive(Debug)]
// enum Message{
//     Quit,
//     Move{x:i32,y:i32},
//     Write(String),
// }
// fn main(){

//     let x =7;

//     let num = match x {
//         1 => "one",
//         2 => "two",
//         3 => "three",
//         4 => "four",
//         5 => "five",
//         _ => "somethingelse:"
//     };

//     println!("Number is : {}",num);

//     //Useful for matching enum
//     let m = Message::Quit;

//     match m {
//         Message::Quit => println!("Quit"),
//         Message::Move{x,y} => println!("Move : ({}{})",x,y),
//         Message::Write(s) => println!("Write {}",s),
//         // y => println!("Matches nothing: {:?}",y)
//         _ => println!("Matches nothing")
//     }


//     //Movement
//     let tuple: (i32,String) = (5,String::from("Hello"));

//     // let (x,s) = tuple;
//     // println!("{:?}",tuple);

//     //Here tuple also moves because String moves(later on probably heap type)

//     //We can do 
//     let (y,_) = tuple; //Does not move  (can also use .. to disregard values)
//     println!("{:?}",tuple);

//     //ref
//     let a =10;

//     match a {
//         ref c => println!("Got ref of a"),
//     }

//     match a{
//         ret mut mr => println!("Got immmuatble ref")
//     }

    
// }

//Method syntax (Chaining) with impl and struct (Essentially rust version of class)
// struct Circle{
//     x:i32,
//     y:i32,
//     radius:i32
// }
// struct CircleBuilder{
//     x:i32,
//     y:i32,
//     radius:i32
// }

// impl CircleBuilder{
//     //Static method (as it does not take in self) accessed with ::
//     fn new() -> CircleBuilder{
//         CircleBuilder{x:0,y:0,radius:0}
//     }

//     fn x(&mut self,value:i32) -> &mut CircleBuilder{
//         self.x = value;
//         self
//     }

//     fn y(&mut self,value:i32) -> &mut CircleBuilder{
//         self.y = value;
//         self
//     }

//     fn radius(&mut self,value:i32) -> &mut CircleBuilder{
//         self.radius = value;
//         self
//     }
//     fn finalize(&self) ->Circle{
//         Circle{x:self.x,y:self.y,radius:self.radius}
//     }
// }

// impl Circle{

//     fn area(& self) -> i32{
//         3 * self.radius * self.radius
//     }

// }


// fn main(){

//     let a = CircleBuilder::new().x(10).y(20).radius(4).finalize().area();
//     println!("Area is: {}",a);
// }


//String
//Two type: String and &str.
//&str is a reference to statically allocated string
//String is a string in heap

// fn main(){

//     let name:&str = "Hello";
//     let name_heap:String = name.to_string();

//     let name2 = name;
//     println!("Name: {}",name);//Copy trait implemented

//     let name_heap2:String = name_heap;
//     // println!("NameHeap: {}",name_heap); //Gives error because heap + stack so no copy trait implemented


//     //String can be coerced to &str  by using &
//     let nickname = "World".to_string();
//     takeslice(&nickname);

//     let sentence = "Hello this is me";
//     for x in sentence.chars(){
//         println!("Char: {}",x);
//     }



// }

// fn takeslice(slice:&str){
//     println!("Slice: {}",slice);
// }


//Generics (Similar to c++)

//Generic function
// fn do_anything<T,Z>(x:T,y:Z)->(T,Z){
//     (x,y)
// }

// //Generic enum
// enum Case<T,E>{
//     Yes(T),
//     No(E)
// }

// //Generic struct with generic impl
// struct Point<T>{
//     x:T,
//     y:T,
//     z:T
// }

// impl<T> Point<T>{
    
//     fn getproduct(&self) -> T{
//         self.x*self.y*self.z //Error ?? to be found
//     }
// }

// fn main(){

//     let (a,b) = do_anything::<i32,f64>(10,1.2);

//     let c:Case<bool,&str> = Case::No("Error");

//     let p:Point<i32> = Point{x:0,y:0,z:0};

    
// }

//Trait (Kind of like typescript interface)

//Trait on struct

// trait hasArea{
//     fn getarea(&self)->i32;
// }

// struct Land{
//     x:i32,
//     y:i32
// }

// impl hasArea for Land{
//     fn getarea(&self)->i32{
//         self.x * self.y
//     }
// }



// //Generic traits on generic function
// trait calArea{
//     fn area(&self)->i32;
// }

// struct rec{
//     x:i32,
//     y:i32
// }

// impl calArea for rec{
//     fn area(&self)->i32{
//         self.x*self.y
//     }
// }

// fn print_area<T:calArea>(shape:T){
//     println!("The area is: {}",shape.area());
// }

// //Generic trait for generic struct
// trait Equal{
//     fn checkeq(&self)->bool;
// }
// struct Rectangle<T>{
//     x:T,
//     y:T
// }

// impl <T:std::cmp::PartialEq> Equal for Rectangle<T>{
//     fn checkeq(&self)->bool{
//         self.x == self.y
//     }

// }

// //traits can be inherited
// trait Foo{
    
// }

// trait Foobar: Foo{

// }

// //Drop trait (default available) (Used to free memory after out of scope)
// struct HasDropped;

// //This overwrites trait
// // trait Drop{
// //     fn drop(&mut self);
// // }

// impl Drop for HasDropped{
//     fn drop(&mut self){
//         println!("Dropping");
//     }
// }

// fn main(){

//     let r = rec{x:10,y:10};
//     print_area(r);

//     let x:i32 = 10;
//     let y:i32 =10;
//     let r2:Rectangle<i32> = Rectangle{x,y};
//     let isequal = r2.checkeq();
//     println!("{}",isequal);


//     let x = HasDropped;

// }


//Trait object
// Determining which version to run during polymorphism is called dispatch
// 2 types => Static and dynamic dispatch