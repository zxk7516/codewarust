// my bad solution.
fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    // let mut x = vec![0; ints.len()];
    // &x.clone_from_slice(ints);
    let mut y = ints.iter().enumerate().collect::<Vec<(usize, &i8)>>();
    y.sort_by(|a, b| a.1.cmp(b.1));
    let mut left = 0;
    let mut right = y.len() - 1;
    let mut r = vec![];
    loop {
        let sum = y[left].1 + y[right].1;
        match sum {
            sum if sum == s => {
                if y[left].0 > y[right].0 {
                    r.push((y[right], y[left]));
                } else {
                    r.push((y[left], y[right]));
                }
                left = left + 1;
            }
            sum if sum > s => {
                right = right - 1;
            }
            sum if sum < s => {
                left = left + 1;
            }
            _ => {}
        };
        if left >= right {
            break;
        }
    }
    if r.len() == 0{
        return None;
    }else if  r.len() == 1{
        return Some( ( *(r[0].0).1, *(r[0].1).1) );
    }else{
        r.sort_by(|a, b| (a.1).0.cmp( &(b.1).0 ) );
        return Some( ( *(r[0].0).1, *(r[0].1).1) );
    }
    
}

use std::collections::HashSet;

fn sum_pairs2(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    let mut seen = HashSet::new();
    for &i in ints {
        if seen.contains(&(s - i)) {
            return Some((s - i, i));
        }
        seen.insert(i);
    }
    None
}

fn sum_pairs3(ints: &[i8], s: i8) -> Option<(i8, i8)> {
  
  let mut nums = ints.to_vec();
  nums.dedup();
  
  for j in 1 .. nums.len() {
    for i in 0 .. j {
      if nums[i] + nums[j] == s { return Some((nums[i], nums[j])); }
    }
  }
  
  None
}



#[test]
fn returns_expected() {
    let l1 = [1, 4, 8, 7, 3, 15];
    let l2 = [1, -2, 3, 0, -6, 1];
    let l3 = [20, -13, 40];
    let l4 = [1, 2, 3, 4, 1, 0];
    let l5 = [10, 5, 2, 3, 7, 5];
    let l6 = [4, -2, 3, 3, 4];
    let l7 = [0, 2, 0];
    let l8 = [5, 9, 13, -3];
    assert_eq!(sum_pairs(&l1, 8), Some((1, 7)));
    assert_eq!(sum_pairs(&l2, -6), Some((0, -6)));
    assert_eq!(sum_pairs(&l3, -7), None);
    assert_eq!(sum_pairs(&l4, 2), Some((1, 1)));
    assert_eq!(sum_pairs(&l5, 10), Some((3, 7)));
    assert_eq!(sum_pairs(&l6, 8), Some((4, 4)));
    assert_eq!(sum_pairs(&l7, 0), Some((0, 0)));
    assert_eq!(sum_pairs(&l8, 10), Some((13, -3)));
}
