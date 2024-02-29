// https://www.codewars.com/kata/5bc7bb444be9774f100000c3

#[cfg(test)]
mod tests {
  #[derive(Clone)]
  struct VersionUnit {
    major: u32,
    minor: u32,
    patch: u32,
  }
  struct VersionManager {
    current: VersionUnit,
    history: Vec<VersionUnit>,
  }

  impl VersionManager {
    fn new() -> Self {
      Self {
        current: VersionUnit{
          major: 0,
          minor: 0,
          patch: 1,
        },
        history: vec![]
      }
    }

    fn from_version(version: &str) -> Result<Self, &'static str> {
      if version == "" {
        return Ok(VersionManager::new());
      }

      let mut iter = version.split(".").into_iter();

      let major = iter.next().unwrap().parse::<u32>();
      if major.is_err() {
        return Err("invalid");
      }
      let minor = iter.next().unwrap_or("0").parse::<u32>();
      if minor.is_err() {
        return Err("invalid");
      }
      let patch = iter.next().unwrap_or("0").parse::<u32>();
      if patch.is_err() {
        return Err("invalid");
      }

      Ok(Self {
        current: VersionUnit {
          major: major.unwrap(),
          minor: minor.unwrap(),
          patch: patch.unwrap(), 
        },
        history: vec![]
      })
    }

    fn major(&mut self) -> &mut Self {
      self.history.push(self.current.clone());

      self.current.major += 1;
      self.current.minor = 0;
      self.current.patch = 0;

      self
    }

    fn minor(&mut self) -> &mut Self {
      self.history.push(self.current.clone());

      self.current.minor += 1;
      self.current.patch = 0;

      self
    }

    fn patch(&mut self) -> &mut Self {
      self.history.push(self.current.clone());

      self.current.patch += 1;

      self
    }

    fn rollback(&mut self) -> Result<&mut Self, &'static str> {
      if self.history.is_empty() {
        return Err("history empty")
      }

      let prev = self.history.pop().unwrap();
      self.current.major = prev.major;
      self.current.minor = prev.minor;
      self.current.patch = prev.patch;

      Ok(self)
    }

    fn release(&self) -> String {
      format!("{}.{}.{}", self.current.major, self.current.minor, self.current.patch)
    }
  }

  #[test]
  fn test_invalid_version() {
    let result = VersionManager::from_version("1.b.2");

    // println!("{}", result.unwrap().release());
    assert_eq!(result.is_ok(), false);
  }
  
  #[test]
  fn test_initial() {
    let instance = VersionManager::new();
    let version = instance.release();

    assert_eq!(version.as_str(), "0.0.1");
  }

  #[test]
  fn test_rollback() {
    let major_rollback = VersionManager::new().major().rollback().unwrap().release();
    let minor_rollback = VersionManager::new().minor().rollback().unwrap().release();

    assert_eq!(major_rollback.as_str(), "0.0.1");
    assert_eq!(minor_rollback.as_str(), "0.0.1");
  }

  #[test]
  fn test_fromversion() {
    let instance = VersionManager::from_version(&"1.2.3.4.5").unwrap();

    assert_eq!(instance.release().as_str(), "1.2.3");
  }

  #[test]
  fn test_general_case() {
    let result = VersionManager::new().major().minor().patch().major().patch().release();

    assert_eq!(result.as_str(), "2.0.1");
  }

  #[test]
  fn test_parse_version() {
    let input = &"1.2.3.4.5";

    let mut iter = input.split(".").into_iter();
    let major = iter.next().unwrap();
    let minor = iter.next().unwrap();
    let patch = iter.next().unwrap();

    assert_eq!(format!("{}.{}.{}", major, minor, patch), "1.2.3");
  }

  #[test]
  fn test_parse_whole() {
    let input = &"10";

    let mut iter = input.split(".").into_iter();
    assert_eq!(iter.next().unwrap(), "10");
  }

  #[test]
  fn test_vector_previous() {
    let mut input = vec![1,2,3];
    input.push(4);
    input.pop();

    assert_eq!(input, vec![1,2,3]);
  }

  #[test]
  fn returns_expected() {
    assert_eq!(1, 1);
  }
}
