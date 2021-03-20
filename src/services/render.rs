use katex_wasmbind::KaTeXOptions;
use pulldown_cmark::{html, Options, Parser};
use yew::prelude::*;

pub fn render(input: String) -> String {
    let d = KaTeXOptions::inline_mode();
    let mut real_output = String::new();
    let mut stack: Vec<usize> = vec![];
    let mut prev = 0;
    for (idx, _) in input.match_indices("$") {
        if stack.is_empty() {
            if let Some(s) = input.get(prev..idx) {
                real_output.push_str(&s);
            }
            stack.push(idx);
        } else {
            if let Some(j) = stack.pop() {
                real_output.push_str(&d.render(input.get((j + 1)..idx).unwrap()));
            }
        }
        prev = idx + 1;
    }
    if let Some(s) = input.get(prev..) {
        real_output.push_str(&s);
    }
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&real_output, options);
    let mut html_output: String = String::with_capacity(input.len() * 3 / 2);
    html::push_html(&mut html_output, parser);
    html_output
}
