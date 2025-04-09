use crate::runner::Statistic;
use crate::runner::*;
use serde_json::json;

impl ToJson for BaseQualityPosStatistic {
    fn to_json(&self) -> String {
        json!({
            "avg_base_quality": self.results()
        })
        .to_string()
    }
}

pub fn calculate_phred(qual: u8) -> u8 {
    (qual - 33) as u8
}

#[derive(Debug, Clone)]
pub struct QualityCounts {
    sum_quality: u64, //Overflow?
    count: u64,
}

impl QualityCounts {
    pub fn new() -> Self {
        QualityCounts { sum_quality: 0, count: 0 }
    }

    pub fn update(&mut self, quality: u8) {
        self.sum_quality += quality as u64;
        self.count += 1;
    }

    pub fn get_average(&self) -> f64 {
        if self.count > 0 {
            self.sum_quality as f64 / self.count as f64
        } else {
            0.0
        }
    }
}

pub struct BaseQualityPosStatistic {
    quality_counts: Vec<QualityCounts>,
    max_read_length: usize,
}

impl BaseQualityPosStatistic {
    pub fn new() -> Self {
        Self {
            quality_counts: Vec::new(),
            max_read_length: 0,
        }
    }

    pub fn results(&self) -> Vec<f64> {
        // iterate over all QualityCounts (sum_quality, counts)
        self.quality_counts.iter().map(|qc| qc.get_average()).collect()
    }
}

impl Statistic for BaseQualityPosStatistic {
    fn process(&mut self, record: &FastqRecord) {
        let read_length = record.qual.len();
        if read_length > self.max_read_length {
            self.quality_counts.resize_with(read_length, QualityCounts::new); // sum_quality: 0, count: 0
            self.max_read_length = read_length;
        }

        for (i, &qual) in record.qual.iter().enumerate() {
            let phred_value = calculate_phred(qual);
            self.quality_counts[i].update(phred_value);
        }
    }
}

#[cfg(test)]
mod test {
    use super::{calculate_phred, BaseQualityPosStatistic};
    use crate::runner::{FastqRecord, Statistic};

    fn test_reads() -> Vec<FastqRecord> {
        vec![
            FastqRecord {
                seq: b"ACGT".to_vec(),
                qual: b"AAA!".to_vec(),
            },
            FastqRecord {
                seq: b"ACGT".to_vec(),
                qual: b"BBB!".to_vec(),
            },
            FastqRecord {
                seq: b"ACGT".to_vec(),
                qual: b"xxx!".to_vec(),
            },
        ]
    }

    #[test]
    fn test_base_quality_pos_statistic() {
        let mut statistic = BaseQualityPosStatistic::new();
        let reads = test_reads();

        for record in reads.iter() {
            statistic.process(record);
        }

        let results = statistic.results();
        let expected: Vec<f64> = vec![50.666666666666664, 50.666666666666664, 50.666666666666664, 0.0];
        assert_eq!(results, expected);
    }

    #[test]
    fn test_calculate_phred() {
        let qual: u8 = b'&';
        let expected: u8 = 5;
        let res = calculate_phred(qual);
        assert_eq!(expected, res);
    }
}
