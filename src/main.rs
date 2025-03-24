use clap::Parser;
use explain_name::explain_filename;
use fastq::{avg_base_quality, avg_err_prob, calculate_phred, read_fastq, print_quality_table};
use std::fs::{self, read};
use std::path::{Path, PathBuf};

mod explain_name;
mod fastq;

#[derive(Parser)]

// comand line interface struct
struct Cli {
    input1: PathBuf,
    input2: Option<PathBuf>,
}

fn main() {

    // Pfad zu einer oder zwei Dateien einlesen
    let args = Cli::parse();
    println!("1st file provided:{}", args.input1.display());

    //
    if let Some(ref input2) = args.input2 {
        println!("2nd file provided: {}", input2.display());
    } else {
        println!("Only one file provided. Likely Single-End.");
    }

    // Prüfen ob Datei existiert
    if !Path::new(&args.input1).exists() {
        println!(
            "Fehler: Die Datei {} existiert nicht.",
            args.input1.display()
        ); //eprintln! ? equivalent to println! except output goes to io::stderrr instead of io::stdout
        return;
    }

    if let Some(ref input2) = args.input2 {
        if !Path::new(input2).exists() {
            println!("Fehler: Die Datei {} existiert nicht.", input2.display());
            return;
        }
    }

    // Reads aus der Datei einlesen
    let reads = match read_fastq(&args.input1) {
        Ok(reads) => reads,
        Err(e) => {
            eprintln!("Fehler beim Einlesen der Datei: {}", e);
            return;
        }
    };
    let avg_qualities = avg_base_quality(reads);
    print_quality_table(avg_qualities);

    // Geben Sie die Informationen über den Dateinamen aus
    let file_info: String = explain_filename(&args.input1.to_string_lossy()); // convert PathBuf to String; alternative adapt explain functions
    println!("{}", file_info);
    // explain_filename(&args.input2);
    if let Some(ref input2) = args.input2 {
        let file_info2: String = explain_filename(&input2.to_string_lossy());
        println!("{}", file_info2);
    }


}
