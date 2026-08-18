use clap::{Parser, Subcommand};
use oddsradar::map::compare_files;
use oddsradar::secrets::forbidden_fields;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "oddsradar", about = "Read-only prediction-market spread radar")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Doctor {
        #[arg(long)]
        config: PathBuf,
    },
    Compare {
        #[arg(long)]
        config: PathBuf,
        #[arg(long)]
        map: PathBuf,
        #[arg(long)]
        quotes: PathBuf,
        #[arg(long)]
        threshold: Option<i64>,
        #[arg(long)]
        notify_file: Option<PathBuf>,
    },
}

fn load_cfg(path: &std::path::Path) -> Result<serde_json::Value, ExitCode> {
    let raw = std::fs::read_to_string(path).map_err(|e| {
        eprintln!("{e}");
        ExitCode::from(1)
    })?;
    let v: serde_json::Value = serde_json::from_str(&raw).map_err(|e| {
        eprintln!("{e}");
        ExitCode::from(1)
    })?;
    let hits = forbidden_fields(&v);
    if !hits.is_empty() {
        eprintln!("doctor: forbidden secret field(s): {}", hits.join(", "));
        return Err(ExitCode::from(2));
    }
    Ok(v)
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Doctor { config } => match load_cfg(&config) {
            Ok(v) => {
                println!(
                    "ok threshold={}",
                    v.get("threshold_millionths").and_then(|x| x.as_i64()).unwrap_or(50_000)
                );
                ExitCode::SUCCESS
            }
            Err(c) => c,
        },
        Cmd::Compare {
            config,
            map,
            quotes,
            threshold,
            notify_file,
        } => {
            let v = match load_cfg(&config) {
                Ok(v) => v,
                Err(c) => return c,
            };
            let th = threshold.unwrap_or_else(|| {
                v.get("threshold_millionths")
                    .and_then(|x| x.as_i64())
                    .unwrap_or(50_000)
            });
            let rows = match compare_files(&map, &quotes, th) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("{e}");
                    return ExitCode::from(1);
                }
            };
            for r in &rows {
                println!("{}", serde_json::to_string(r).unwrap());
            }
            if let Some(path) = notify_file {
                if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(&path) {
                    for r in rows.iter().filter(|r| r.kind == "spread") {
                        let _ = writeln!(f, "{}", serde_json::to_string(r).unwrap());
                    }
                    println!("notify file:{}", path.display());
                }
            }
            ExitCode::SUCCESS
        }
    }
}
