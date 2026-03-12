fn main() {
    if let Err(error) = shadcn_rs_cli::run() {
        eprintln!("Error: {error:#}");
        std::process::exit(1);
    }
}
