fn main() {
    let string = "Hello, world!";
    println!("{}", string);

    let max_lenght = 4;

    println!(
        "String \"{}\" is {} than {} characters.",
        string,
        if max_lenght < 10 {
            "shorter"
        } else {
            "not shorter"
        },
        max_lenght
    );
}
