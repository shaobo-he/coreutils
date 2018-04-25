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
  
  let t1: u64 = 0;
  let t0    : u64 = 6;
  let mut c_factors = c_factors{plarge: [0;2], p: [0;26], e: [0;26], nfactors: 0};	
  unsafe { factor(t1, t0, &mut c_factors) };


  table_division(6, &mut factors);
  for i in 0..factors.nums {
    println!("{}", factors.factors[i as usize]);
  }
}
