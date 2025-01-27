// -- casting btw primitive types
// -- specifying the type literals
// -- using type inference
// -- Aliasing
// #[allow(overflowing_literals)]
pub fn types() {
    // casting types
    let decimal = 55.8;

    //explicit conversion
    let interger = decimal as u8;
    let character = interger as char;

    // println!("Casting: {} --> {} --> {}", decimal, interger, character);

    // println!("1000 as u16 is {}:", 1000 as u16);
    // println!("1000 as u8 is {}:", 1000 as u8);
    // println!("-1 as a u8 is {}", (1i8) as u8);

    //literals
    // 80i32

    let x = 1u8;
    let y = 2u32;
    let z = 2.0f32;

    let i = 0;
    let f = 3.2;

    // find the memory size
    println!("size of x in byte: {}", std::mem::size_of_val(&x));
    println!("size of y in byte: {}", std::mem::size_of_val(&y));
    println!("size of z in byte: {}", std::mem::size_of_val(&z));

    //Type inference
    let elem = 5u8;
    let mut vec = Vec::new();
    vec.push(elem);
    println!("{:?}", vec);

    //Aliasing Types
    type NanoSec = u64;
    type Inch = u64;
    type U64 = u64;

    let nanosec:NanoSec = 5 as u64;
    let inches:Inch = 2 as U64;

    println!("{} nanosec + {} inches = {} unit?",
 nanosec, inches,nanosec + inches);
}
