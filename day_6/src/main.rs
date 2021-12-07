use nalgebra::{base::{SMatrix, SVector}, ArrayStorage, Complex};
use problem::{solve_main, CSV, Problem};

pub fn simulate(initial: &[usize], duration: usize) -> u64 {
    let mut count = [0; 9];
    for i in initial.iter() {
        count[*i] += 1;
    }

    for _ in 0..duration {
        let zeros = count[0];
        for i in 0..8 {
            count[i] = count[i + 1];
        }
        count[8] = zeros;
        count[6] += zeros;
    }

    count.iter().sum()
}

const ZERO: Complex<f64> = Complex { re: 0.0, im: 0.0 };
const ONE: Complex<f64> = Complex { re: 1.0, im: 0.0 };
const TWO: Complex<f64> = Complex { re: 2.0, im: 0.0 };

// Roots of the characteristic polynomial x^9 - x^2 - 1
// x≈-0.996130622055441 - 0.417311836335793 i
// x≈-0.996130622055441 + 0.417311836335793 i
// x≈-0.379213980654811 - 0.892877546086168 i
// x≈-0.379213980654811 + 0.892877546086168 i
// x≈0.095754469006120 - 0.870198718672104 i
// x≈0.095754469006120 + 0.870198718672104 i
// x≈0.734077898463753 - 0.742065121962188 i
// x≈0.734077898463753 + 0.742065121962188 i
// x≈1.0910244704807567604
const ROOTS: [Complex<f64>; 9] = [
    Complex { re: -0.996130622055441, im:  0.417311836335793 },
    Complex { re: -0.996130622055441, im: -0.417311836335793 },
    Complex { re: -0.379213980654811, im:  0.892877546086168 },
    Complex { re: -0.379213980654811, im: -0.892877546086168 },
    Complex { re:  0.095754469006120, im:  0.870198718672104 },
    Complex { re:  0.095754469006120, im: -0.870198718672104 },
    Complex { re:  0.734077898463753, im:  0.742065121962188 },
    Complex { re:  0.734077898463753, im: -0.742065121962188 },
    Complex { re:  1.0910244704807567604, im: 0.0 },
];

// Used to calculate the values of COEFFS
pub fn calc_coeffs() -> SVector<Complex<f64>, 9> {
    let m = SMatrix::<Complex<f64>, 9, 9>::from_fn(|r, c| ROOTS[c].powu(r as u32));
    let b = SVector::<Complex<f64>, 9>::from([ONE, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ONE, ZERO]);
    let decomp = m.lu();
    decomp.solve(&b).expect("Linear resolution failed.")
}

pub fn calculate(initial: &[usize], duration: usize) -> u64 {
    #[inline]
    fn f(roots: &[Complex<f64>; 9], coeffs: &SVector<Complex<f64>, 9>, day: i32) -> Complex<f64> {
        let mut result = ZERO;
        for i in 0..9 {
            result += coeffs[i] * roots[i].powi(day);
        }
        result
    }

    #[inline]
    fn g(roots: &[Complex<f64>; 9], coeffs: &SVector<Complex<f64>, 9>, day: i32) -> Complex<f64> {
        TWO * f(roots, coeffs, day - 1)
        + TWO * f(roots, coeffs, day - 2)
        + TWO * f(roots, coeffs, day - 3)
        + TWO * f(roots, coeffs, day - 4)
        + TWO * f(roots, coeffs, day - 5)
        + TWO * f(roots, coeffs, day - 6)
        + TWO * f(roots, coeffs, day - 7)
        + f(roots, coeffs, day - 8)
        + f(roots, coeffs, day - 9)
    }

    let mut count = [0; 9];
    for i in initial.iter() {
        count[*i] += 1;
    }

    const COEFFS: SVector<Complex<f64>, 9> = SVector::from_array_storage(
        ArrayStorage([[
            Complex { re: 0.1260768114963526, im: -0.00663123683235536 },
            Complex { re: 0.12607681149635266, im: 0.006631236832355397 },
            Complex { re: 0.11279107390309294, im: -0.032200935222245805 },
            Complex { re: 0.11279107390309287, im: 0.032200935222245756 },
            Complex { re: 0.07359404243842448, im: 0.021469402001632148 },
            Complex { re: 0.07359404243842455, im: -0.02146940200163227 },
            Complex { re: 0.12435173095411725, im: 0.015825935818472307 }, 
            Complex { re: 0.12435173095411729, im: -0.015825935818472303 },
            Complex { re: 0.12637268241602523, im: -0.00000000000000000021747809549865233 }
        ]])
    );

    let result = count.iter().enumerate().map(|(i, n)| g(&ROOTS, &COEFFS, duration as i32 - i as i32) * *n as f64).sum::<Complex<f64>>();
    result.re as u64
}

struct Day6;

impl Problem for Day6 {
    type Input = CSV<usize>;
    type PartOne = u64;
    type PartTwo = u64;

    fn solve_part_one(input: &Self::Input) -> Self::PartOne {
        simulate(input.values(), 80)
        // calculate(input.values(), 80)
    }

    fn solve_part_two(input: &Self::Input) -> Self::PartTwo {
        simulate(input.values(), 256)
        // calculate(input.values(), 256)
    }
}

fn main() {
    solve_main::<Day6>();
}
