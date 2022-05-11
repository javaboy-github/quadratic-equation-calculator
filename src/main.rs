use fraction;
use fraction::CheckedMul;
use core::str::FromStr;
use std::env;
use std::error::Error;
use std::fmt;

type F = fraction::Fraction;

trait Sqrt {
    fn sqrt(&self) -> Self;
}

impl Sqrt for F {
    fn sqrt(&self) -> Self {
        match self {
            F::Rational(sign, ratio) => {
                F::Rational(*sign, fraction::Ratio::new_raw((*ratio.numer() as f64).sqrt() as u64, (*ratio.denom() as f64).sqrt() as u64))
            },
            _ => F::clone(&self),
        }
    }
}

#[derive(Debug, Clone)]
struct ArgmentsAreNotSatified;
impl fmt::Display for ArgmentsAreNotSatified {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

// This is important for other errors to wrap this one.
impl Error for ArgmentsAreNotSatified {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        // 基本となるエラー、原因は記録されていない。
        None
    }
}


fn get_args() -> Result<(F, F, F), &'static dyn Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        Err(&ArgmentsAreNotSatified{})
    } else {
        Ok((F::from_str(&args[1]).unwrap(), F::from_str(&args[2]).unwrap(), F::from_str(&args[3]).unwrap()))
    }
}

fn calc(a: F, b:F, c: F) -> (F, F) {
    let x1 = (-b + (b.checked_mul(&b).unwrap() - F::from(4u64).checked_mul(&a).unwrap().checked_mul(&c).unwrap()).sqrt()) / (F::from(2u64) * a);
    let x2 = (-b - (b.checked_mul(&b).unwrap() - F::from(4u64).checked_mul(&a).unwrap().checked_mul(&c).unwrap()).sqrt()) / (F::from(2u64) * a);
    (x1, x2)
}

fn to_math_str(a:F, b:F, c:F) -> String {
    let mut result = String::from("");
    if let F::Rational(_, ratio) = a {
        if *ratio.numer() != 0{
            if *ratio.numer() != 1 {
                result += &format!("{}", a);
            }
            result += "x^2";
        }

    }
    if let F::Rational(_, ratio) = b {
        if *ratio.numer() != 0{
            if result != "" {
                result += " + ";
            }
            if *ratio.numer() != 1 {
                result += &format!("{}", b);
            }
            result += "x";
        }
    }
    if let F::Rational(_, ratio) = c {
        if *ratio.numer() != 0{
            if result != "" {
                result += " + ";
            }
            result += &format!("{}", c);
        }
    }
    if result != "" {
        result += " = 0";
    }
    result
}

fn main() {
    let (a, b, c) = get_args().unwrap();
    println!("{}", to_math_str(a, b, c));
    let (x1, x2) = calc(a, b, c);
    println!("x={}, {}", x1, x2);
}
