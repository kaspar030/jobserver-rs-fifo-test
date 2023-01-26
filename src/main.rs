fn main() {
    let envs = ["CARGO_MAKEFLAGS", "MAKEFLAGS", "MFLAGS"];
    for e in envs.iter() {
        if let Ok(s) = std::env::var(e) {
            println!("{}={}", e, s);
        }
    }

    println!("{:?}", unsafe { jobserver::Client::from_env() });
}
