use unicode_segmentation::UnicodeSegmentation;

pub const DEFAULT_FONT_SIZE: f64 = 16.0;
pub const DEFAULT_LINE_HEIGHT: f64 = 1.1;
pub const DEFAULT_WRAP_WIDTH: f64 = 200.0;
pub const DEFAULT_CHAR_WIDTH: f64 = 8.0;
pub const DEFAULT_TEXT_HEIGHT: f64 = 24.0;

/// Mirrors mermaid.js splitText.ts splitLineToFitWidth behavior for non-markdown labels.
/// Source: packages/mermaid/src/rendering-util/splitText.ts.
pub fn wrap_text_lines(text: &str, max_width: f64, char_width: f64) -> Vec<Vec<String>> {
    if text.is_empty() {
        return Vec::new();
    }
    let max_width = if max_width.is_finite() {
        max_width
    } else {
        f64::INFINITY
    };

    let mut lines = Vec::new();
    for raw_line in text.split('\n') {
        let trimmed = raw_line.trim();
        if trimmed.is_empty() {
            lines.push(vec![String::new()]);
            continue;
        }
        let words = split_line_to_words(trimmed);
        let wrapped = split_line_to_fit_width(words, max_width, char_width);
        lines.extend(wrapped);
    }

    lines
}

/// Matches mermaid.js createText.ts line-width checks using grapheme-based estimation.
pub fn line_width(line: &str, char_width: f64) -> f64 {
    if line.is_empty() {
        return 0.0;
    }
    let grapheme_count = line.graphemes(true).count() as f64;
    grapheme_count * char_width
}

pub fn measure_wrapped_lines_with_font_size(
    lines: &[Vec<String>],
    char_width: f64,
    font_size: f64,
) -> (f64, f64) {
    let max_width = lines
        .iter()
        .map(|line| line_width_words(line, char_width))
        .fold(0.0, f64::max);
    (
        max_width,
        wrapped_text_height_with_font_size(lines.len(), font_size),
    )
}

pub fn wrapped_text_height_with_font_size(line_count: usize, font_size: f64) -> f64 {
    if line_count == 0 {
        return 0.0;
    }
    let font_size = normalized_font_size(font_size);
    let text_height = DEFAULT_TEXT_HEIGHT * font_size / DEFAULT_FONT_SIZE;
    let line_spacing = font_size * DEFAULT_LINE_HEIGHT;
    text_height + (line_count.saturating_sub(1)) as f64 * line_spacing
}

pub fn scale_char_width(char_width: f64, font_size: f64) -> f64 {
    char_width * normalized_font_size(font_size) / DEFAULT_FONT_SIZE
}

fn normalized_font_size(font_size: f64) -> f64 {
    if font_size.is_finite() && font_size > 0.0 {
        font_size
    } else {
        DEFAULT_FONT_SIZE
    }
}
fn split_line_to_words(text: &str) -> Vec<String> {
    let mut words = Vec::new();
    for word in text.split_whitespace() {
        words.push(word.to_string());
    }
    if words.is_empty() {
        words.push(String::new());
    }
    words
}

fn split_line_to_fit_width(
    words: Vec<String>,
    max_width: f64,
    char_width: f64,
) -> Vec<Vec<String>> {
    let mut remaining = std::collections::VecDeque::from(words);
    let mut lines: Vec<Vec<String>> = Vec::new();
    let mut current: Vec<String> = Vec::new();

    loop {
        if remaining.is_empty() {
            if !current.is_empty() {
                lines.push(current);
            }
            break;
        }

        let next_word = remaining.pop_front().unwrap_or_default();

        let mut line_with_next = current.clone();
        line_with_next.push(next_word.clone());

        if check_fit(&line_with_next, max_width, char_width) {
            current = line_with_next;
            continue;
        }

        if !current.is_empty() {
            lines.push(current);
            current = Vec::new();
            remaining.push_front(next_word);
            continue;
        }

        if !next_word.is_empty() {
            let (first, rest) = split_word_to_fit_width(&next_word, max_width, char_width);
            lines.push(vec![first]);
            if !rest.is_empty() {
                remaining.push_front(rest);
            }
        }
    }

    lines
}

fn check_fit(words: &[String], max_width: f64, char_width: f64) -> bool {
    line_width_words(words, char_width) <= max_width
}

fn split_word_to_fit_width(word: &str, max_width: f64, char_width: f64) -> (String, String) {
    let graphemes: Vec<&str> = word.graphemes(true).collect();
    if graphemes.is_empty() {
        return (String::new(), String::new());
    }

    let mut used = Vec::new();
    let mut remaining_start = graphemes.len();
    for (idx, grapheme) in graphemes.iter().enumerate() {
        let mut candidate = used.clone();
        candidate.push(*grapheme);
        let candidate_str = candidate.concat();
        if line_width(&candidate_str, char_width) <= max_width || used.is_empty() {
            used = candidate;
            continue;
        }
        remaining_start = idx;
        break;
    }

    if used.is_empty() {
        used.push(graphemes[0]);
        remaining_start = 1;
    }

    let remaining = if remaining_start < graphemes.len() {
        graphemes[remaining_start..].concat()
    } else {
        String::new()
    };
    (used.concat(), remaining)
}

pub fn line_width_words(words: &[String], char_width: f64) -> f64 {
    let joined = join_words(words);
    line_width(&joined, char_width)
}

fn join_words(words: &[String]) -> String {
    let mut out = String::new();
    for (idx, word) in words.iter().enumerate() {
        if idx > 0 {
            out.push(' ');
        }
        out.push_str(word);
    }
    out
}
