#[derive(Clone, Debug)]
pub enum Variable {
    String(String),
    Integer(i32),
    Decimal(f32),
    Boolean(bool),
}
