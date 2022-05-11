use std::char;
use std::num::ParseIntError;
use std::u8;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_num() {
    let input = "81df";

    assert_eq!("81df", rlp_transform(input));
  }

  #[test]
  fn test_string() {
    let input = "83017d45";

    assert_eq!("83457d01", rlp_transform(input));
  }

  #[test]
  fn test_string_2() {
    let input = "83017d45";

    assert_eq!("83457d01", rlp_transform(input));
  }

  #[test]
  fn test_struct() {
    let input = "cb8277658673616c75746575";

    assert_eq!("cb758673616c757465827765", rlp_transform(input));
  }
}

fn str_to_bytes(input: &str) -> Result<Vec<u8>, ParseIntError> {
  (0..input.len())
    .step_by(2)
    .map(|i| u8::from_str_radix(&input[i..i + 2], 16))
    .collect()
}

fn bytes_to_str(in_bytes: &[u8]) -> String {
  let mut results = String::new();

  for byte in in_bytes.iter() {
    results.push(char::from_digit((byte >> 4) as u32, 16).unwrap());
    results.push(char::from_digit((byte & 0xf) as u32, 16).unwrap());
  }
  results
}

fn be_to_le(in_bytes: &mut [u8]) {
  for i in (0..in_bytes.len() - 1).step_by(2) {
    in_bytes.swap(i, i + 1);
  }
}

fn reverse(in_bytes: &mut [u8]) {
  in_bytes.reverse();
}

fn reverse_list(in_bytes: &[u8]) -> Vec<u8> {
  let mut output = Vec::new();

  let mut structs: Vec<&[u8]> = Vec::new();

  let mut index = 0;
  while index < in_bytes.len() {
    // println!("index:{}", index);
    // num
    if in_bytes[index] <= 0x7f {
      structs.push(&in_bytes[index..index + 1]);
      index += 1;
      continue;
    }
    // string len<55
    else if in_bytes[index] <= 0xb7 {
      let len = in_bytes[index] as usize - 0x80;
      structs.push(&in_bytes[index..index + len + 1]);
      index += 1 + len;
      continue;
    }
    // string len>55
    else if in_bytes[index] <= 0xbf {
      let len_of_len: usize = in_bytes[index] as usize - 0xb7;
      let mut len: usize = 0;
      for (i, item) in in_bytes[index + 1..(index + 1 + len_of_len)]
        .iter()
        .enumerate()
      {
        len += (item << (i * 4)) as usize;
      }
      structs.push(&in_bytes[index..index + len_of_len + len + 1]);
      index += 1 + (len_of_len + len);
      continue;
    }
  }

  // println!("{}", structs.len());

  for &st in structs.iter().rev() {
    for &item in st {
      // print!("{:x} ", item);
      output.push(item);
    }
    // print!("   ");
  }

  output
}

pub fn rlp_transform(input: &str) -> String {
  let mut in_bytes: Vec<u8> = str_to_bytes(&input).unwrap();

  // num
  if in_bytes[0] <= 0x7f {
    be_to_le(&mut in_bytes[1..]);
  }
  // string len<55
  else if in_bytes[0] <= 0xb7 {
    reverse(&mut in_bytes[1..]);
  }
  // string len>55
  else if in_bytes[0] <= 0xbf {
    let len_of_len = in_bytes[0] - 0xb7;
    reverse(&mut in_bytes[1 + len_of_len as usize..]);
  }
  // struct
  else if in_bytes[0] <= 0xf7 {
    let mut tmp = reverse_list(&in_bytes[1..]);

    in_bytes = vec![in_bytes[0]];
    in_bytes.append(&mut tmp);
  }
  // struct len>55
  else {
    let len_of_len = in_bytes[0] as usize - 0xf7;

    let mut tmp = reverse_list(&in_bytes[1 + len_of_len..]);

    in_bytes = vec![in_bytes[0]];
    in_bytes.append(&mut tmp);
  }

  bytes_to_str(&in_bytes)
}
