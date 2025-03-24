use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use flate2::read::GzDecoder;
use prettytable::{Table, Row, Cell, format};
// 4.5
// Übung 1

// 1) Konvertieren Sie die ASCII-Symbole für die Basen des folgenden Reads in Phred-Scores
// schreiben Sie eine korrekte Version der Funktion

// if it is out of range ?!
// qual is byte, u32 -> 4 bytes is more than we need
// use u8
// Option: Return result: -> Result<f64, &static str>
pub fn calculate_phred(qual: u8) -> u8 {
    (qual - 33) as u8
}

// 2) berechnen Sie den durchschnittlichen Phred-Score dieser Basen

// Schreiben Sie dann eine Funktion für read_qual und testen Sie diese Funktion

// String can include weird characters, better taking a sclice of bytes ?!

pub fn read_qual(qual_string: &[u8]) -> f64 {
    let n: f64 = qual_string.len() as f64; // taking u8 casting into f64, potential panic!?
    let mut qual_sum: f64 = 0.0;
    for qual in qual_string {
        let q: u8 = calculate_phred(*qual); //*qual
        qual_sum += q as f64 // overflow keep in mind 
    }
    return qual_sum / n;
}

// 3) Verwenden Sie dann die Gleichung, um die durchschnittliche Fehlerwahrscheinlichkeit zu berechnens

pub fn avg_err_prob(phred: usize) -> f64 {
    let x = (10 as f64).powf(-(phred as f64) / 10.0); //10f64
    let rounded = (x * 100.0).round() / 100.0;
    return rounded;
}

// Sequenzqualität (pro Base)
// Geben Sie eine Tabelle aus mit der durchschnittlichen Qualität (Phred-Wert) für jede Position der Reads.

// add option for plain txt
pub fn read_fastq(filename: &Path) -> io::Result<Vec<(String, String, String, String)>> { //more flexible <P: AsRef<Path>>
    let file = File::open(&filename)?;
    let reader = BufReader::new(GzDecoder::new(file)); // add option for plain txt

    let mut reads = Vec::new();
    let mut lines = reader.lines();

    while let (Some(id), Some(seq), Some(plus), Some(qual)) = (
        lines.next().transpose()?,
        lines.next().transpose()?,
        lines.next().transpose()?,
        lines.next().transpose()?,
    ) {
        reads.push((id, seq, plus, qual));
    }

    Ok(reads)
}

// 1st draft
pub fn avg_base_quality(reads: Vec<(String, String, String, String)>) -> Vec<f64> {
    // in case reads is empty
    // -> number_of_reads != 0 (division in map)
    if reads.is_empty() {
        return Vec::new();
    }

    // seperate function or change requirments
    let read_length: usize = reads[0].1.len(); // Annahme: alle Reads haben die gleiche Länge // find alternative!
    let number_of_reads: u64 = (reads.len()) as u64;

    // vector saving sums of for each position
    let mut quality_sum: Vec<u64> = vec![0; read_length];

    for read in reads {
        let qualtiy_string: &[u8] = &read.3.as_bytes(); // phred scores in 4th line of read
        for (i, qual) in qualtiy_string.iter().enumerate() {
            let phred_value = calculate_phred(*qual);
            quality_sum[i] += phred_value as u64;
        }
    }
    // divide the sums by the number_of_reads
    // quality_sum.iter() iterates over quality_sum vector
    // collect() collects results of map
    // into_iter() owned values
    quality_sum
        .into_iter()
        .map(|sum: u64| sum as f64 / number_of_reads as f64)
        .collect() 
}




// Sequenzqualität (pro Sequenz)

// Geben Sie die durchschnittliche Qualität aller Reads aus.

pub fn avg_seq_quality(reads: Vec<(String, String, String, String)>) -> Vec<f64>{
    // in case reads is empty
    // -> number_of_reads != 0 (division in map)
    if reads.is_empty() {
        return Vec::new();
    }

    let mut avg_qualities = Vec::new();

    for read in reads.iter() {
        let quality_string = read.3.as_bytes(); // Extract quality scores as bytes
        let mut sum: u64 = 0;

        for &qual in quality_string.iter() {
            let phred_value = calculate_phred(qual);
            sum += phred_value as u64;
        }

        let avg_quality = sum as f64 / quality_string.len() as f64;
        avg_qualities.push(avg_quality);
    }

    avg_qualities
    
}


// Sequenzidentität (pro Base)

// Geben Sie die durchschnittlichen Anteile von A, C, G, T für jede Position der Reads aus.

