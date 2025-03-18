// 4.5 

// Übung 2

// Schreiben Sie ein kleines Rust-Programm, um die in dem folgenden Datei-Namen enthaltenen Information zu erklären.

// @HWI-D00107:50:H6BP8ACWV:5:2204:10131:51624 2:N:0:AGGCAGAA ???
// {sample_name}_{barcode_sequence}_L{lane}_R{read_number}_{set_number}.fastq.gz 
// NIST7035_TAAGGCGA_L001_R1_001.fastq.gz

// make a struct?

pub fn explain_filename(filename: &str) -> String {
    
    let filename = filename.trim_end_matches(".fastq.gz");
    
    // Teilen an "_"
    // substrings collected into vector of string slices
    // parts = ["NIST7035", "TAAGGCGA", "L001", "R1", "001"]
    let parts: Vec<&str> = filename.split('_').collect();
    
    if parts.len() == 5 {
        let sample_name = parts[0];
        let barcode_sequence = parts[1];
        let lane = parts[2].trim_start_matches("L"); 
        let read_number = parts[3].trim_start_matches("R"); 
        let set_number = parts[4];

        //String erstellen, umformen
        return format!(
            "Sample Name: {}\nBarcode Sequence: {}\nLane: {}\nRead Number: {}\nSet Number: {}",
            sample_name, barcode_sequence, lane, read_number, set_number
        );
    } else {
        // Turning static string into String (return type)
        return "Invalid filename format.".to_string();
    }
}

// Übung 3

// Schreiben Sie ein kleines Rust-Programm, um die in dem folgenden Read-Namen enthaltenen Information zu erklären.

// @Machine42:1:FC7:7:19:4229:1044 1:N:0:TTAGGC
// @HWI-D00107:50:H6BP8ACWV:5:2204:10131:51624 2:N:0:AGGCAGAA

// make a struct?

pub fn explain_readname(readname: &str) -> String {

    let readname: &str = readname.trim_start_matches('@');

    let parts: Vec<&str> = readname.split(|c| c == ':' || c == ' ').collect();

    if parts.len() == 11 {
        let instrument = parts[0];
        let run: &str = parts[1];
        let flowcell_id = parts[2];
        let lane: &str = parts[3];
        let tile: &str = parts[4];
        let x_pos: &str = parts[5];
        let y_pos: &str = parts[6];
        let read: &str = parts[7];
        let is_filtered: &str = parts[8];
        let control_number: &str = parts[9];
        let index: &str = parts[10]; 

        return format!(
            "Instrument: {}\nRun: {}\nFlowcell_ID: {}\nLane: {}\n Tile: {}\nx-pos: {}\ny-pos: {}\nRead: {}\nis filtered: {}\nControl Number: {}\nIndex: {}",
             instrument, run, flowcell_id, lane, tile, x_pos, y_pos, read, is_filtered, control_number, index
        );

    } else { 
        return String::from("Invalid Format!");
    }

}





#[cfg(test)]

mod test {
    use std::slice::EscapeAscii;
    use super::{explain_filename, explain_readname};

    #[test]

    fn test_explain_readname(){
        let readname: &str = "@Machine42:1:FC7:7:19:4229:1044 1:N:0:TTAGGC";
        let expected: &str = "Instrument: Machine42\nRun: 1\nFlowcell_ID: FC7\nLane: 7\n Tile: 19\nx-pos: 4229\ny-pos: 1044\nRead: 1\nis filtered: N\nControl Number: 0\nIndex: TTAGGC";
        let res: String = explain_readname(readname);
        assert_eq!(expected, res);
    }

    #[test]
    fn test_invalid_format() {
        let readname = "@InvalidReadName";
        let expected = "Invalid Format!";
        let res = explain_readname(readname);
        assert_eq!(expected, res);
    }

    #[test]

    fn test_explain_filename(){
        let filename = "NIST7035_TAAGGCGA_L001_R1_001.fastq.gz";
        let expected: &str = "Sample Name: NIST7035\nBarcode Sequence: TAAGGCGA\nLane: 001\nRead Number: 1\nSet Number: 001";        
        let res: String = explain_filename(filename);
        assert_eq!(expected, res);
    }
}

