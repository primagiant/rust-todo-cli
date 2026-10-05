use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "CLI To-Do List App sederhana di Rust", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Menambahkan tugas baru
    Add { title: String },
    /// Menampilkan semua daftar tugas
    List,
    /// Menandai tugas sebagai selesai berdasarkan ID
    Done { id: u32 },
    /// Menghapus tugas berdasarkan ID
    Delete { id: u32 },
}
