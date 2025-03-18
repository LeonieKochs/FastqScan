use explain_name::explain_filename;
use fastq::{avg_err_prob, calculate_phred};
use clap::Parser;
use std::fs;
use std::path::Path;

mod fastq;
mod explain_name;

#[derive(Parser)]

// comand line interface struct
struct Cli {
    input1: String,
    input2:Option<String>,
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
    println!("1st file probided:{}", args.input1);
    
    // 
    if let Some(ref input2) = args.input2 { // wrapped in Option<String>
        println!("2nd file provided: {}", input2); // input2 is now a regular String
    } else {
        println!("Only one file probided. Likely Single-End.");
    }


    // Prüfen ob Datei existiert
    if !Path::new(&args.input1).exists(){
        println!("Fehler: Die Datei {} existiert nicht.", args.input1); //eprintln! ? equivalent to println! except output goes to io::stderrr instead of io::stdout
        return;
    }

    if let Some(ref input2) = args.input2 { // use of moved value !!!
        if !Path::new(input2).exists(){
            println!("Fehler: Die Datei {} existiert nicht.", input2);
            return;
        }
    }

    // Geben Sie die Informationen über den Dateinamen aus
    let file_info = explain_filename(&args.input1); //explain_name::explain_filename() ?
    println!("{}", file_info);
    // 2nd ?
    // explain_filename(&args.input2);
    if let Some(ref input2) = args.input2 {
        let file_info2:String = explain_filename(&input2);
        println!("{}", file_info2);
    }



}


