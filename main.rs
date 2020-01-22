use std::io;
  2 
  3 fn main() {
  4     println!("Please enter the number of iterations for fibonnaci sequence:");
  5     let mut number = String::new();
  6     io::stdin().read_line(&mut number).expect("failed to read line");
  7     let value:u8 = number.trim().parse().expect("failed to parse string to u8");
  8     println!("FIBBONACCI SEQUENCE COMPUTED UNTIL ITERATION {}", value);
  9     fib(1,1,value);
 10 
 11 }
 12 
 13 fn fib(prev: u128, current: u128, itr: u8){
 14     let newcurrent = prev + current;
 15     let itr = itr - 1;
 16     println!("{}", current);
 17     if itr > 1 {
 18         fib(current, newcurrent, itr);
 19     }
 20 }
