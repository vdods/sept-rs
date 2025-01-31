// TODO: Should probably actually use something like std::fmt::Display, where it writes to a std::io::Write,
// and then provide stringify as a convenience method that writes to a String.
pub trait Stringifiable {
    fn stringify(&self) -> String;
}
