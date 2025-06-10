fn main() {
    md_print();
}

fn f_prime_x_64(h: f64) -> f64 {
    (f64::sin(1.0 + h) - f64::sin(1.0)) / h
}

const fn gen_inv_powers_of_two() -> [f64; 30] { let mut arr = [0.0; 30];

    let mut i = 0;
    while i < 30  {
        arr[i] = 1.0 / (1u64 << (i + 1)) as f64;
        i += 1;
    }
    arr
}

const INV_POWS_OF_TWO: [f64; 30] = gen_inv_powers_of_two();

fn md_print() {
    const X: f64 = 1.0;
    let cosx: f64 = X.cos();

    println!("|  h   |       x       | Approx. f'(x) |  Known f'(x)  |  Abs. Error   |");
    println!("|:----:|--------------:|--------------:|--------------:|--------------:|");
    for (i, pow2) in INV_POWS_OF_TWO.iter().enumerate() {
        let approx = f_prime_x_64(*pow2);
        let abserr = (approx - cosx).abs();
        println!("|2^-{:02} |{:>14.8} |{:>14.8} |{:>14.8} |{:>14.8} |",
            i, X, approx, cosx, abserr);
    }
}
