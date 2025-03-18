// 4.5 
// Übung 1

// 1) Konvertieren Sie die ASCII-Symbole für die Basen des folgenden Reads in Phred-Scores
// schreiben Sie eine korrekte Version der Funktion

// if it is out of range ?!
pub fn calculate_phred(qual: char) -> usize {
    (qual as u8 - 33) as usize
}


// 2) berechnen Sie den durchschnittlichen Phred-Score dieser Basen

// Schreiben Sie dann eine Funktion für read_qual und testen Sie diese Funktion

// String can include weird characters, better taking a sclice of bytes ?!

pub fn read_qual(qual_string: &String)-> f64 {
    let n = qual_string.len();
    let mut qual_sum = 0; 
    for qual in qual_string.chars() { // iterate over the characters of the string
        let q = calculate_phred(qual);
        qual_sum +=q
    }
    return (qual_sum / n) as f64
}


// 3) Verwenden Sie dann die Gleichung, um die durchschnittliche Fehlerwahrscheinlichkeit zu berechnens

pub fn avg_err_prob(phred: usize) -> f64 {
    let x = (10 as f64).powf(-(phred as f64) / 10.0); //10f64
    let rounded = (x * 100.0).round() / 100.0;
    return rounded
}


// Sequenzqualität (pro Base)
// Geben Sie eine Tabelle aus mit der durchschnittlichen Qualität (Phred-Wert) für jede Position der Reads.

// 1st draft
pub fn avg_base_quality(reads: Vec<(String, String, String, String)>) -> Vec<usize>{
    
    let read_length: usize = reads[0].1.len(); // Annahme: alle Reads haben die gleiche Länge // find alternative!
    let number_of_reads: usize = reads.len();

    // vector saving sums of for each position
    let mut quality_sum: Vec<usize> = vec![0; read_length];

    for read in reads {
        let qualtiy_string = &read.3; // phred scores in 4th line of read
        for(i, ch) in qualtiy_string.chars().enumerate(){
            let phred_value = calculate_phred(ch);
            quality_sum[i] += phred_value;
        }
    }
    // divide the sums by the number_of_reads
    // quality_sum.iter() iterates over quality_sum vector
    // collect() collects results of map
    quality_sum.iter().map(|&sum| sum / number_of_reads).collect()
  
    
}





#[cfg(test)]
mod test {

    use super::avg_base_quality;
    use super::calculate_phred;
    use super::read_qual;
    use super::avg_err_prob;


    #[test]

    fn test_avg_base_quality(){

        let test_reads = vec![
            ("@read1".to_string(), "ACGT".to_string(), "+".to_string(), "IIII".to_string()),  // 40, 40, 40, 40
            ("@read2".to_string(), "ACGT".to_string(), "+".to_string(), "HHHH".to_string()),  // 39, 39, 39, 39
            ("@read3".to_string(), "ACGT".to_string(), "+".to_string(), "GGGG".to_string()),  // 38, 38, 38, 38
        ];

        let expected = vec![39, 39, 39, 39];

        let res = avg_base_quality(test_reads);

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
        let qual_string: String = "!&+".to_string(); //.to_string()
        let expected: f64 = 5.0; 
        let res = read_qual(&qual_string); // reference: can be large, avoid copying
        assert_eq!(expected, res);
    }
    
    #[test]
    fn test_calculate_phred() {
        let qual: char = '&';
        let expected:usize =	5; // Phred-Score für das Zeichen '&'
        let res = calculate_phred(qual);
        assert_eq!(expected, res);
    }

    #[test]
    fn test_calculate_phred_range() {
        for ascii in 33..=126 {  // Alle ASCII-Werte von '!' bis '~'
            let qual = ascii as u8 as char;
            let expected = ascii - 33;
            assert_eq!(calculate_phred(qual), expected as usize);
        }
}


    // Alternativ können viele Ergebnisse auf einmal getestet werden:
    #[test]
    fn test_calculate_phred_other() {
        let tests: Vec<(char, usize)> = vec![
            ('&', 5),
            ('+', 10)
        ];
        for test in tests {
            let res = calculate_phred(test.0);
            assert_eq!(test.1, res);
        }
    }
    // ggf. andere Testfunktionen
}