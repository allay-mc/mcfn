use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::errors::{Error, ErrorKind, Location};
use crate::macros::Macro;
use crate::utils::read_lines;

const MAGIC: &'static str = "__mcfn_internal";
const PRELUDE: &'static str = include_str!("prelude.mcfunction");
const MACRO_IDENTIFIER: &'static str = "#!";

#[derive(Clone)]
struct Line {
    pub file: String,
    pub line: u32,
    pub content: LineContent,
}

#[derive(Clone)]
enum LineContent {
    /// A (partial) Minecraft command.
    Command(String),

    /// A mcfn macro.
    Macro(Macro),

    /// A mcfn comment or a blank line.
    Empty,
}

impl FromStr for LineContent {
    type Err = ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let l = s.trim_start();
        if l.is_empty() {
            return Ok(Self::Empty);
        }
        if let Some(content) = l.strip_prefix(MACRO_IDENTIFIER) {
            let (name, arg) = match content.split_once(char::is_whitespace) {
                Some((name, value)) => (name, Some(value)),
                None => (content, None),
            };
            let r#macro = match name {
                "call" => Macro::Call(
                    arg.ok_or(ErrorKind::MissingArgument(name.to_string()))?
                        .to_string(),
                ),
                "case" => Macro::Case(
                    arg.ok_or(ErrorKind::MissingArgument(name.to_string()))?
                        .to_string(),
                ),
                "default" => {
                    if arg.is_some() {
                        return Err(ErrorKind::UnexpectedArgument(name.to_string()));
                    }
                    Macro::Default
                }
                "else" => {
                    if arg.is_some() {
                        return Err(ErrorKind::UnexpectedArgument(name.to_string()));
                    }
                    Macro::Else
                }
                "end" => {
                    if arg.is_some() {
                        return Err(ErrorKind::UnexpectedArgument(name.to_string()));
                    }
                    Macro::End
                }
                "if" => Macro::If(
                    arg.ok_or(ErrorKind::MissingArgument(name.to_string()))?
                        .to_string(),
                ),
                "ifn" => Macro::Ifn(
                    arg.ok_or(ErrorKind::MissingArgument(name.to_string()))?
                        .to_string(),
                ),
                "include" => Macro::Include(
                    arg.ok_or(ErrorKind::MissingArgument(name.to_string()))?
                        .to_string(),
                ),
                "proc" => Macro::Proc(
                    arg.ok_or(ErrorKind::MissingArgument(name.to_string()))?
                        .to_string(),
                ),
                "switch" => Macro::Switch(
                    arg.ok_or(ErrorKind::MissingArgument(name.to_string()))?
                        .to_string(),
                ),
                "then" => {
                    if arg.is_some() {
                        return Err(ErrorKind::UnexpectedArgument(name.to_string()));
                    }
                    Macro::Then
                }
                "with" => Macro::With(
                    arg.ok_or(ErrorKind::MissingArgument(name.to_string()))?
                        .to_string(),
                ),
                "!" => return Ok(Self::Empty),
                macro_name => return Err(ErrorKind::UnknownMacro(macro_name.to_string())),
            };
            Ok(Self::Macro(r#macro))
        } else {
            Ok(Self::Command(l.to_string()))
        }
    }
}

pub fn parse_file(path: &Path) -> anyhow::Result<()> {
    // TODO: validate file extension
    let mut rendered_lines = Vec::new();
    for (row, line) in read_lines(path)?.enumerate() {
        rendered_lines.push(Line {
            file: path.display().to_string(),
            line: row.try_into()?,
            content: LineContent::from_str(&line?)?,
        });
    }
    let mut rendered = PRELUDE.to_string();
    rendered.push_str(&render_block(
        &mut rendered_lines.into_iter(),
        Vec::new(),
        &mut HashMap::new(),
        0,
    )?);
    fs::write(path.with_extension("mcfunction"), rendered)?;
    Ok(())
}

