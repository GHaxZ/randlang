//  TODO: Create "Variable" struct with "VariableType" enum field, maybe use traits?

// Types of variables
#[derive(Clone, Debug)]
pub enum Variable {
    String(String),
    Integer(i32),
    Decimal(f32),
    Boolean(bool),
}
