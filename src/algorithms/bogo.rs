use rand::prelude::*;

pub fn bogo_sort(mut arr: Vec<i32>) -> (Vec<i32>, f64) {
  let mut shuffle_count = 0;

  while !sorted(arr.clone()) {
      arr = shuffle(arr);
      //println!("Shuffle #{}: {:?}", shuffle_count, arr);
      shuffle_count += 1;
  }

  (arr, shuffle_count as f64)
}

fn shuffle(arr: Vec<i32>) -> Vec<i32> {
  let mut new = arr;
  let mut rng = rand::thread_rng();

  for i in 0..new.len() {
      let y = f64::floor(rng.gen::<f64>() * new.len() as f64) as usize;

      let temp = new[y];
      new[y] = new[i];
      new[i] = temp;
  }

  new
}

fn sorted(arr: Vec<i32>) -> bool {
  for i in 0..arr.len() -1 {
      if arr[i] > arr[i+1] {
          return false;
      }
  }
  true
}