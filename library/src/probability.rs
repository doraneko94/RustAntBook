pub fn permutation<T: Clone + Copy>(v: &Vec<T>, m: usize, duplicate: bool) -> Vec<Vec<T>> {
    let n = v.len();
    let mut incl = m - 1;
    let mut v_tmp = vec![0; m];
    let mut ret = Vec::new();
    if duplicate {
        ret.push(vec![v[0]; m]);
    }
    while incl > 0 || v_tmp[0] < n - 1 {
        if v_tmp[incl] == n - 1 {
            v_tmp[incl] = 0;
            incl -= 1;
            continue;
        }
        v_tmp[incl] += 1;
        incl = m - 1;
        let mut flag = true;
        if !duplicate {
            let mut test = vec![0; n];
            for &i in v_tmp.iter() {
                if test[i] == 1 {
                    flag = false;
                    break;
                } else {
                    test[i] = 1;
                }
            }
        }
        if flag {
            ret.push(v_tmp.iter().map(|&index| v[index]).collect::<Vec<T>>());
        }
    }
    ret
}