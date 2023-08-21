use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::cmp::{Eq, Ord};
use std::collections::{hash_map, HashMap};
use std::fmt::{self, Debug, Display};
use std::hash::Hash;
//use csv::WriterBuilder;

#[derive(Serialize, Deserialize, Debug)]
pub struct Hist<T: Ord + Eq + Hash + Clone> {
    pub hist: HashMap<Option<T>, usize>,
}

impl<T: Ord + Eq + Hash + Clone> Hist<T> {
    pub fn new() -> Hist<T> {
        Hist {
            hist: HashMap::new(),
        }
    }

    pub fn add_dist(&mut self, d: Option<T>) {
        self.hist
            .entry(d)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    pub fn to_vec(&self) -> Vec<(Option<T>, usize)> {
        let mut h2 = self.hist.clone();
        let inf_rds = h2.remove(&None);
        let mut hvec: Vec<(Option<T>, usize)> = h2.iter().map(|(x, y)| (x.clone(), *y)).collect();
        hvec.sort_by(|a, b| a.0.cmp(&b.0));
        if let Some(cnt) = inf_rds {
            hvec.push((None, cnt));
        }
        hvec
    }

    pub fn check(&self, d: Option<T>) -> u64 {
        let value = self.hist.get(&d);
        if let Some(value) = value {
            *value as u64
        } else {
            0
        }
    }

    pub fn iter(&self) -> hash_map::Iter<Option<T>, usize> {
        self.hist.iter()
    }
}

impl<T: Ord + Eq + Hash + Clone> Default for Hist<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord + Eq + Hash + Clone> IntoIterator for Hist<T> {
    type Item = <HashMap<Option<T>, usize> as IntoIterator>::Item;
    type IntoIter = <HashMap<Option<T>, usize> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.hist.into_iter()
    }
}

impl<T: Ord + Eq + Clone + Hash + Debug + Display> fmt::Display for Hist<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut hvec = self.to_vec();
        let tot = hvec.iter().fold(0, |acc, x| acc + x.1);
        if !hvec.is_empty() {
            writeln!(f, "Reuse distance histogram:\n\t{} distance value(s), min {:?}, max {:?}\n\t{} accesses",
                     hvec.len(), hvec[0].0, hvec[hvec.len() - 1].0, tot)?;
            if hvec[hvec.len() - 1].0.is_none() {
                writeln!(f, "\t({} first accesses)", hvec[hvec.len() - 1].1)?;
                hvec.pop();
            }
        }
        writeln!(f, "value, count")?;
        hvec.into_iter()
            .fold(Ok(()), |_, (d, cnt)| writeln!(f, "{}, {}", d.unwrap(), cnt))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut h = Hist::new();
        h.add_dist(None);
        h.add_dist(Some(1));
        h.add_dist(Some(1));
        h.add_dist(Some(100));

        let v = h.to_vec();
        assert_eq!(v[0], (Some(1), 2));
        assert_eq!(v[1], (Some(100), 1));
        assert_eq!(v[2], (None, 1));

        assert_eq!(
            format!("{}", h),
            "Reuse distance histogram:
	3 distance value(s), min Some(1), max None
	4 accesses
	(1 first accesses)
value, count
1, 2
100, 1
"
        );
        // use cargo test -- --show-output to see the result
        println!("{}", h);
    }
}
