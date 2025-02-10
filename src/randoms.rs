use std::fs;

// Option<T> // Represents a value that might exist or might not
// Result<T, E> Represents an operation that might succeed or fail, with detailed error messages when it fails.

//Option Enum
// enum Option<T> {
//     Some(T), // there is a value
//     None, // we have no value
// }

// Result Enum
// enum Result<T, E> {
//     Ok(T), // success contains values
//     Err(E) // Failed contains error
// }

// example use case of Option enum
// fn find_user_by_id(id: usize) -> Option<User> {
//     if database.contains(id) {
//         Some(database.get(id))
//     } else {
//         None
//     }
// }

// match find_user_by_id(200) {
//     Some(user) => println!("User found! {}", user.name);
//     None => println!("User not found")
// }

struct Config {
    contents: String,
}

impl Config {
    fn parse(contents: &str) -> Config {
        Config {
            contents: contents.to_string(),
        }
    }
}

// fn load_config() -> Result<Config, std::io::Error> {
//        let contents = read_config_file("./src/randoms.rs")?;
//        Ok(Config::parse(&contents))
// }

fn read_config_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

pub fn random_fn() -> Result<Config, std::io::Error> {
    let maybe_some_string = Some(String::from("Hello, World!"));
    // `Option::map` takes self *by value*, consuming `maybe_some_string`
    let maybe_some_len = maybe_some_string.map(|s| s.len());

    let number = Some(10);
    // let boolean: Option<bool> = Some(true);

    let doubled = number.map(|x| x * 2);

    let none = None.unwrap_or(0);

    println!("doubled: {:?}, none: {:?}", doubled, none);

    // match read_config_file("./src/randoms.rs") {
    //     Ok(contents) => println!("Contents: {}", contents),
    //     Err(e) => println!("Error: {}", e),
    // };

    let contents = read_config_file("./src/randoms.rs")?;
    Ok(Config::parse(&contents))

    // Ok(())
}
