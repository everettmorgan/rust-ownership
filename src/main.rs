fn main () {
    // allocated on heap
    let s = String::from ("Hello, world!");
    // s changes scope
    change_scope (s);

    // println!("{}", s); compile-time error

    // allocated on stack
    let x = 5;
    makes_copy (x);
    // x remains in scope
    println! ("{}", x);
    // x goes out of scope and drop is called

    // s comes into scope
    let s = give_ownership ();
    println! ("{}", s);

    // s goes out of scope
    let a = take_and_giveback (s);
    // s comes back into scope as a
    println! ("{}", a);
}

fn makes_copy (x : i32) {
    // x comes into scope
    println! ("{}", x);
    // x goes out of scope and drop is called
}

fn change_scope (x : String) {
    // x comes into scope
    println! ("{}", x);
    // x goes out of scope and drop is called
}

fn give_ownership () -> String {
    // some_string comes into scope
    let some_string = String::from ("Hello, world!");
    some_string
    // some_string transfers out of scope
}

fn take_and_giveback (s : String) -> String {
    // s comes into scope
    s
    // s transfers out of scope
}