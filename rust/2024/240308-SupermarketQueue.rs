// https://www.codewars.com/kata/57b06f90e298a7b53d000a86

/**
 * fn queue_time(customers: &[u32], n: u32) -> u32 {
    let mut q: Vec<u32> = vec![0; n as usize];
    customers.iter().for_each(|&x| {
        *q.iter_mut().min().unwrap() += x;
    });
    *q.iter().max().unwrap()
}
 */

#[cfg(test)]
mod tests {
  fn queue_time(customers: &[u32], n: u32) -> u32 {
    if customers.is_empty() {
      return 0;
    }

    let mut proc_queue = vec![0; n as usize];
    let mut customer_index: usize = 0;

    let mut counter: u32 = 0;

    loop {
      if customer_index < customers.len() {
        for i in 0..proc_queue.len() {
          if proc_queue[i] == 0 && customer_index < customers.len() {
            proc_queue[i] = customers[customer_index];
            customer_index += 1;
          }
        }
      }

      for i in 0..proc_queue.len() {
        if proc_queue[i] > 0 {
          proc_queue[i] -= 1;
        }
      }

      counter += 1;

      if customer_index == customers.len() && proc_queue.iter().sum::<u32>() == 0 {
        break;
      }
    }

    counter
  }

  #[test]
  fn returns_expected() {
    assert_eq!(queue_time(&[5,3,4], 1), 12);
    assert_eq!(queue_time(&[10,2,3,3], 2), 10);
    assert_eq!(queue_time(&[2,3,10], 2), 12);
  }

  #[test]
  fn test_simple_case() {
    assert_eq!(queue_time(&[1,2], 1), 3);
    assert_eq!(queue_time(&[1,2], 2), 2);
  }

  #[test]
  fn test_mut_vector() {
    let mut input = vec![0; 5];

    input[2] += 1;

    assert_eq!(input[2], 1);
  }

  #[test]
  fn test_vector_fixed_size() {
    let input = vec![0; 3];

    assert_eq!(input, [0,0,0]);
  }
}