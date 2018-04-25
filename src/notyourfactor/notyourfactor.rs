//use numeric::*;

//mod numeric;
use std::u64::MAX as MAX_U64;
use std::num::Wrapping;

pub struct Factors {
  pub factors: [u64; 20],
  pub nums: u8,
}

impl Factors {
  fn add_factor(&mut self, factor: u64) {
    if !(self.nums != 0 && self.factors[(self.nums - 1) as usize] == factor) {
      self.factors[self.nums as usize] = factor;
      self.nums += 1;
    }
  }
}

pub fn table_division(mut num: u64, factors: &mut Factors) {
    if num < 2 {
        return;
    }
    while num % 2 == 0 {
        num /= 2;
        factors.add_factor(2);
    }
    if num == 1 {
        return;
    }
    if is_prime(num) {
        factors.add_factor(num);
        return;
    }
    for &(prime, inv, ceil) in P_INVS_U64 {
        if num == 1 {
            break;
        }

        // inv = prime^-1 mod 2^64
        // ceil = floor((2^64-1) / prime)
        // if (num * inv) mod 2^64 <= ceil, then prime divides num
        // See http://math.stackexchange.com/questions/1251327/
        // for a nice explanation.
        loop {
            let Wrapping(x) = Wrapping(num) * Wrapping(inv); // x = num * inv mod 2^64
            if x <= ceil {
                num = x;
                factors.add_factor(prime);
                if is_prime(num) {
                    factors.add_factor(num);
                    return;
                }
            } else {
                break;
            }
        }
    }

    // do we still have more factoring to do?
    // Decide whether to use Pollard Rho or slow divisibility based on
    // number's size:
    //if num >= 1 << 63 {
    // number is too big to use rho pollard without overflowing
    //trial_division_slow(num, factors);
    //} else if num > 1 {
    // number is still greater than 1, but not so big that we have to worry
    //rho_pollard_factor(num, factors);
    //}
}

pub const P_INVS_U64: &'static [(u64, u64, u64)] = &[
    (3, 12297829382473034411, 6148914691236517205), (5, 14757395258967641293, 3689348814741910323),
    (7, 7905747460161236407, 2635249153387078802), (11, 3353953467947191203, 1676976733973595601),
];

// computes (a + b) % m using the russian peasant algorithm
// CAUTION: Will overflow if m >= 2^63
pub fn sm_mul(mut a: u64, mut b: u64, m: u64) -> u64 {
    let mut result = 0;
    while b > 0 {
        if b & 1 != 0 {
            result = (result + a) % m;
        }
        a = (a << 1) % m;
        b >>= 1;
    }
    result
}

// computes (a + b) % m using the russian peasant algorithm
// Only necessary when m >= 2^63; otherwise, just wastes time.
pub fn big_mul(mut a: u64, mut b: u64, m: u64) -> u64 {
    // precompute 2^64 mod m, since we expect to wrap
    let Wrapping(msb_mod_m) = Wrapping(MAX_U64) - Wrapping(m) + Wrapping(1);
    let msb_mod_m = msb_mod_m % m;

    let mut result = 0;
    while b > 0 {
        if b & 1 != 0 {
            let Wrapping(next_res) = Wrapping(result) + Wrapping(a);
            let next_res = next_res % m;
            result = if result <= MAX_U64 - a {
                next_res
            } else {
                (next_res + msb_mod_m) % m
            };
        }
        let Wrapping(next_a) = Wrapping(a) << 1;
        let next_a = next_a % m;
        a = if a < 1 << 63 {
            next_a
        } else {
            (next_a + msb_mod_m) % m
        };
        b >>= 1;
    }
    result
}

// computes a.pow(b) % m
fn pow(mut a: u64, mut b: u64, m: u64, mul: fn(u64, u64, u64) -> u64) -> u64 {
    let mut result = 1;
    while b > 0 {
        if b & 1 != 0 {
            result = mul(result, a, m);
        }
        a = mul(a, a, m);
        b >>= 1;
    }
    result
}

fn witness(mut a: u64, exponent: u64, m: u64) -> bool {
    if a == 0 {
        return false;
    }

    let mul = if m < 1 << 63 {
        sm_mul as fn(u64, u64, u64) -> u64
    } else {
        big_mul as fn(u64, u64, u64) -> u64
    };

    if pow(a, m - 1, m, mul) != 1 {
        return true;
    }
    a = pow(a, exponent, m, mul);
    if a == 1 {
        return false;
    }
    loop {
        if a == 1 {
            return true;
        }
        if a == m - 1 {
            return false;
        }
        a = mul(a, a, m);
    }
}

// uses deterministic (i.e., fixed witness set) Miller-Rabin test
pub fn is_prime(num: u64) -> bool {
    if num < 2 {
        return false;
    }
    if num % 2 == 0 {
        return num == 2;
    }
    let mut exponent = num - 1;
    while exponent & 1 == 0 {
        exponent >>= 1;
    }

    // These witnesses detect all composites up to at least 2^64.
    // Discovered by Jim Sinclair, according to http://miller-rabin.appspot.com
    let witnesses = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];
    !witnesses
        .iter()
        .any(|&wit| witness(wit % num, exponent, num))
}
