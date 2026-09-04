//! Markdown + LaTeX rendering.
//!
//! Content bodies are Markdown with `$…$` (inline) and `$$…$$` (display) LaTeX. Markdown is
//! rendered with `pulldown-cmark`; math events are intercepted and converted to MathML Core
//! with `pulldown-latex`, so equations render natively without any JavaScript library.

use pulldown_cmark::{html, CowStr, Event, Options, Parser, Tag, TagEnd};
use pulldown_latex::config::DisplayMode;
use pulldown_latex::{push_mathml, Parser as LatexParser, RenderConfig, Storage};

/// Escapes text for safe inclusion in HTML.
pub fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

/// Renders one LaTeX expression to MathML. Falls back to a visible code span on failure.
pub fn latex_to_mathml(src: &str, display: bool) -> String {
    let storage = Storage::new();
    let parser = LatexParser::new(src, &storage);
    let config = RenderConfig {
        display_mode: if display {
            DisplayMode::Block
        } else {
            DisplayMode::Inline
        },
        annotation: Some(src),
        ..RenderConfig::default()
    };
    let mut out = String::new();
    match push_mathml(&mut out, parser, config) {
        Ok(()) if !out.is_empty() => out,
        _ => format!("<code class=\"math-error\">{}</code>", escape_html(src)),
    }
}

/// Renders a two-column listing (a directory tree: path, then two or more spaces, then a
/// description) as a grid whose description column wraps. Continuation lines that start at
/// the description column are appended to the previous description.
pub fn tree_to_html(src: &str) -> String {
    struct Row {
        depth: usize,
        path: String,
        desc: String,
    }
    let mut rows: Vec<Row> = Vec::new();
    let mut desc_col: Option<usize> = None;
    for raw in src.lines() {
        let line = raw.trim_end();
        if line.trim().is_empty() {
            continue;
        }
        let indent = line.len() - line.trim_start().len();
        let content = line.trim_start();
        // A continuation line begins at (or beyond) the description column of the block.
        if let (Some(col), Some(last)) = (desc_col, rows.last_mut()) {
            if indent >= col && !last.desc.is_empty() {
                last.desc.push(' ');
                last.desc.push_str(content.trim());
                continue;
            }
        }
        let (path, desc) = match content.find("  ") {
            Some(i) => (content[..i].to_string(), content[i..].trim().to_string()),
            None => (content.to_string(), String::new()),
        };
        if !desc.is_empty() {
            let col = indent + content.len() - content[path.len()..].trim_start().len();
            desc_col = Some(desc_col.map_or(col, |c| c.min(col)));
        }
        rows.push(Row {
            depth: indent / 2,
            path,
            desc,
        });
    }
    let mut out = String::from("<div class=\"tree\" role=\"table\">");
    for r in rows {
        if r.desc.is_empty() {
            out.push_str(&format!(
                "<div class=\"tree-row tree-head\" role=\"row\" style=\"--depth:{}\"><code class=\"tree-path\" role=\"cell\">{}</code></div>",
                r.depth,
                escape_html(&r.path)
            ));
        } else {
            out.push_str(&format!(
                "<div class=\"tree-row\" role=\"row\" style=\"--depth:{}\"><code class=\"tree-path\" role=\"cell\">{}</code><span class=\"tree-desc\" role=\"cell\">{}</span></div>",
                r.depth,
                escape_html(&r.path),
                escape_html(&r.desc)
            ));
        }
    }
    out.push_str("</div>");
    out
}

