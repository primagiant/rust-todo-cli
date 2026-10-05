use std::fmt;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "CLI To-Do List App sederhana di Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Menambahkan tugas baru
    Add { title: String },
    /// Menampilkan semua daftar tugas
    List,
    /// Menandai tugas sebagai selesai berdasarkan ID
    Done { id: u32 },
    /// Menghapus tugas berdasarkan ID
    Delete { id: u32 },
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
}

impl Task {
    fn new(id: u32, title: String) -> Self {
        Self {
            id,
            title,
            completed: false,
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.completed { "[X]" } else { "[ ]" };
        write!(f, "{} {} - {}", status, self.id, self.title)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Todolist {
    items: Vec<Task>,
}

impl Todolist {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn add_task(&mut self, title: String) {
        let id = self.items.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        let task = Task::new(id, title);
        self.items.push(task);
    }

    fn list_task(&self) {
        if self.items.is_empty() {
            println!("Belum ada tugas.");
        } else {
            print!("{}", self);
        }
    }

    fn complete_task(&mut self, id: u32) -> Result<(), String> {
        if let Some(todo) = self.items.iter_mut().find(|t| t.id == id) {
            todo.completed = true;
            Ok(())
        } else {
            Err(format!("Task dengan ID {} tidak ditemukan.", id))
        }
    }

    fn delete_task(&mut self, id: u32) -> Result<(), String> {
        if let Some(index) = self.items.iter().position(|t| t.id == id) {
            self.items.remove(index);
            Ok(())
        } else {
            Err(format!("Task dengan ID {} tidak ditemukan.", id))
        }
    }

    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let file_out = File::create(filename)?;
        let writer = BufWriter::new(file_out);
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }

    fn load_from_file(filename: &str) -> io::Result<Self> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        let data: Todolist = serde_json::from_reader(reader)?;
        Ok(data)
    }
}

impl fmt::Display for Todolist {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for todo in &self.items {
            writeln!(f, "{}", todo)?;
        }
        Ok(())
    }
}

fn main() {
    let cli = Cli::parse();

    let filename = "todos.json";
    let mut todos = match Todolist::load_from_file(filename) {
        Ok(data) => data,
        Err(e) if e.kind() == io::ErrorKind::NotFound => Todolist::new(),
        Err(e) => {
            eprintln!(
                "Peringatan: Gagal membaca file ({}), membuat daftar baru.",
                e
            );
            Todolist::new()
        }
    };

    match cli.command {
        Commands::Add { title } => {
            todos.add_task(title.clone());
            println!("Menambahkan tugas baru: \"{}\"", title);
            if let Err(e) = todos.save_to_file(filename) {
                eprintln!("Gagal menyimpan data: {}", e);
            }
        }
        Commands::List => {
            println!("Menampilkan semua daftar tugas:");
            todos.list_task();
        }
        Commands::Done { id } => match todos.complete_task(id) {
            Ok(_) => {
                println!("Menandai tugas dengan ID {} sebagai SELESAI!", id);
                if let Err(e) = todos.save_to_file(filename) {
                    eprintln!("Gagal menyimpan data: {}", e);
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        },
        Commands::Delete { id } => match todos.delete_task(id) {
            Ok(_) => {
                println!("Menghapus tugas dengan ID {}", id);
                if let Err(e) = todos.save_to_file(filename) {
                    eprintln!("Gagal menyimpan data: {}", e);
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        },
    }
}
