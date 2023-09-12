use std::io:: Write;

struct LogReader {}

impl LogReader {
    pub fn print<T>(&self, dest: &mut T)
    where
        T: Write,
    {
        dest.write("Hello World".as_bytes()).expect("");
        dest.flush().expect("");
    }
}
fn main() {
    let obj= LogReader{};
    obj.print(&mut std::io::stdout());
}
