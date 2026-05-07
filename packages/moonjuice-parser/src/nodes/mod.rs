use moonjuice_common::Position;

pub mod expression;
pub mod lvalue;
pub mod statement;

pub trait Node {
  fn get_start(&self) -> Position;

  fn get_end(&self) -> Position;
}