fn render_block(
    block: &mut impl Iterator<Item = Line>,
    mut context: Vec<Macro>,
    procs: &mut HashMap<String, String>,
    depth: usize,
) -> Result<String, Error> {
    let mut rendered = String::new();

    let mut ifs: Vec<String> = Vec::new();
    let mut ifns: Vec<String> = Vec::new();

    let scoped_variable: String = format!("{}_{}", MAGIC, depth);

    while let Some(line) = block.next() {
        match line.content {
            LineContent::Command(command) => {
                rendered.push_str(&command);
                rendered.push('\n');
            }
            LineContent::Macro(Macro::Call(identifier)) => {
                let content = procs.get(&identifier).ok_or(Error {
                    location: Location {
                        file: line.file,
                        line: line.line,
                    },
                    kind: ErrorKind::UnknownProc(identifier),
                })?;
                rendered.push_str(content);
            }
            LineContent::Macro(Macro::Case(condition)) => match context.first() {
                Some(Macro::Switch(prefix_cond)) => {
                    let prefix = format!("execute if {} {} run ", prefix_cond, condition);
                    let content = render_block(block, context.clone(), procs, depth + 1)?;
                    rendered.push_str(
                        &content
                            .lines()
                            .map(|l| format!("{}{}\n", prefix, l))
                            .collect::<String>(),
                    );
                }
                _ => {
                    return Err(Error {
                        location: Location {
                            file: line.file,
                            line: line.line,
                        },
                        kind: ErrorKind::UnexpectedMacro("case".to_string()),
                    })
                }
            },
            LineContent::Macro(Macro::Default) => {
                // TODO
            }
            LineContent::Macro(Macro::Else) => match context.first() {
                Some(Macro::If(_) | Macro::Ifn(_)) => {
                    return Ok(rendered);
                }
                _ => {
                    return Err(Error {
                        location: Location {
                            file: line.file,
                            line: line.line,
                        },
                        kind: ErrorKind::UnexpectedMacro("else".to_string()),
                    })
                }
            },
            LineContent::Macro(Macro::End) => match context.first() {
                Some(
                    Macro::If(_)
                    | Macro::Ifn(_)
                    | Macro::Proc(_)
                    | Macro::Switch(_)
                    | Macro::Case(_), // cannot be first FIXME
                ) => return Ok(rendered),
                _ => {
                    return Err(Error {
                        location: Location {
                            file: line.file,
                            line: line.line,
                        },
                        kind: ErrorKind::UnexpectedMacro("end".to_string()),
                    })
                }
            },
            LineContent::Macro(ref ctx @ Macro::If(ref condition)) => {
                ifs.push(condition.to_string());
                context.push(ctx.clone());
            }
            LineContent::Macro(ref ctx @ Macro::Ifn(ref condition)) => {
                ifns.push(condition.to_string());
                context.push(ctx.clone());
            }
            LineContent::Macro(Macro::Include(path)) => todo!(),
            LineContent::Macro(ref ctx @ Macro::Proc(ref identifier)) => {
                context.push(ctx.clone());
                let content = render_block(block, context.clone(), procs, depth + 1)?;
                procs.insert(identifier.to_string(), content);
            }
            LineContent::Macro(ref ctx @ Macro::Switch(_)) => context.push(ctx.clone()),
            LineContent::Macro(Macro::Then) => {
                match context.first() {
                    Some(Macro::If(_) | Macro::Ifn(_)) => {
                        // a score is used to save the condition before running any
                        // commands so in case properties which affect the condition
                        // change in an if(n)-block, the else-block is ensured to not
                        // be evaluated

                        // initialize condition check
                        rendered.push_str(&format!(
                            "scoreboard objectives add {} dummy\n",
                            scoped_variable,
                        ));
                        rendered.push_str(&format!(
                            "scoreboard players set {} {} 0\n",
                            MAGIC, scoped_variable
                        ));

                        // check condition
                        let mut verify = String::from("execute");
                        for cond in &ifs {
                            verify.push_str(&format!(" if {}", cond));
                        }
                        for cond in &ifns {
                            verify.push_str(&format!(" unless {}", cond));
                        }
                        verify.push_str(&format!(
                            " run scoreboard players set {} {} 1",
                            MAGIC, scoped_variable
                        ));

                        // prefix each command in if block
                        let prefix = format!(
                            "execute if scores {} {} matches 1 run ",
                            MAGIC, scoped_variable
                        );
                        let content = render_block(block, context.clone(), procs, depth + 1)?;
                        rendered.push_str(
                            &content
                                .lines()
                                .map(|l| format!("{}{}\n", prefix, l))
                                .collect::<String>(),
                        );

                        // prefix each command in else-block
                        let else_prefix = format!(
                            "execute unless scores {} {} matches 1 run ",
                            MAGIC, scoped_variable
                        );
                        let else_content = render_block(block, context.clone(), procs, depth + 1)?;
                        rendered.push_str(
                            &else_content
                                .lines()
                                .map(|l| format!("{}{}\n", else_prefix, l))
                                .collect::<String>(),
                        );
                    }
                    Some(Macro::Case(_)) => {
                        // TODO
                    }
                    _ => {
                        return Err(Error {
                            location: Location {
                                file: line.file,
                                line: line.line,
                            },
                            kind: ErrorKind::UnexpectedMacro("then".to_string()),
                        })
                    }
                }
            }
            LineContent::Macro(Macro::With(prefix)) => todo!(),
            LineContent::Empty => {}
        };
    }

    // FIXME: ensure end is used if necessary
    Ok(rendered)
}
