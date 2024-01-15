use structopt::StructOpt;
#[derive(Debug,StructOpt)]
struct Option{
    message : String
}

fn main() {
    let options = Option::from_args();
    let message = options.message;
    println!("{}",message);
    println!(" \\");
    println!(" \\");
    println!("  /\\_/\\");
    println!(" ( o o )");
    println!(" =( I )=");
}