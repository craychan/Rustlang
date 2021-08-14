extern crate structopt;
extern crate colored;

use structopt::StructOpt;
use colored::*;

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

    if message.to_lowercase() == "woof" {
        eprint!("A cat shouldn't bark like a dog.")
    }

    // let message = std::env::args().nth(1)
        // .expect("missing the message. usage: catsay <message>");

    println!("{}", message.bright_yellow().underline().on_purple());
    println!("\\");
    println!(" \\");
    println!("    /\\_/\\");
    println!("   ( {eye} {eye} )", eye=eyes.red().bold());
    println!("   =( I )=");
}