fn main() {
    let n = 10;
    println!("fib({n}) = {}", fib(n));

    let n = 20;
    println!("fib2({n}) = {}", fib2(n));
}

fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

fn fib2(n: u64) -> u64 {
    if n < 2 {
        return n;
    }
    let (mut fn2, mut fn1) = (0, 1);
    for _ in 2..=n {
        (fn1, fn2) = (fn1 + fn2, fn1);
    }
    fn1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_0() {
        assert_eq!(fib(0), 0);
    }

    #[test]
    fn fib2_0() {
        assert_eq!(fib2(0), 0);
    }

    #[test]
    fn fib_1() {
        assert_eq!(fib(1), 1);
    }

    #[test]
    fn fib2_1() {
        assert_eq!(fib2(1), 1);
    }

    #[test]
    fn fib_2() {
        assert_eq!(fib(2), 1);
    }

    #[test]
    fn fib2_2() {
        assert_eq!(fib2(2), 1);
    }

    #[test]
    fn fib_3() {
        assert_eq!(fib(3), 2);
    }

    #[test]
    fn fib2_3() {
        assert_eq!(fib2(3), 2);
    }

    #[test]
    fn fib_4() {
        assert_eq!(fib(4), 3);
    }

    #[test]
    fn fib2_4() {
        assert_eq!(fib2(4), 3);
    }

    #[test]
    fn fib_5() {
        assert_eq!(fib(5), 5);
    }

    #[test]
    fn fib2_5() {
        assert_eq!(fib2(5), 5);
    }

    #[test]
    fn fib_7() {
        assert_eq!(fib(7), 13);
    }

    #[test]
    fn fib2_7() {
        assert_eq!(fib2(7), 13);
    }

    #[test]
    fn fib_10() {
        assert_eq!(fib(10), 55);
    }

    #[test]
    fn fib2_10() {
        assert_eq!(fib2(10), 55);
    }

    #[test]
    fn fib_12() {
        assert_eq!(fib(12), 144);
    }

    #[test]
    fn fib2_12() {
        assert_eq!(fib2(12), 144);
    }

    #[test]
    fn fib2_17() {
        assert_eq!(fib2(17), 1597);
    }

    #[test]
    fn fib2_20() {
        assert_eq!(fib2(20), 6765);
    }

    #[test]
    fn fib2_37() {
        assert_eq!(fib2(37), 24_157_817);
    }

    #[test]
    fn fib2_50() {
        assert_eq!(fib2(50), 12_586_269_025);
    }

    #[test]
    fn fib2_77() {
        assert_eq!(fib2(77), 5_527_939_700_884_757);
    }
}
