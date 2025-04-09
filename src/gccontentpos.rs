use crate::statistics::avproportion::BaseCounts;
use crate::runner::*;
use serde_json::json;

impl ToJson for GCContentPosStatistic {
    fn to_json(&self) -> String {
        json!({ 
            "gc_content_per_position": self.results() 
    }).to_string()
    }
}

pub struct GCContentPosStatistic {
    base_counts: Vec<BaseCounts>,
    read_count: usize,
    max_length: usize,
}

impl GCContentPosStatistic {
    pub fn new() -> Self {
        Self {
            base_counts: Vec::new(),
            read_count: 0,
            max_length: 0,
        }
    }

    fn results(&self) -> Vec<f64> {
        if self.read_count == 0 {
            return Vec::new();
        }
        self.base_counts
            .iter()
            .map(|counts| {
                let total = counts.get_total();
                if total > 0 {
                    (counts.c + counts.g) as f64 / total as f64
                } else {
                    0.0
                }
            })
            .collect()
    }
}

impl Statistic for GCContentPosStatistic {
    fn process(&mut self, record: &FastqRecord) {
        let read_length = record.seq.len();

        if read_length > self.max_length {
            self.max_length = read_length;
            self.base_counts.resize_with(read_length, BaseCounts::new);
        }

        for (i, &base) in record.seq.iter().enumerate() {
            self.base_counts[i].update(base);
        }

        self.read_count += 1;
    }
}

#[cfg(test)]
mod test {
    use super::GCContentPosStatistic;
    use crate::runner::{FastqRecord, Statistic};

    fn test_reads() -> Vec<FastqRecord> {
        vec![
            FastqRecord { seq: b"ACGT".to_vec(), qual: b"IIII".to_vec() },
            FastqRecord { seq: b"GGCC".to_vec(), qual: b"IIII".to_vec() },
            FastqRecord { seq: b"ATGC".to_vec(), qual: b"IIII".to_vec() },
        ]
    }

    #[test]
    fn test_gc_content_pos_statistic() {
        let mut statistic = GCContentPosStatistic::new();
        let reads = test_reads();

        for record in reads.iter() {
            statistic.process(record);
        }

        let results = statistic.results();
        let expected: Vec<f64> = vec![0.333, 0.666, 1.0, 0.666];

        for (computed, expected) in results.iter().zip(expected.iter()) {
            assert!((computed - expected).abs() < 0.01, "Mismatch at position");
        }
    }
}
