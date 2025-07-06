use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to password list file (.txt or .csv format)
    #[arg(short, long)]
    pub password_list_path: String,

    /// Path to encrypted .dmg file
    #[arg(short, long)]
    pub dmg_path: String,

    /// Number of threads to run (defaults to num of logical cores)
    #[arg(short, long, default_value_t = num_cpus::get())]
    pub thread_count: usize,

    /// Randomize password order to avoid predictable patterns
    #[arg(short, long, default_value_t = false)]
    pub randomize: bool,
}
