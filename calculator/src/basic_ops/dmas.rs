#[derive(Debug)]
pub struct TwoDigit(pub i32,pub i32);
pub struct ThreeDigit(pub i32,pub i32,pub i32);
impl TwoDigit {
   pub fn add(&self) -> i32 {
        self.0 + self.1
    }
   pub fn sub(&self) -> i32 {
        self.0 - self.1
    }
   pub fn mul(&self) -> i32 {
        self.0 * self.1
    }
   pub  fn div(&self) -> i32 {
        self.0 / self.1
    }
}
