use indicatif;
use std::thread;
use std::time::Duration;
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    load_type: String,
    load_time: u64,
    #[clap(short, long)]
    style: Option<String>,
}

fn main() {
    let args = Cli::parse();

    if args.load_type == "bar" {
        let pb = indicatif::ProgressBar::new(args.load_time);

        if !args.style.is_none() {
            pb.set_style(indicatif::ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .unwrap()
                .progress_chars("##-"));
        }
        
        for _ in 0..args.load_time {
            thread::sleep(Duration::from_secs(1));
            pb.inc(1); 
        }

        pb.finish_with_message("done");
    }
    else if args.load_type == "spinner" {
        let pb = indicatif::ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(100));
        for _ in 0..args.load_time {
            thread::sleep(Duration::from_secs(1));
        }
        pb.finish();
    }
    
}