/// Renders Markdown (with math, tables, footnotes) to HTML. Fenced blocks tagged `tree` become
/// wrapping path/description grids (see [`tree_to_html`]).
pub fn markdown_to_html(md: &str) -> String {
    use pulldown_cmark::CodeBlockKind;
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_MATH);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_SMART_PUNCTUATION);
    let mut events: Vec<Event<'_>> = Vec::new();
    let mut tree_buf: Option<String> = None;
    for ev in Parser::new_ext(md, opts) {
        if let Some(buf) = tree_buf.as_mut() {
            match ev {
                Event::Text(t) => buf.push_str(&t),
                Event::End(TagEnd::CodeBlock) => {
                    let html = tree_to_html(buf);
                    tree_buf = None;
                    events.push(Event::Html(CowStr::from(html)));
                }
                _ => {}
            }
            continue;
        }
        match ev {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref info)))
                if info.trim() == "tree" =>
            {
                tree_buf = Some(String::new());
            }
            Event::InlineMath(s) => {
                events.push(Event::Html(CowStr::from(latex_to_mathml(&s, false))))
            }
            Event::DisplayMath(s) => events.push(Event::Html(CowStr::from(format!(
                "<div class=\"math-display\">{}</div>",
                latex_to_mathml(&s, true)
            )))),
            Event::Start(Tag::Table(al)) => {
                events.push(Event::Html(CowStr::from("<div class=\"table-scroll\">")));
                events.push(Event::Start(Tag::Table(al)));
            }
            Event::End(TagEnd::Table) => {
                events.push(Event::End(TagEnd::Table));
                events.push(Event::Html(CowStr::from("</div>")));
            }
            other => events.push(other),
        }
    }
    let mut out = String::new();
    html::push_html(&mut out, events.into_iter());
    out
}

const PSEUDO_KEYWORDS: &[&str] = &[
    "function", "for", "in", "if", "else", "elif", "while", "repeat", "until", "return", "each",
    "and", "or", "not", "break", "continue", "then", "do", "assert", "yield", "true", "false",
    "with", "where", "of", "to", "from", "by", "step",
];

