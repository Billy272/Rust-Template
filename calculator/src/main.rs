use std::env::{args, Args};

fn main() {
  let args: Args = args();

  let first = args.nth(2),unwrap(); 
  println!("{:?}", args);
}
