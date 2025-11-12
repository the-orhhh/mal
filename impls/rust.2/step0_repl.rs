use std::io;
use std::io::Write;

#[allow(non_snake_case)]
fn READ(x: String) -> String {
    x
}

#[allow(non_snake_case)]
fn EVAL(x: String) -> String {
    x
}

#[allow(non_snake_case)]
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
        print!("user> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if let Ok(bytes_in) = io::stdin().read_line(&mut input) {
            if bytes_in == 0 {
                break; // 0 bytes = EOF
            }
            let output = rep(input);
            print!("{}", output);
        } else {
            println!("Error reading input");
            break;
        }
    }
}
