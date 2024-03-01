use pfact::config;
use pfact::fork_join;

fn main() {
    let num = config::get_args();
    println!("{}", fork_join::compute(num));
}
