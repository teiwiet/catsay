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
    println!("{}", message);
    println!(" \\");
    println!(" \\");
    println!("  /\\_/\\");
    println!(" ( o o )");
    println!(" =( I )=");
}