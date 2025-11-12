use std::io;

fn READ(x: String) -> String {
    x
}

fn EVAL(x: String) -> String {
    x
}

fn PRINT(x: String) -> String {
    x
}

fn rep(input: String) -> String {
    let read: String = READ(input);
    let evaled: String = EVAL(read);
    let printed: String = PRINT(evaled);
    printed
}


fn main(){
    loop {
        let mut input = String::new();
        println!("input:");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let output: String = rep(input);
        println!("{}", output);
    }
}
