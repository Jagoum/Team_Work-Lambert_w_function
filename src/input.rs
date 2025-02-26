use structopt::StructOpt;
/// CLI Options
#[derive(StructOpt)]

pub struct Cli {
    /// The input value for the Lambert W function
    #[structopt(parse(try_from_str = parse_f64))]
    x: f64,
}

fn parse_f64(src: &str) -> Result<f64, String> {
    src.parse().map_err(|_| format!("Invalid number: {}", src))
}

impl Cli {
    pub fn new() -> Cli {
        Cli::from_args()
    }

    pub fn get(&self) -> f64 {
        self.x
    }
}
