fn main() -> std::io::Result<()> {
    if let Err(e) = cutr::get_args().and_then(cutr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
    Ok(())
}
