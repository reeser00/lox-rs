use lox_rs::Lox;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    println!("{:?}", &args);
    let mut lox_interp: Lox = Lox::new();
    lox_interp.main(args);
} 
