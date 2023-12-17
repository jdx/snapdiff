use crate::file::ContentsHash;
use crate::snapshot::Snapshot;
use crate::snapshot_1::Comparison::{Identical, Modified};
use crate::{file, stats};
use file::File;
use stats::Stats;
use std::collections::HashMap;
use std::path;

#[derive(Debug)]
pub struct Snapshot1 {
    files_by_path: HashMap<path::PathBuf, File>,
    total: Stats,
}

impl Snapshot for Snapshot1 {
    fn add(&mut self, f1: File) {
        self.total.record_file(&f1);
        if self.files_by_path.contains_key(&f1.path) {
            panic!("Added duplicate file")
        }
        self.files_by_path.insert(f1.path.clone(), f1);
    }
}

pub enum Comparison {
    Identical,
    Modified,
}

impl Snapshot1 {
    pub fn new() -> Snapshot1 {
        return Snapshot1 {
            files_by_path: HashMap::new(),
            total: Stats::new(),
        };
    }

    pub fn digest(&mut self, f2: &File) -> Option<(Comparison, File)> {
        return self.files_by_path.remove(&f2.path).map(|f1| {
            if f2.checksum == f1.checksum {
                (Identical, f1)
            } else {
                (Modified, f1)
            }
        });
    }

    pub fn conclude(&mut self) -> (Stats, HashMap<ContentsHash, Vec<File>>) {
        let mut files_by_hash: HashMap<ContentsHash, Vec<File>> = HashMap::new();

        for (_, f) in self.files_by_path.drain() {
            if !files_by_hash.contains_key(&f.checksum) {
                files_by_hash.insert(f.checksum, vec![]);
            }
            files_by_hash.get_mut(&f.checksum).unwrap().push(f);
        }
        return (self.total, files_by_hash);
    }
}
