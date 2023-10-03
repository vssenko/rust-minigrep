use crate::config::Config;
use std::collections::HashMap;

static TRIMMED_SPACE_ADD: usize = 10;

// I am sorry for all these clones.

pub fn search_content(content: &str, q: &str) -> Vec<String> {
    let mut lineMap: HashMap<String, usize> = HashMap::new();

    let lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();

    lines.iter().enumerate().for_each(|(i, l)| {
        lineMap.insert(l.clone(), i);
    });

    let mut contained_lines: Vec<String> = lines
        .iter()
        .filter(|line| line.contains(q))
        .map(|l| l.clone())
        .collect();

    if Config::load().options.trim {
        let query_len = q.chars().count();
        let part_len = query_len + TRIMMED_SPACE_ADD;
        let trimmed: Vec<String> = contained_lines
            .iter()
            .map(|l| {
                let line_len: usize = l.chars().count();
                if line_len > part_len {
                    let ind = _find_char_ind(l, q).unwrap();
                    let offset = TRIMMED_SPACE_ADD / 2;
                    let start: usize = if (ind as i32) - (offset as i32) < 0 {
                        0
                    } else {
                        ind - offset
                    };
                    let end: usize = if start + part_len >= line_len {
                        line_len
                    } else {
                        start + part_len
                    };
                    let line_number = lineMap.get(l.as_str()).unwrap();
                    let result: String = l.chars().skip(start).take(end - start).collect();
                    let result = format!("...{}...", result);
                    lineMap.insert(result.clone(), *line_number);
                    result
                } else {
                    l.to_string()
                }
            })
            .collect();

        contained_lines = trimmed;
    }

    return contained_lines
        .iter()
        .map(|l| {
            let line_number = lineMap.get(l).unwrap();

            format!("line {}: {}", line_number, l)
        })
        .collect();
}

fn _find_char_ind(s: &str, q: &str) -> Option<usize> {
    let ind = s.find(q);

    if ind.is_none() {
        return None;
    }

    let ind = ind.unwrap();

    let mut byte_count = 0;
    let mut char_count = 0;

    for char in s.chars() {
        if byte_count >= ind {
            break;
        }

        char_count = char_count + 1;
        byte_count = byte_count + char.len_utf8();
    }

    return Some(char_count);
}
