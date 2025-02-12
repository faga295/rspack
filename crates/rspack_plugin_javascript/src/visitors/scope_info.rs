use rustc_hash::FxHashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeInfoId(u32);

const TOMBSTONE: ScopeInfoId = ScopeInfoId(0);
const UNDEFINED: ScopeInfoId = ScopeInfoId(1);

impl ScopeInfoId {
  fn init() -> ScopeInfoId {
    ScopeInfoId(2)
  }
}

#[derive(Debug)]
pub struct ScopeInfoDB {
  count: ScopeInfoId,
  map: FxHashMap<ScopeInfoId, ScopeInfo>,
}

impl ScopeInfoDB {
  fn next(&mut self) -> ScopeInfoId {
    let id = self.count;
    self.count.0 += 1;
    id
  }

  pub fn new() -> Self {
    Self {
      count: ScopeInfoId::init(),
      map: Default::default(),
    }
  }

  fn _create(&mut self, parent: Option<&ScopeInfoId>) -> ScopeInfoId {
    let id = self.next();
    let stack = match parent {
      Some(parent) => {
        let mut parnet_stack = self.expect_get(parent).stack.clone();
        parnet_stack.push(id);
        parnet_stack
      }
      None => vec![id],
    };
    let is_strict = match parent {
      Some(parent) => self.expect_get(parent).is_strict,
      None => false,
    };
    let info = ScopeInfo {
      is_strict,
      stack,
      map: Default::default(),
    };
    let prev = self.map.insert(id, info);
    assert!(prev.is_none());
    id
  }

  pub fn create(&mut self) -> ScopeInfoId {
    self._create(None)
  }

  pub fn create_child(&mut self, parent: &ScopeInfoId) -> ScopeInfoId {
    self._create(Some(parent))
  }

  pub fn expect_get(&self, id: &ScopeInfoId) -> &ScopeInfo {
    self
      .map
      .get(id)
      .unwrap_or_else(|| panic!("{id:#?} should exist"))
  }

  pub fn expect_get_mut(&mut self, id: &ScopeInfoId) -> &mut ScopeInfo {
    self
      .map
      .get_mut(id)
      .unwrap_or_else(|| panic!("{id:#?} should exist"))
  }

  pub fn get<S: AsRef<str>>(&mut self, id: &ScopeInfoId, key: S) -> Option<ScopeInfoId> {
    let definitions = self.expect_get(id);
    if let Some(&top_value) = definitions.map.get(key.as_ref()) {
      if top_value == TOMBSTONE || top_value == UNDEFINED {
        None
      } else {
        Some(top_value)
      }
    } else if definitions.stack.len() > 1 {
      for index in (0..definitions.stack.len() - 1).rev() {
        // SAFETY: boundary had been checked
        let id = unsafe { definitions.stack.get_unchecked(index) };
        if let Some(&value) = self.expect_get(id).map.get(key.as_ref()) {
          if value == TOMBSTONE || value == UNDEFINED {
            return None;
          } else {
            return Some(value);
          }
        }
      }
      let definitions = self.expect_get_mut(id);
      definitions.map.insert(key.as_ref().to_string(), TOMBSTONE);
      None
    } else {
      None
    }
  }

  pub fn set(&mut self, id: ScopeInfoId, key: String) {
    let scope = self.expect_get_mut(&id);
    scope.map.insert(key, id);
  }

  pub fn delete<S: AsRef<str>>(&mut self, id: ScopeInfoId, key: S) {
    let scope = self.expect_get_mut(&id);
    if scope.stack.len() > 1 {
      scope.map.insert(key.as_ref().to_string(), TOMBSTONE);
    } else {
      scope.map.remove(key.as_ref());
    }
  }
}

#[derive(Debug)]
pub struct ScopeInfo {
  // id: ScopeInfoId,
  stack: Vec<ScopeInfoId>,
  map: FxHashMap<String, ScopeInfoId>,
  pub is_strict: bool,
}
