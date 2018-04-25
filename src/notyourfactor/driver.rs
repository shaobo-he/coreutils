use notyourfactor::*;

mod notyourfactor;

#[repr(C)]
struct c_factors {
  pub plarge: [u64;2],
  pub p: [u64;26],
  pub e: [u8;26],
  pub nfactors: u8
}

extern { fn factor(t1: u64, t0: u64, factors: *mut c_factors); }

fn main() {
  let mut factors = Factors {
    factors: [0; 20],
    nums: 0,
  };
  let n = 540;
  // GNU
  let t1: u64 = 0;
  let t0: u64 = n;
  let mut gnu_factors = c_factors{plarge: [0;2], p: [0;26], e: [0;26], nfactors: 0};	
  unsafe { factor(t1, t0, &mut gnu_factors) };

  // Rust
  table_division(n, &mut factors);

  let mut gnu_num_factors = 0;
  for i in 0..26 {
    gnu_num_factors += gnu_factors.e[i];
  }

  println!("{} {}", factors.nums, gnu_factors.nfactors);
  assert!(factors.nums == gnu_num_factors);
  for i in 0..factors.nums {
    println!("Rust: factor_{}={}", i, factors.factors[i as usize]);

    // GNU
    println!("GNU:  factor_{}={}", i, gnu_factors.p[i as usize]);
  }
}