fn is_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Highlights one pseudocode line: comments, keywords, function names, upper-case procedure calls.
fn highlight_pseudocode_line(line: &str) -> String {
    let (code, comment) = match line.find("//") {
        Some(i) => (&line[..i], Some(&line[i..])),
        None => (line, None),
    };
    let mut out = String::with_capacity(line.len() + 32);
    let chars: Vec<char> = code.chars().collect();
    let mut i = 0;
    let mut after_function = false;
    while i < chars.len() {
        let c = chars[i];
        if is_ident_char(c) {
            let start = i;
            while i < chars.len() && is_ident_char(chars[i]) {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            let next_paren = chars.get(i).copied() == Some('(');
            let lower = word.to_lowercase();
            if after_function {
                out.push_str(&format!(
                    "<span class=\"pc-f\">{}</span>",
                    escape_html(&word)
                ));
                after_function = false;
            } else if PSEUDO_KEYWORDS.contains(&lower.as_str()) && lower == word {
                out.push_str(&format!(
                    "<span class=\"pc-k\">{}</span>",
                    escape_html(&word)
                ));
                after_function = word == "function";
            } else if next_paren
                && word.chars().any(|c| c.is_ascii_uppercase())
                && word == word.to_uppercase()
            {
                out.push_str(&format!(
                    "<span class=\"pc-f\">{}</span>",
                    escape_html(&word)
                ));
            } else {
                out.push_str(&escape_html(&word));
            }
        } else {
            out.push_str(&escape_html(&c.to_string()));
            i += 1;
        }
    }
    if let Some(cm) = comment {
        out.push_str(&format!("<span class=\"pc-c\">{}</span>", escape_html(cm)));
    }
    out
}

/// Renders a pseudocode block body: text outside the first fenced code block is Markdown, the
/// fence becomes a numbered line list that keeps each line's indentation when it wraps.
pub fn pseudocode_to_html(body: &str) -> String {
    let lines: Vec<&str> = body.lines().collect();
    let open = lines.iter().position(|l| l.trim_start().starts_with("```"));
    let close = open.and_then(|o| {
        lines[o + 1..]
            .iter()
            .position(|l| l.trim_start().starts_with("```"))
            .map(|p| o + 1 + p)
    });
    let (Some(o), Some(c)) = (open, close) else {
        return markdown_to_html(body);
    };
    let mut out = String::new();
    let before = lines[..o].join("\n");
    if !before.trim().is_empty() {
        out.push_str(&markdown_to_html(&before));
    }
    out.push_str("<div class=\"pc\"><ol class=\"pc-lines\">");
    for line in &lines[o + 1..c] {
        let expanded = line.replace('\t', "    ");
        let indent = expanded.len() - expanded.trim_start_matches(' ').len();
        let content = expanded.trim_start_matches(' ');
        if content.trim().is_empty() {
            out.push_str("<li class=\"pc-line pc-blank\"><code> </code></li>");
        } else {
            out.push_str(&format!(
                "<li class=\"pc-line\" style=\"--pc-indent:{indent}\"><code>{}</code></li>",
                highlight_pseudocode_line(content)
            ));
        }
    }
    out.push_str("</ol></div>");
    let after = lines[c + 1..].join("\n");
    if !after.trim().is_empty() {
        out.push_str(&markdown_to_html(&after));
    }
    out
}

/// Strips Markdown to plain text (rough, for snippets and search).
pub fn markdown_to_text(md: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_MATH);
    opts.insert(Options::ENABLE_TABLES);
    let mut out = String::new();
    for ev in Parser::new_ext(md, opts) {
        match ev {
            Event::Text(t) | Event::Code(t) | Event::InlineMath(t) | Event::DisplayMath(t) => {
                out.push_str(&t);
                out.push(' ');
            }
            Event::SoftBreak | Event::HardBreak => out.push(' '),
            Event::End(TagEnd::Paragraph)
            | Event::End(TagEnd::Item)
            | Event::End(TagEnd::Heading(_)) => out.push(' '),
            _ => {}
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inline_math_becomes_mathml() {
        let html = markdown_to_html("Let $c_i = p_i + k$ hold.");
        assert!(html.contains("<math"), "{html}");
        assert!(html.contains("<p>"));
    }

    #[test]
    fn display_math_is_block() {
        let html = markdown_to_html("$$\\sum_{i=0}^{n} x_i \\pmod{26}$$");
        assert!(html.contains("math-display"), "{html}");
        assert!(html.contains("display=\"block\""), "{html}");
    }

    #[test]
    fn tables_are_wrapped() {
        let html = markdown_to_html("| a | b |\n| --- | --- |\n| 1 | 2 |\n");
        assert!(
            html.starts_with("<div class=\"table-scroll\"><table>"),
            "{html}"
        );
        assert!(
            html.contains("</table>\n</div>") || html.contains("</table></div>"),
            "{html}"
        );
    }

    #[test]
    fn plain_text_extraction() {
        let t = markdown_to_text("# Head\n\nSome *text* with `code` and $x^2$.\n\n- item");
        assert_eq!(t, "Head Some text with code and x^2 . item");
    }

    #[test]
    fn pseudocode_lines_keep_indent_and_highlight() {
        let body = "```text\n// setup\nfunction ENCRYPT(p, k):\n    for i in 0..n-1:\n        c[i] = (p[i] + k[i mod m]) mod 26   // add\n    return HISTOGRAM(c)\n```";
        let html = pseudocode_to_html(body);
        assert!(html.contains("<ol class=\"pc-lines\">"));
        assert!(html.contains("style=\"--pc-indent:8\""), "{html}");
        assert!(
            html.contains(
                "<span class=\"pc-k\">function</span> <span class=\"pc-f\">ENCRYPT</span>"
            ),
            "{html}"
        );
        assert!(
            html.contains("<span class=\"pc-f\">HISTOGRAM</span>"),
            "{html}"
        );
        assert!(
            html.contains("<span class=\"pc-c\">// add</span>"),
            "{html}"
        );
        assert!(html.contains("<span class=\"pc-c\">// setup</span>"));
        assert!(!html.contains("<pre>"));
    }

    #[test]
    fn pseudocode_without_fence_falls_back_to_markdown() {
        assert!(pseudocode_to_html("Just *text*").contains("<em>text</em>"));
    }

    #[test]
    fn tree_fences_become_wrapping_grids() {
        let md = "```tree\n<repo>/\n  a.py        first thing that is long\n              and continues here\n  dir/        second\n```";
        let html = markdown_to_html(md);
        assert!(html.contains("<div class=\"tree\""), "{html}");
        assert!(html.contains("tree-head"), "{html}");
        assert!(
            html.contains("first thing that is long and continues here"),
            "{html}"
        );
        assert!(html.contains("style=\"--depth:1\""), "{html}");
        assert!(!html.contains("<pre>"), "{html}");
        // Ordinary fences are untouched.
        assert!(markdown_to_html("```text\nx\n```").contains("<pre>"));
    }
}
