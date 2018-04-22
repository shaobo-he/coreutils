use numeric::is_prime;

mod numeric;

extern { fn prime_p(n: u64) -> bool; }

fn main() {
  let n = 10;
  let g = unsafe {prime_p(n)};
  let t = is_prime(n);
  assert!(g == t);
}
