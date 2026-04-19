use std::ops::Add;

fn main() {
    let c1 = Complex::new(1.0, 1f64);
    let c2 = Complex::new(2 as f64, 3.0);
    println!("{:?}", c1 + c2);

    let c3 = Complex::new(1.0, 1f64);
    let c4 = Complex::new(2 as f64, 3.0);
    println!("c3 is {:?}, c4 is {:?}, add is {:?}", c3, c4, &c3 + &c4);

    let c5 = Complex::new(1.0, 1f64);
    println!("c5 is {:?}, add is {:?}", c5, &c5 + 3.16);
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imag = self.imag + rhs.imag;
        Self::new(real, imag)
    }
}

impl Add for &Complex {
    // 此时的 Self 是 &Complex
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imag = self.imag + rhs.imag;
        Complex::new(real, imag)
    }
}

impl Add<f64> for &Complex {
    type Output = Complex;

    fn add(self, rhs: f64) -> Self::Output {
        let real = self.real + rhs;
        Complex::new(real, self.imag)
    }
}
