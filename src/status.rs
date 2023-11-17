use termstatus::TermStatus;

#[derive(TermStatus)]
pub enum Status {
    #[style(bold, red)]
    Error,

    #[style(bold, green)]
    Finished,

    #[style(bold, green)]
    Transpiled,

    #[style(bold, green)]
    Transpiling,
}
