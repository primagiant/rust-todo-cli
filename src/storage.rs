use crate::models::Todolist;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};

pub fn save_to_file(todos: &Todolist, filename: &str) -> io::Result<()> {
    let file_out = File::create(filename)?;
    let writer = BufWriter::new(file_out);
    serde_json::to_writer_pretty(writer, todos)?;
    Ok(())
}

pub fn load_from_file(filename: &str) -> io::Result<Todolist> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let data: Todolist = serde_json::from_reader(reader)?;
    Ok(data)
}
