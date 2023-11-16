#[derive(Clone, Debug)]
pub enum Macro {
    Call(String),
    Declare(String),
    Delete(String),
    Else,
    End,
    If(String),
    Ifn(String),
    Include(String),
    Log(String),
    Proc(String),
    Then,
    When(String, String),
    With(String),
}
