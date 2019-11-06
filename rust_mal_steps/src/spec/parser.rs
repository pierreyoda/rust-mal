use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use super::MalTestingLine;

pub enum Test {
    Ok,
    None,
}

fn load_local_mal_spec_file(filepath: &Path) -> Result<BufReader<File>, String> {
    Ok(BufReader::new(File::open(filepath).map_err(|err| {
        format!(
            "load_local_mal_spec_file({:?}) error: {}",
            filepath.display(),
            err
        )
    })?))
}

fn parse_mal_spec(mut reader: BufReader<File>) -> Result<Vec<MalTestingLine>, String> {
    // read all lines
    let mut content = String::new();
    reader
        .read_to_string(&mut content)
        .map_err(|err| format!("parse_mal_spec reading error: {}", err))?;
    let lines: Vec<String> = content.split('\n').map(|l| l.trim().into()).collect();

    // parse
    let mut parsed_lines = Vec::with_capacity(lines.len());
    let mut declaring_input: Vec<String> = vec![];
    for line in lines {
        if line.is_empty() {
            continue;
        }

        let flag = line.trim_start_matches(";>>> ");
        if flag.len() != line.len() {
            let parts: Vec<&str> = flag.split('=').collect();
            if parts.len() == 2 && parts[0] != "optional" {
                parsed_lines.push(MalTestingLine::ToggleOptional(parts[1] == "True"));
                continue;
            }
        }

        let section_name = line.trim_start_matches(";;");
        if section_name.len() != line.len()
        /*&& !section_name.starts_with('-')*/
        {
            parsed_lines.push(MalTestingLine::BeginSection(section_name.trim().into()));
            continue;
        }

        let output = line.trim_start_matches(";=>");
        if output.len() != line.len() {
            if declaring_input.is_empty() {
                return Err(format!("no matching input for output \"{}\"", output));
            }
            parsed_lines.push(MalTestingLine::InputShouldOutput(
                declaring_input.clone(),
                output.into(),
            ));
            declaring_input = vec![];
            continue;
        }

        let error = line.trim_start_matches(";/.");
        if error.len() != line.len() {
            if declaring_input.is_empty() {
                return Err(format!("no matching input for error output \"{}\"", output));
            }
            parsed_lines.push(MalTestingLine::InputShouldThrow(declaring_input.clone()));
            declaring_input = vec![];
            continue;
        }

        declaring_input.push(line);
    }
    Ok(parsed_lines)
}

pub fn load_and_parse_mal_spec(filename: &str) -> Result<Vec<MalTestingLine>, String> {
    let filename = format!("./tests/{}", filename);
    let filepath = Path::new(&filename);
    parse_mal_spec(load_local_mal_spec_file(&filepath)?)
}
