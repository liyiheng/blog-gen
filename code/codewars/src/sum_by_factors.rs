fn sum_of_divided(l: Vec<i64>) -> Vec<(i64, i64)> {
    let max = l
        .iter()
        .fold(0, |max, i| if i.abs() > max { i.abs() } else { max });
    let mut result: Vec<(i64, Option<i64>)> =
        get_primes(max).into_iter().map(|p| (p, None)).collect();
    for (p, sum) in result.iter_mut() {
        let factors: Vec<&i64> = l.iter().filter(|i| *i % *p == 0).collect();
        if !factors.is_empty() {
            *sum = Some(factors.iter().map(|r| *r).sum())
        }
    }
    result
        .into_iter()
        .filter(|(_, sum)| sum.is_some())
        .map(|(p, sum)| (p, sum.unwrap()))
        .collect()
}

fn get_primes(mut n: i64) -> Vec<i64> {
    if n < 0 {
        n = -n;
    }
    let n = n + 1;
    match n {
        0 | 1 => vec![],
        2 => vec![2],
        _ => {
            let mut result = vec![2];
            for i in 3..n {
                let mut is_prime = true;
                for p in result.iter() {
                    if *p > i / 2 {
                        break;
                    }
                    if i % p == 0 {
                        is_prime = false;
                        break;
                    }
                }
                if is_prime {
                    result.push(i);
                }
            }
            result
        }
    }
}

fn testing(l: Vec<i64>, exp: Vec<(i64, i64)>) -> () {
    assert_eq!(sum_of_divided(l), exp)
}

//#[test]
fn basics_sum_of_divided() {
    testing(
        vec![12, 15, 209],
        vec![(2, 12), (3, 27), (5, 15), (107, 209)],
    );
    testing(
        vec![15, 21, 24, 30, 45],
        vec![(2, 54), (3, 135), (5, 90), (7, 21)],
    );
}
