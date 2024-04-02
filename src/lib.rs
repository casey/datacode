//! Datacode encoding and decoding.

/// Encode byte slice to datacode string
pub fn encode(bytes: &[u8]) -> String {
  let mut datacode = String::new();
  for chunk in bytes.chunks(2) {
    if chunk.len() == 2 {
      let low = u32::from(chunk[0]);
      let high = u32::from(chunk[1]) << 8;
      let value = 0x40000 | high | low;
      datacode.push(char::try_from(value).unwrap());
    } else {
      let value = 0x1FF00 | u32::from(chunk[0]);
      datacode.push(char::try_from(value).unwrap());
    }
  }
  datacode
}

/// Decode datacode string to byte vector
pub fn decode(datacode: &str) -> Result<Vec<u8>, String> {
  let mut bytes = Vec::new();
  for c in datacode.chars() {
    let value = u32::from(c);
    match value {
      0x40000..=0x4FFFF => {
        bytes.extend_from_slice(&value.to_le_bytes()[0..2]);
      }
      0x1FF00..=0x1FFFF => {
        bytes.push(value.to_le_bytes()[0]);
      }
      _ => return Err(format!("invalid character `{c}`")),
    }
  }
  Ok(bytes)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn encoder() {
    assert_eq!(encode(&[]), "");
    assert_eq!(encode(&[0]), "\u{1FF00}");
    assert_eq!(encode(&[1]), "\u{1FF01}");
    assert_eq!(encode(&[255]), "\u{1FFFF}");
    assert_eq!(encode(&[0, 0]), "\u{40000}");
    assert_eq!(encode(&[1, 0]), "\u{40001}");
    assert_eq!(encode(&[255, 0]), "\u{400FF}");
    assert_eq!(encode(&[255, 1]), "\u{401FF}");
    assert_eq!(encode(&[255, 255]), "\u{4FFFF}");
    assert_eq!(encode(&[0, 0, 0]), "\u{40000}\u{1FF00}");
    assert_eq!(encode(&[0, 0, 1]), "\u{40000}\u{1FF01}");
    assert_eq!(encode(&[255, 255, 255]), "\u{4FFFF}\u{1FFFF}");
  }

  #[test]
  fn decoder() {
    assert_eq!(decode(""), Ok(vec![]));
    assert_eq!(decode("\u{1FF00}"), Ok(vec![0]));
    assert_eq!(decode("\u{1FF01}"), Ok(vec![1]));
    assert_eq!(decode("\u{1FFFF}"), Ok(vec![255]));
    assert_eq!(decode("\u{40000}"), Ok(vec![0, 0]));
    assert_eq!(decode("\u{40001}"), Ok(vec![1, 0]));
    assert_eq!(decode("\u{400FF}"), Ok(vec![255, 0]));
    assert_eq!(decode("\u{401FF}"), Ok(vec![255, 1]));
    assert_eq!(decode("\u{4FFFF}"), Ok(vec![255, 255]));
    assert_eq!(decode("\u{40000}\u{1FF00}"), Ok(vec![0, 0, 0]));
    assert_eq!(decode("\u{40000}\u{1FF01}"), Ok(vec![0, 0, 1]));
    assert_eq!(decode("\u{4FFFF}\u{1FFFF}"), Ok(vec![255, 255, 255]));
    assert_eq!(decode("hello"), Err("invalid character `h`".into()));
  }
}
