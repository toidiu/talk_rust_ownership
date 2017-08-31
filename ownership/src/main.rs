#![allow(unused)]


fn main() {
    println!("Hello, world!");

    let foo = 5;
}


fn what_is_ownership() {
    // “data” is owned by foo
    let foo: String = String::from("data");
    // “ownership of data” is moved to bar
    let bar: String = foo; //ownership moved
    // invalid because foo no longer owns “data”
    //    println!("{}", foo); //moved after use
}


fn ways_to_share_data() {
    {
        // “data” is owned by foo
        let foo: String = String::from("data");
        let bar: String = foo; //ownership moved
        //        println!("{}", foo); //moved after use
    }
    {
        //reference (immutable)
        let foo = String::from("data");
        let bar = &foo;  //create a reference
        println!("{}", foo);  //foo still owns its data
        println!("{}", foo);  //multiple reference allowed
    }
    {
        //mutable reference
        let mut foo = String::from("data");
        let bar = &mut foo;  //create a mut reference
        //        println!("{}", foo);  //only 1 mut reference allowed
    }
}

