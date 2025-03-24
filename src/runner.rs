use std::io::{self, BufRead};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FastqRecord {
    seq: Vec<u8>,
    qual: Vec<u8>,
}

pub trait Statistic {
    /* Statistics:
     * average base quality (Phred)
     * average quality of all reads
     * average proportions of `{A, C, G, T, N}` for each read position
     * ...
     */

    fn process(&mut self, record: &FastqRecord);

    // TODO - find a way to represent the results.
    // Let's try to identify the shared parts of *any* statistic
    // and report these in some fashion.
    // fn report(self) -> ?
}

/// Computes mean base quality for a position read.
pub struct BaseQualityPosStatistic {
    position_avg_qualities: Vec<f64>,
    
}

impl BaseQualityPosStatistic {
    pub fn new() -> Self {
        Self {
            position_avg_qualities: Vec::new(),
        }
    }

    pub fn results(&self) -> &Vec<f64> {
        &self.position_avg_qualities
    }
}

impl Statistic for BaseQualityPosStatistic {
    fn process(&mut self, record: &FastqRecord) {
        let reads = vec![( String::new(), record.seq.clone(), String::new(), record.qual.clone())];

    self.position_avg_qualities = avg_base_quality(reads);
    }
}

/// Computes mean base quality for a read.
pub struct ReadQualityStatistic {
    read_avg_qualities: Vec<f64>,
}
impl ReadQualityStatistic{
    pub fn new() -> Self{
        Self{
            read_avg_qualities:Vec::new(),
        }
    }

    pub fn result(&self) -> &Vec<f64>{
        &self.read_avg_qualities
    }
}

impl Statistic for ReadQualityStatistic {
    fn process(&mut self, record: &FastqRecord) {
        let avg_quality = read_qual(&record.qual);
        self.read_avg_qualities.push(avg_quality);
    }
}

pub struct WorkflowRunner {
    statistics: Vec<Box<dyn Statistic>>,
}

impl WorkflowRunner {
    /// Process the FASTQ file.
    ///
    /// Can return an I/O error or other errors (not in the signature at this point)
    pub fn process<R>(&mut self, mut read: R)
    where
        R: BufRead,
    {
        let mut record = FastqRecord::default();

        while let Ok(()) = WorkflowRunner::parse_record(&mut read, &mut record) {
            for statistic in self.statistics.iter_mut() {
                statistic.process(&record);
            }
        }
    }

    // Read data for a complete FASTQ record from `read`.
    pub fn parse_record<R>(read: &mut R, record: &mut FastqRecord) -> io::Result<()>
    where
        R: BufRead,
    {
        // unimplemented!() // TODO: implement
        
        let mut _id = String::new(); // unused, intentionally ignored
        let mut seq = String::new();
        let mut _plus = String::new(); //unused, intentionally ignored
        let mut qual = String::new();

        if read.read_line(&mut _id)? == 0 { //? handels errors
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "No more records"));
        }
        read.read_line(&mut seq)?;
        read.read_line(&mut _plus)?;
        read.read_line(&mut qual)?;

        record.seq = seq.trim().as_bytes().to_vec();
        record.qual = qual.trim().as_bytes().to_vec();

        Ok(())
    }

    pub fn finalize(self) -> Vec<Box<dyn Statistic>> {
        // Move out the statistics, effectively preventing the future use of the runner.
        self.statistics
    }
}