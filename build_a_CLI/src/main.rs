extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Option {
    #[structopt(default_value = "Meow!")]
    /// What does the cat say?
    message: String,

    #[structopt(short = "d", long = "dead")]
    /// Make the cat appear dead
    dead: bool,
}


fn main() {
    let options = Option::from_args();
    let message = options.message;

    let eyes = if options.dead {"x"} else{"o"};

    // let message = std::env::args().nth(1)
        // .expect("missing the message. usage: catsay <message>");

    println!("{}", message);
    println!("\\");
    println!(" \\");
    println!("    /\\_/\\");
    println!("   ( {eye} {eye} )", eye=eyes);
    println!("   =( I )=");
}