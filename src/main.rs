use rand::prelude::*;

use algorithms::{bogo::bogo_sort, gnome::gnome_sort};

mod algorithms;

const ARR_LEN: usize = 16384;
const MAX_NUM: i32 = i32::MAX;

fn main() {
  let arr: Vec<i32> = populate();
  // println!("--- UNSORTED ARRAY ---\n{:?}\n----------------------\n", arr);


  // print_stats("BOGO", bogo_sort(arr.clone()));
  
  
  let gnome = gnome_sort(arr);
  print_stats("GNOME", (None, gnome.1));
  
}

fn print_stats(algo: &str, stats: (Option<Vec<i32>>, f64)) {
  println!("--- {} --- \n Took {} Shuffles / Seconds. \n {:?} \n--------{}\n",algo, stats.1, stats.0, "-".repeat(algo.len()));
}


fn populate() -> Vec<i32> {
  let mut new = Vec::new();
  let mut rng = rand::thread_rng();

  for i in 0..ARR_LEN {
      // let val = f64::floor(rng.gen::<f64>() * MAX_NUM as f64) as i32;
      new.push(i as i32);
  }
  
  new.shuffle(&mut rng);

  new
}