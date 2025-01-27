pub fn bindings() {
    //Variable bindings
    let an_interger = 1i8;
    let _a_boolean = true;
    let _unit = ();

    // copy 'an_interger' into 'copied_interger'
    let _copied_interger = an_interger;

    // println!("An Interger: {:?}", copied_interger);
    // println!("A boolen: {:?}", a_boolean);
    // println!("Meet thge unit value: {:?}", unit);

    //Unused Variables and warnings
    let _unused_var = 3u32;

    let _noisy_var = 2u32;

    //Mutablity
    let _immutable_binding = 1;
    let mut _mutable_binding = 1;

    // println!("Before Mutation: {}", mutable_binding); //1

    // mutable_binding += 1;

    // println!("After Mutation: {}", mutable_binding); //2

    //Err: cannot mutate immutable variable
    // _immutable_binding += 1;

    //Scope and shadowing
    let long_lived_binding = 1;
    let shadowed_binding = 1;

    //scope
    {
        let short_lived_binding = 2;
        println!("Inner Short: {}", short_lived_binding);
        println!("Inner Long: {}", long_lived_binding);
    }

    // println!("Outer Short: {}", short_lived_binding);
    println!("Outer Long: {}", long_lived_binding);

    //shadow
    {
        let shadowed_binding = "abc";
        println!("shadowed inner block: {}", shadowed_binding);
    }

    let shadowed_binding = 2;
    println!("shadowed outer block: {}", shadowed_binding);

    //Declaring Variables first

    let a_binding;

    {
        let x = 2;
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding: i32;
    // Err: `another_binding` used here but it isn't initialized
    // println!("another binding: {}", another_binding);

    another_binding = 10;
    println!("another binding: {}", another_binding);
}
