use notyourfactor::*;

mod notyourfactor;

fn main() {
  let mut factors = Factors {
    factors: [0; 20],
    nums: 0,
  };
  table_division(6, &mut factors);
  for i in 0..factors.nums {
    println!("{}", factors.factors[i as usize]);
  }
}
