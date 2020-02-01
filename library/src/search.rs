pub fn binary_search<T: std::cmp::PartialOrd>(x: T, v: &[T]) -> bool {
    let mut l = 0;
    let mut r = v.len();

    while r > l {
        let i = (l + r) / 2;
        if v[i] == x { return true; }
        else if v[i] < x { l = i + 1; }
        else { r = i; }
    }

    false
}