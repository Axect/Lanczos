extern crate peroxide;
use peroxide::*;

const G: f64 = 5f64;
const N: usize = 8;

fn main() {
    b_gen(N).print();
    c_gen(N).print();
    dc_gen(N).print();
    dr_gen(N).print();
    f_gen(G,N).print();
    lanczos_coeff(G, N).print();
    tlg1(G, N).print();
}

fn tlg1(g: f64, n: usize) -> Matrix {
    lanczos_coeff(g, n-1) * g.exp() / (2f64 * std::f64::consts::PI).sqrt()
}

fn lanczos_coeff(g: f64, n: usize) -> Matrix {
    let m = dr_gen(n) * b_gen(n) * (c_gen(n) * dc_gen(n));
    let f = f_gen(g, n);
    m * f
}

fn b_gen(n: usize) -> Matrix {
    Matrix::from_index(
        |i, j| {
            if i == 0 {
                1f64
            } else if j >= i {
                sgn(j - i) * C(i+j-1, j-i) as f64  
            } else {
                0f64
            }
        },
        (n, n)
    )
}

fn c_gen(n: usize) -> Matrix {
    Matrix::from_index(
        |i, j| {
            if i == 0 && j == 0 {
                0.5
            } else if j > i {
                0f64
            } else {
                sgn(i-j) * 4f64.powi(j as i32) * (i as f64) * (C(i+j, 2*j) as f64) / (i+j) as f64
            }
        },
        (n, n)
    )
}

fn dc_gen(n: usize) -> Matrix {
    let mut m = zeros(n, n);
    m[(0,0)] = 2f64;
    for i in 1 .. n {
        m[(i,i)] = 2f64 * double_factorial(2*i-1) as f64;
    }
    m
}

fn dr_gen(n: usize) -> Matrix {
    let mut m = zeros(n, n);
    m[(0,0)] = 1f64;
    for i in 1 .. n {
        m[(i,i)] = - ((i * C(2*i-1, i)) as f64);
    }
    m
}

fn f(g: f64, n: usize) -> f64 {
    2f64.sqrt() * (n as f64 + 0.5).exp() / (2f64 * (n as f64 + g) + 1f64).powf(n as f64 + 0.5)
}

fn f_gen(g: f64, n: usize) -> Vec<f64> {
    let mut v = vec![0f64; n];
    for i in 0 .. n {
        v[i] = f(g, i);
    }
    v
}

fn sgn(x: usize) -> f64 {
    if x % 2 == 0 {
        1f64
    } else {
        -1f64
    }
}

fn double_factorial(n: usize) -> usize {
    let mut s = 1usize;
    let mut n = n;
    while n >= 2 {
        s *= n;
        n -= 2;
    }
    s
}
