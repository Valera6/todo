pub mod config;
pub mod day_section;
pub mod timer;
pub mod manual_stats;
pub mod todos;
pub mod utils;
use config::Config;
use utils::ExpandedPath;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
	#[arg(long, default_value = "~/.config/todo.toml")]
	config: ExpandedPath,
}

#[derive(Subcommand)]
enum Commands {
	/// Opens the target path
	///Ex
	///```rust
	///todo open -w
	///```
	Open(todos::OpenArgs),
	/// Add a new task.
	/// Every entry has the following format:
	/// `{importance}-{difficulty}-{name}`,
	///where:
	///- importance: 0->9, the higher the more important
	///- difficulty: 0->9, the higher the more difficult
	///Ex:
	///```rust
	///todo add 2-3-test -n
	///```
	Add(todos::AddArgs),
	/// Compile list of first priority tasks based on time of day
	///Ex:
	///```rust
	///todo quickfix
	///```
	Quickfix(todos::QuickfixArgs),
	/// Record day's ev and other stats
	///```rust
	///todo manual --ev 420 -oy
	///```
	Manual(manual_stats::ManualArgs),
	/// Start a task with timer, then store error (to track improvement of your estimations of time spent on different task categories)
	///Example Usage:
	///'''rust
	///todo do start -t=15 -w --description==do-da-work
	///. . . // start doing the task, then:
	///todo do done
	///'''
	Timer(timer::TimerArgs),
}

fn main() {
	let cli = Cli::parse();

	let config = match Config::try_from(cli.config) {
		Ok(cfg) => cfg,
		Err(e) => {
			eprintln!("Error: {}", e);
			std::process::exit(1);
		}
	};

	// All the functions here can rely on config being correct.
	let success = match cli.command {
		Commands::Open(open_args) => {
			let mut todos_flags = open_args.shared;
			todos_flags.open = true;
			todos::open_or_add(config, todos_flags, None)
		}
		Commands::Add(add_args) => todos::open_or_add(config, add_args.shared, Some(add_args.name)),
		Commands::Quickfix(_) => todos::compile_quickfix(config),
		Commands::Manual(manual_args) => manual_stats::update_or_open(config, manual_args),
		Commands::Timer(timer_args) => timer::timing_the_task(config, timer_args),
	};

	match success {
		Ok(_) => std::process::exit(0),
		Err(e) => {
			eprintln!("Error: {}", e);
			std::process::exit(1);
		}
	}
}
