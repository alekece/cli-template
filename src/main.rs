use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {}

fn main() {
  let _opt = Opt::from_args();
}
