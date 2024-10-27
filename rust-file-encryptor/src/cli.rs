use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,
    #[structopt(short, long)]
    decrypt: bool,
}

pub fn parse_args() -> Cli {
    Cli::from_args()
}
