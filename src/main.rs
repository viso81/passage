fn main() {
    // env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    if let Err(e) = passage::run() {
        eprintln!("{:?}", e);
        std::process::exit(1);
    }
}