// need to add rounding function
pub fn avg_base_proportion(reads: Vec<(String, String, String, String)>) -> Vec<(f64, f64, f64, f64)> {
    /*let solution: Vec<(f64, f64, f64, f64)> = vec![
        (1.0, 0.0, 0.0, 0.0), // Position 0 → 100% A
        (0.0, 1.0, 0.0, 0.0), // Position 1 → 100% C
        (0.0, 0.0, 1.0, 0.0), // Position 2 → 100% G
        (0.0, 0.0, 0.0, 1.0)];
    solution*/

    if reads.is_empty() {
        return Vec::new();
    }

    // read_length => max read
    let read_length = reads.iter()
        .map(|read| read.1.len())  
        .max()  
        .unwrap_or(0);

    let number_of_reads: f64 = (reads.len()) as f64;

    let mut base_counts: Vec<(f64, f64, f64, f64)> = vec![(0.0, 0.0, 0.0, 0.0); read_length];

    for read in reads {
        let seq = &read.1;

        for (i, base) in seq.chars().enumerate() {
            match base {
                'A' => base_counts[i].0 += 1.0,
                'C' => base_counts[i].1 += 1.0,
                'G' => base_counts[i].2 += 1.0,
                'T' => base_counts[i].3 += 1.0,
                _ => {}
            }
        }

        // add 0 for missing position of shorter reads
        for i in seq.len()..read_length{
            base_counts[i].0 += 0.0;
            base_counts[i].1 += 0.0;
            base_counts[i].2 += 0.0;
            base_counts[i].3 += 0.0;
        }
    }

    base_counts
        .iter()
        .map(|&(a, c, g, t)| (a as f64 / number_of_reads, c as f64 / number_of_reads, g as f64 / number_of_reads , t as f64 / number_of_reads))
        .collect()



}


#[cfg(test)]
mod test {

    use super::avg_base_proportion;
    use super::avg_base_quality;
    use super::avg_err_prob;
    use super::avg_seq_quality;
    use super::calculate_phred;
    use super::read_qual;

    fn test_reads() -> Vec<(String, String, String, String)> {
        vec![
            (
                "@read1".to_string(),
                "ACGT".to_string(),
                "+".to_string(),
                "IIII".to_string(),
            ),
            (
                "@read2".to_string(),
                "ACGT".to_string(),
                "+".to_string(),
                "HHHH".to_string(),
            ),
            (
                "@read3".to_string(),
                "ACGT".to_string(),
                "+".to_string(),
                "GGGG".to_string(),
            ),
        ]
    }


    #[test]

    fn test_avg_base_proportion(){

        let avg_base_proportions = avg_base_proportion(test_reads());

        let expected = vec![
            (1.0, 0.0, 0.0, 0.0), // Position 0 → 100% A
            (0.0, 1.0, 0.0, 0.0), // Position 1 → 100% C
            (0.0, 0.0, 1.0, 0.0), // Position 2 → 100% G
            (0.0, 0.0, 0.0, 1.0), // Position 3 → 100% T
        ];
        

        assert_eq!(avg_base_proportions, expected);
    }


    #[test]

    fn test_avg_seq_quality(){

        let avg_seq_qualities = avg_seq_quality(test_reads());
        let expected = vec![40.0, 39.0, 38.0];

        assert_eq!(avg_seq_qualities, expected);

    }


    #[test]

    fn test_avg_base_quality() {

        let expected: Vec<f64> = vec![39.0, 39.0, 39.0, 39.0];

        let res = avg_base_quality(test_reads());

        assert_eq!(expected, res);
    }

    #[test]
    fn test_avg_err_prob() {
        let phred: usize = 5;
        let expected: f64 = 0.32;
        let res = avg_err_prob(phred);
        assert_eq!(expected, res);
    }

    #[test]
    fn test_read_qual() {
        let qual_string: &[u8] = "!&+".as_bytes();
        let expected: f64 = 5.0;
        let res = read_qual(qual_string); // reference: can be large, avoid copying
        assert_eq!(expected, res);
    }

    #[test]
    fn test_calculate_phred() {
        let qual: u8 = b'&'; // byte representation
        let expected: u8 = 5; // Phred-Score für das Zeichen '&'
        let res = calculate_phred(qual);
        assert_eq!(expected, res);
    }

    #[test]
    fn test_calculate_phred_range() {
        for ascii in 33..=126 {
            // Alle ASCII-Werte von '!' bis '~'
            let qual = ascii as u8;
            let expected = (ascii - 33) as u8;
            assert_eq!(calculate_phred(qual), expected);
        }
    }

    // Alternativ können viele Ergebnisse auf einmal getestet werden:
    #[test]
    fn test_calculate_phred_other() {
        let tests: Vec<(u8, u8)> = vec![(b'&', 5), (b'+', 10)];
        for test in tests {
            let res = calculate_phred(test.0);
            assert_eq!(test.1, res);
        }
    }
    // ggf. andere Testfunktionen
}
