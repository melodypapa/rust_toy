fn main() {
    println!("Hello, world!");

    let string_literal = "Hello world 2";
    print(string_literal.to_string());

    //let test_string = &"Hello word 3";
    //print(test_string);

    std::process::exit(0);
}

fn print(input: String){
    println!("{}", input);
}

/*fn print(input_string: &str) {
    println!("{}", input_string);
}*/