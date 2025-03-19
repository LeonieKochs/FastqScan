use explain_name::explain_filename;
use fastq::{avg_err_prob, calculate_phred};
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};

mod fastq;
mod explain_name;

#[derive(Parser)]

// comand line interface struct
struct Cli {
    input1: PathBuf,
    input2:Option<PathBuf>,
}


fn main() {

    /* 
    FASTQ-Analyse mit Rust: Übung 4
    println!("Übung 4: \nStellen wir uns einen Read zu 1000bb vor, bei dem alle Basen einen Phred-Score von & aufweisen. Berechen Sie die erwartete Anzahl von Fehlern für den Read (unter der Unabhängigkeitsannahme). ");

    let phred_score: usize = calculate_phred('&');
    let error_prob: f64 = avg_err_prob(phred_score);
    let expected_err: f64 = error_prob * 1000.0;

    println!("Erwartete Fehleranzahl im Read: {}", expected_err);
    */


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
    if !Path::new(&args.input1).exists(){
        println!("Fehler: Die Datei {} existiert nicht.", args.input1.display()); //eprintln! ? equivalent to println! except output goes to io::stderrr instead of io::stdout
        return;
    }

    if let Some(ref input2) = args.input2 { 
        if !Path::new(input2).exists(){
            println!("Fehler: Die Datei {} existiert nicht.", input2.display());
            return;
        }
    }

    // Geben Sie die Informationen über den Dateinamen aus
    let file_info:String = explain_filename(&args.input1.to_string_lossy()); // convert PathBuf to String; alternative adapt explain functions
    println!("{}", file_info);
    // 2nd ?
    // explain_filename(&args.input2);
    if let Some(ref input2) = args.input2 {
        let file_info2:String = explain_filename(&input2.to_string_lossy());
        println!("{}", file_info2);
    }



}


