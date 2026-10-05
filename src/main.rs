use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write};

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
    println!("=== Aplikasi CLI To-Do List ===");
    println!("Perintah: add <title> | list | done <id> | delete <id> | exit\n");

    let filename = "todos.json";
    let mut todos = match Todolist::load_from_file(filename) {
        Ok(data) => data,
        Err(e) if e.kind() == io::ErrorKind::NotFound => Todolist::new(),
        Err(e) => {
            println!(
                "Peringatan: Gagal membaca file ({}), membuat daftar baru.",
                e
            );
            Todolist::new()
        }
    };

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Gagal membaca input.");
            continue;
        }

        let trimmed_input = input.trim();
        if trimmed_input.is_empty() {
            continue;
        }

        let mut parts = trimmed_input.splitn(2, ' ');
        let command = parts.next().unwrap_or("");
        let argument = parts.next();

        match command {
            "add" => {
                if let Some(title) = argument {
                    todos.add_task(title.to_string());
                    println!("Tugas berhasil ditambahkan.");
                    let _ = todos.save_to_file(filename);
                } else {
                    println!("Error: Judul tugas tidak boleh kosong! (Contoh: add Belajar Rust)");
                }
            }
            "list" => {
                println!("--- Daftar Tugas ---");
                todos.list_task();
            }
            "done" => {
                if let Some(id_str) = argument {
                    match id_str.parse::<u32>() {
                        Ok(id) => match todos.complete_task(id) {
                            Ok(_) => {
                                println!("Tugas ID {} selesai.", id);
                                let _ = todos.save_to_file(filename);
                            }
                            Err(err) => println!("Error: {}", err),
                        },
                        Err(_) => println!("Error: ID harus berupa angka!"),
                    }
                } else {
                    println!("Error: Masukkan ID tugas! (Contoh: done 1)");
                }
            }
            "delete" => {
                if let Some(id_str) = argument {
                    match id_str.parse::<u32>() {
                        Ok(id) => match todos.delete_task(id) {
                            Ok(_) => {
                                println!("Tugas ID {} dihapus.", id);
                                let _ = todos.save_to_file(filename);
                            }
                            Err(err) => println!("Error: {}", err),
                        },
                        Err(_) => println!("Error: ID harus berupa angka!"),
                    }
                } else {
                    println!("Error: Masukkan ID tugas! (Contoh: delete 1)");
                }
            }
            "exit" => {
                if todos.save_to_file(filename).is_ok() {
                    println!("Berhasil menyimpan. Keluar dari program. Sampai jumpa!");
                } else {
                    println!("Gagal menyimpan data ke file! Keluar dari program.");
                }
                break;
            }
            _ => {
                println!("Perintah '{}' tidak dikenal.", command);
            }
        }

        println!();
    }
}
