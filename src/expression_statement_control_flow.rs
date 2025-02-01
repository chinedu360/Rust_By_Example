pub fn express_pattern() {
    //statement
    let x = 15;
    x + 1;

    //expression
    let y = {
        let a = 10;
        a + 10
    };
    println!("value of y: {}", y);

    //control flow

    // if and else
    let is_rust_cool = false;

    let message = if is_rust_cool {
        "Absolutely"
    } else {
        "You must be joking"
    };

    println!("{}", message);

    // match
    enum Weather {
        Rainy,
        Sunny,
        Snowy,
    }

    let today = Weather::Rainy;

    match today {
        Weather::Rainy => {
            println!("🌧️ its rainy");
            println!("Do not forget to carry an umbrella");
        }
        Weather::Snowy => {
            println!("❄️ its Snowy, get a jacket.")
        }
        Weather::Sunny => {
            println!("☀️ its sunny today")
        }
    }

    let number = 400;

    match number {
        1 => println!("it's one"),
        2..200 => println!("the number is btw 2 and 200"),
        n => println!("Got {}", n),
        // _ => println!("Got somethin else")
    }

    // if let
    let maybe_age = Some(38);

    //example with match
    match maybe_age {
        Some(age) => println!("Match Age is {}", age),
        _ => (),
    }

    // example with if let
    if let Some(age) = maybe_age {
        println!("If let Age is {}", age)
    }

    // let-else
    let Some(name) = Some("Captain") else {
        eprintln!("no name was found at this time");
        return;
    };

    println!("Hello, {}", name);

    // while let

    let mut coin = vec![Some(1), Some(2), Some(3), Some(4)];

    while let Some(Some(coin)) = coin.pop() {
        println!("Found a {}₦ coin", coin);
    }

    // for loops and range

    loop {
        println!("My name is chinedu");
        break;
    }

    let mut count = 0;

    while count <= 3 {
        println!("Count: {}", count);
        count += 1;
    }

    //iterate over a range
    for i in 0..10 {
        // println!("{:?}", i)
    }
    for i in 0..=10 {
        // println!("{:?}", i)
    }

    // itrarete over a collection
    let fruits = ["apple", "orange", "mango"];

    for fruit in fruits.iter() {
        // println!("{:?}", fruit)
    }
}
