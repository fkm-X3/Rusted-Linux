use vt_core::vt;

fn main() {
    match vt::current() {
        Ok(n) => println!("{}", n),
        Err(e) => {
            eprintln!("fgconsole: {}", e);
            std::process::exit(1);
        }
    }
}
