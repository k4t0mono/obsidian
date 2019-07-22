mod config;

use config::Config;

fn main() {
    let c = Config::loads();
    println!("{:?}", c);
}
