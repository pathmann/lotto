use colored::*;
use clap::{Parser, ValueEnum, CommandFactory};
use rand::{thread_rng};

/// Predefined game types
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum GameType {
    /// numcount_a = 6, poolsize_a = 49
    Lotto,
    /// numcount_a = 5, poolsize_a = 50, numcount_b = 2, poolsize_b = 12
    Eurojackpot,
    /// use numcounts and poolsizes from app arguments
    Custom
}

/// Arguments which can be passed to the app
#[derive(Parser)]
struct AppArgs {
    #[arg(short, long, default_value_t = 1)]
    fieldcount: u32,
    #[arg(short, long, value_enum, default_value_t = GameType::Eurojackpot)]
    game: GameType,

    //for GameType::Custom
    ///Numbercount (Only for custom games)
    numbercount_a: Option<usize>,
    ///Poolsize (Only for custom games)
    poolsize_a: Option<usize>,
    ///Second numbercount (Only for custom games)
    numbercount_b: Option<usize>,
    ///Second poolsize (Only for custom games)
    poolsize_b: Option<usize>,

    ///don't print indexes (if fieldcount > 1)
    #[arg(short, long)]
    noindexprint: bool,
} 

/// Returns a random generated lottery field
/// 
/// # Arguments
/// 
/// * `numcount` - numbers taken from the pool
/// 
/// * `poolsize` - size of the sample's pool
/// 
fn get_field(numcount: usize, poolsize: usize) -> String {
    let mut rng = thread_rng();

    let mut sample = rand::seq::index::sample(&mut rng, poolsize, numcount).into_vec();

    sample.sort();

    sample = sample.iter().map(|i| i +1).collect();

    let mut res = String::new();
    for i in sample {
        res += &i.to_string();
        res += " ";
    }

    res
}

/// Prints a random generated lottery field 
    /// 
    /// # Arguments
    /// 
    /// * `id` - index of the field, if equals to 0 the index won't be printed
    /// 
    /// * `numcount1` - first count of numbers taken for one field 
    /// 
    /// * `poolsize1` - first poolsize for one field
    /// 
    /// * `numcount2` - second count of numbers taken for one field, pass 0 to omit
    /// 
    /// * `poolsize2` - second poolsize for one field, pass 0 to omit
    /// 
fn print_field(id: u32, numcount1: usize, poolsize1: usize, numcount2: usize, poolsize2: usize) {
    if id != 0 {
        print!("{}\t", (id.to_string() + ".").yellow());
    }

    print!("{}", get_field(numcount1, poolsize1).green());

    if numcount2 > 0 && poolsize2 > 0 {
        println!("");

        let field2 = get_field(numcount2, poolsize2);
        if id != 0 {
            print!("\t");
        }
        
        print!("{}", field2.to_string().green());
    }
    
    println!("");
}

fn main() {
    let args = AppArgs::parse();

    let numcounta;
    let poola;
    let numcountb;
    let poolb;

    match args.game {
        GameType::Eurojackpot => {
            numcounta = 5;
            poola = 50;
            numcountb = 2;
            poolb = 12;
        },
        GameType::Lotto => {
            numcounta = 6;
            poola = 49;
            numcountb = 0;
            poolb = 0;
        },
        GameType::Custom => {
            if args.numbercount_a.is_none() || args.poolsize_a.is_none() {
                println!("{}", "No numbercount_a and/or poolsize_a set".red());
                AppArgs::command().print_help().unwrap();
                std::process::exit(1);

            }

            numcounta = args.numbercount_a.unwrap();
            poola = args.poolsize_a.unwrap();

            if args.numbercount_b.is_some() {
                if args.poolsize_b.is_none() {
                    println!("{}", "poolsize_b must be set when numbercount_b is set".red());
                    AppArgs::command().print_help().unwrap();
                    std::process::exit(1);
                }

                numcountb = args.numbercount_b.unwrap();
                poolb = args.poolsize_b.unwrap();
            }
            else {
                numcountb = 0;
                poolb = 0;
            }
        }
    }

    let noprint = args.noindexprint;
    if noprint || args.fieldcount == 1 {
        for _ in 0..args.fieldcount {
            print_field(0, numcounta, poola, numcountb, poolb);
        }
    }
    else {
        for i in 0..args.fieldcount {
            print_field(i +1, numcounta, poola, numcountb, poolb);
        }
    }
}
