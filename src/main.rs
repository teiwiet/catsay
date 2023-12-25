use std::env;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options{
    message : String,
    #[structopt(short="d",long = "dead")]
    dead : bool
}


fn main(){
    let options = Options::from_args();
    let message = options.message;
    // let mut args : Vec<String> = env::args().collect();
    // println!("{}", args[1]);
    if message.to_lowercase() == "woof"{
        eprintln!(":-))");
        return
    }
    let eye = if options.dead {
        "x"
    }
    else {
        "O"
    };
    println!("{}", message);
    println!(" \\");
    println!(" \\");
    println!("  /\\_/\\");
    println!(" ( {eye} {eye} )");
    println!(" =( I )=");
}