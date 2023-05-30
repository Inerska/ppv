use std::io;

pub trait Clonable {
    fn clone_project(&self) -> io::Result<()>;
}
