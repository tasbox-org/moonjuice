use moonjuice_common::Position;
use serde::Serialize;

pub mod expression;
pub mod lvalue;
pub mod statement;

#[derive(Serialize)]
pub struct Node<T> {
  pub value: Box<T>,
  pub start: Position,
  pub end: Position,
}
