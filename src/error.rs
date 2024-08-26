#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Parser,
    Runtime,
}

