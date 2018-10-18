/// [Which x for that sum?](https://www.codewars.com/kata/which-x-for-that-sum/rust)
/// Sn=x+2x^2+3X^3+……+nx^n = -[x^(n+1)-x]/(x-1)^2-nx^(n+1)/(1-x) = x/(x*x -2x+1) = m   x-2+1/x = 1/y

fn solve(m: f64) -> f64 {
    let y = (2.0 * m + 1.0) / (2.0 * m);
    let x = y - (y * y - 1.0).sqrt();
    println!("{}-------------{}",m, x - 2.0 + 1.0 / x);
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_fuzzy(m: f64, expect: f64) -> () {
        let merr = 1e-12;
        println!("{:?}", m);
        let actual = solve(m);
        println!("Actual {:e}", actual);
        println!("Expect {:e}", expect);
        let inrange = (actual - expect).abs() <= merr;
        if inrange == false {
            println!("Expected value near: {:e} but got: {:e}", actual, expect);
        }
        assert_eq!(inrange, true);
    }

    #[test]
    fn basic_tests() {
        assert_fuzzy(2.00, 5.000000000000e-01);
        assert_fuzzy(4.00, 6.096117967978e-01);
        assert_fuzzy(5.00, 6.417424305044e-01);
    }
}
