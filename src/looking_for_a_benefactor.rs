/// ### [Looking for a benefactor](https://www.codewars.com/kata/569b5cec755dd3534d00000f/train/rust)
/// The accounts of the \"Fat to Fit Club (FFC)\" association are supervised by John as a volunteered accountant.
/// The association is funded through financial donations from generous benefactors. John has a list of
/// the first `n` donations: `[14, 30, 5, 7, 9, 11, 15]`
/// He wants to know how much the next benefactor should give to the association so that the 
/// average of the first `n + 1` donations should reach an average of `30`.
/// After doing the math he found `149`. He thinks that he made a mistake.
/// Could you help him?
///
/// if `dons = [14, 30, 5, 7, 9, 11, 15]` then `new_avg(dons, 30) --> 149`
///
/// The function `new_avg(arr, navg)` should return the expected donation
/// (rounded up to the next integer) that will permit to reach the average `navg`. 
///
/// Should the last donation be a non positive number `(<= 0)` John wants us to throw an error 
///
/// (return Nothing in Haskell, return None in F# and Ocaml, return `-1` in C, Fortran,  Nim, echo `ERROR`in Shell) 
///
/// so that he clearly sees that his expectations are not great enough.
///
/// Notes: 
///
/// - all donations and `navg` are numbers (integers or floats depending on the language), `arr` can be empty.
/// - See examples below and \"Test Samples\" to see which error is to be thrown.
///
/// ```
/// new_avg([14, 30, 5, 7, 9, 11, 15], 92) should return 645
/// new_avg([14, 30, 5, 7, 9, 11, 15], 2) 
/// should raise an error (ValueError or invalid_argument or DomainError) 
/// or return `-1` or ERROR depending on the language
/// ```

use std::time::Instant;


fn new_avg(arr: &[f64], newavg: f64) -> Option<i32> {
    unimplemented!()
}


fn testing(arr: &[f64], newavg: f64, exp: Option<i32>) -> () {
    assert_eq!(exp, new_avg(arr, newavg))
}

#[test]
fn basic_tests() {
    let a1 = [14.0, 30.0, 5.0, 7.0, 9.0, 11.0, 16.0];
    testing(&a1, 90.0, Some(628));
    let a2 = [14.0, 30.0, 5.0, 7.0, 9.0, 11.0, 15.0];
    testing(&a2, 92.0, Some(645));
    let a3 = [14.0, 30.0, 5.0, 7.0, 9.0, 11.0, 15.0];
    testing(&a3, 2.0, None);
    let a4 = [14000.25, 300.76, 50.56, 70.0, 90.0, 11.0, 150.48, 1200.98];
    testing(&a4, 4800.0, Some(27326));
}
