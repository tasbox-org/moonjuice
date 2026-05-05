pub struct PeekableStream<T>
where
  T: std::cmp::PartialEq,
{
  collection: Vec<T>,
  index: usize,
}

impl<T: std::cmp::PartialEq> PeekableStream<T> {
  pub fn new(collection: Vec<T>) -> Self {
    PeekableStream { collection, index: 0 }
  }

  pub fn get_index(&self) -> usize {
    self.index
  }

  pub fn unwrap(&self) -> &Vec<T> {
    &self.collection
  }

  pub fn peek(&self, distance: usize) -> Option<&T> {
    self.collection.get(self.index.checked_add(distance)?)
  }

  pub fn peek_back(&self, distance: usize) -> Option<&T> {
    self.collection.get(self.index.checked_sub(distance)?)
  }

  pub fn peek_next(&self) -> Option<&T> {
    self.peek(0)
  }

  pub fn consume(&mut self) -> Option<&T> {
    let item = self.collection.get(self.index)?;
    self.index += 1;

    Some(item)
  }

  pub fn advance_by(&mut self, n: usize) {
    self.index += n
  }

  pub fn consume_if(&mut self, predicate: impl Fn(&T) -> bool) -> Option<&T> {
    if predicate(self.peek(0)?) { self.consume() } else { None }
  }

  pub fn has_next(&self) -> bool {
    self.index < self.collection.len()
  }

  pub fn has_remaining(&self, count: usize) -> bool {
    self.collection.len() - self.index >= count
  }

  pub fn is_match(&self, pattern: impl IntoIterator<Item = T>) -> bool {
    pattern
      .into_iter()
      .enumerate()
      .all(|(index, item)| self.peek(index).is_some_and(|actual| *actual == item))
  }
}
