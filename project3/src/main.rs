fn prime_factors(num : i64) -> Vec<i64> {
    let mut n: i64 = num;
    let mut prs: Vec<i64> = Vec::new();
    if n % 2 == 0 {
        prs.push(2);
        n = n /2;
    }
    let mut i: i64 = 3;
    loop{
        if i*i > n {
            break;
        }

        loop {
            if n % i != 0 {
                break;
            }
            prs.push(i);
            n = n / i;
        }
        i = i + 2
    }

    if n > 2 {
        prs.push(n);
    }

    return prs;
} 

fn highest_factor(prs: Vec<i64>) -> i64 {
    let mut hi: i64 = 0;
    for i in prs {
        if i > hi {
            hi = i
        }
    }
    return hi
}

fn main() {
    let prs: Vec<i64> =  prime_factors(600851475143);
    let high: i64 = highest_factor(prs);
    println!("{:?}", high)

}
