/*

Here is how a garden gnome sorts a line of flower pots.
Basically, he looks at the flower pot next to him and the previous one;
  if they are in the right order he steps one pot forward.
  otherwise, he swaps them and steps one pot backward.

Boundary conditions: if there is no previous pot, he steps forwards; if there is no pot next to him, he is done. 

*/

use std::time::Instant;

pub fn gnome_sort(mut arr: Vec<i32>) -> (Vec<i32>, f64) {
  let mut pos = 0;

  let timer = Instant::now();

  while pos < arr.len() {

    if (pos == 0) || (arr[pos] >= arr[pos -1]) {
      pos += 1;
    }
    else {
      let temp = arr[pos];
      arr[pos] = arr[pos -1];
      arr[pos -1] = temp;
      pos -= 1;
    }
  }

  (arr, timer.elapsed().as_secs_f64())
}