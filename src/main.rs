use std::env;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options{
    #[structopt(default_value = "Meow!")]
    ///what does the cat say ?
    message : String,
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