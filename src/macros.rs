#[derive(Clone, Debug)]
pub enum Macro {
    Call(String),
    Case(String),
    Default,
    Else,
    End,
    If(String),
    Ifn(String),
    Include(String),
    Proc(String),
    Switch(String),
    Then,
    With(String),
}
