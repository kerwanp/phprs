use std::{fs, path::Path};

use anyhow::Result;
use ariadne::{Color, Label, Report, ReportKind, Source};
use clap::Parser;
use globwalk::GlobWalkerBuilder;

#[derive(Debug, Parser)]
pub struct ParseCommand {
    path: Vec<String>,
}

pub fn run(cmd: ParseCommand) -> Result<()> {
    for input in cmd.path {
        let path = Path::new(&input);

        if path.is_dir() {
            let walker = GlobWalkerBuilder::from_patterns(input, &["**/*.php"]).build()?;

            for entry_result in walker {
                let entry = entry_result?;
                parse_file(entry.path());
            }
        } else if path.is_file() {
            parse_file(path);
        } else {
            let walker = GlobWalkerBuilder::from_patterns(".", &[&input]).build()?;

            for entry_result in walker {
                let entry = entry_result?;
                parse_file(entry.path());
            }
        }
    }

    Ok(())
}

pub fn parse_file(path: &Path) {
    let content = &fs::read_to_string(path).unwrap();
    let result = phprs_parser::parse(content);

    match result {
        Ok(_) => {}
        Err(errs) => {
            let label = path.display().to_string();
            for err in errs {
                Report::build(ReportKind::Error, (&label, err.span().into_range()))
                    .with_code(3)
                    .with_message("Parsing error")
                    .with_label(
                        Label::new((&label, err.span().into_range()))
                            .with_message(err.reason().to_string())
                            .with_color(Color::Red),
                    )
                    .finish()
                    .print((&label, Source::from(&content)))
                    .unwrap();
            }
        }
    }
}
