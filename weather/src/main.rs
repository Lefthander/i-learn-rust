use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    country_code : String,
    city: String,
}
fn main() {
    let args = Cli::from_args();
    println!("City code = {}, City = {}",args.country_code, args.city);
}
