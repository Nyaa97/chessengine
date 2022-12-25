use clap::Parser;

#[derive(Parser)]
#[command(name = "Chess Engine")]
#[command(author = "Michał Zawieja")]
#[command(version)]
pub struct CLI {
	#[arg(short, long, value_name = "FILE")]
	pub json: Option<std::path::PathBuf>,
}
