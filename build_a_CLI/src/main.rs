extern crate colored;
extern crate structopt;

use colored::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "Meow!")]
    /// What does the cat say?
    message: String,

    #[structopt(short = "d", long = "dead")]
    /// Make the cat appear dead
    dead: bool,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Load the cat picture from the specified file
    catfile: Option<std::path::PathBuf>,
}

fn main() {
    let options = Options::from_args();
    let message = options.message;

    let eyes = if options.dead { "x" } else { "o" };
    //println!("{}", message);
    match &options.catfile {
        Some(path) => {
            let cat_template =
                std::fs::read_to_string(path).expect(&format!("could not read file{:?}", path));
            let cat_picture = cat_template.replace("{eye}", eyes);
            println!("{}", &cat_picture);
        }

        None => {
            if message.to_lowercase() == "woof" {
                eprint!("A cat shouldn't bark like a dog.")
            }
            // let message = std::env::args().nth(1)
            // .expect("missing the message. usage: catsay <message>");
            println!("{}", message.bright_yellow().underline().on_purple());
            println!("\\");
            println!(" \\");
            println!("    /\\_/\\");
            println!("   ( {eye} {eye} )", eye = eyes.red().bold());
            println!("   =( I )=");
        }
    }
}
