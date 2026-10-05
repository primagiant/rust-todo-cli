mod cli;
mod models;
mod storage;

use clap::Parser;
use cli::{Cli, Commands};
use models::Todolist;
use std::io;

fn main() {
    let cli = Cli::parse();
    let filename = "todos.json";

    let mut todos = match storage::load_from_file(filename) {
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
            if let Err(e) = storage::save_to_file(&todos, filename) {
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
                if let Err(e) = storage::save_to_file(&todos, filename) {
                    eprintln!("Gagal menyimpan data: {}", e);
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        },
        Commands::Delete { id } => match todos.delete_task(id) {
            Ok(_) => {
                println!("Menghapus tugas dengan ID {}", id);
                if let Err(e) = storage::save_to_file(&todos, filename) {
                    eprintln!("Gagal menyimpan data: {}", e);
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        },
    }
}
