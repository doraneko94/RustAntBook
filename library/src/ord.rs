use std::cmp::Ordering;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Rev<T>(pub T);

impl<T: PartialOrd> PartialOrd for Rev<T> {
    fn partial_cmp(&self, other: &Rev<T>) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<T: Ord> Ord for Rev<T> {
    fn cmp(&self, other: &Rev<T>) -> Ordering {
        other.0.cmp(&self.0)
    }
}

pub fn lower_bound<T: Ord>(sorted: &[T], x: &T) -> usize {
    let mut low = 0;
    let mut high = sorted.len();

    while low != high {
        let mid = (low + high) / 2;
        match sorted[mid].cmp(x) {
            Ordering::Less => low = mid + 1,
            Ordering::Greater | Ordering::Equal => high = mid,
        };
    }

    low
}

pub fn upper_bound<T: Ord>(sorted: &[T], x: &T) -> usize {
    let mut low = 0;
    let mut high = sorted.len();

    while low != high {
        let mid = (low + high) / 2;
        match sorted[mid].cmp(x) {
            Ordering::Less | Ordering::Equal => low = mid + 1,
            Ordering::Greater => high = mid,
        };
    }

    low
}