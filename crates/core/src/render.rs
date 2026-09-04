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

/// Renders Markdown (with math, tables, footnotes) to HTML.
pub fn markdown_to_html(md: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_MATH);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_SMART_PUNCTUATION);
    let events = Parser::new_ext(md, opts).flat_map(|ev| -> Vec<Event<'_>> {
        match ev {
            Event::InlineMath(s) => vec![Event::Html(CowStr::from(latex_to_mathml(&s, false)))],
            Event::DisplayMath(s) => vec![Event::Html(CowStr::from(format!(
                "<div class=\"math-display\">{}</div>",
                latex_to_mathml(&s, true)
            )))],
            Event::Start(Tag::Table(al)) => vec![
                Event::Html(CowStr::from("<div class=\"table-scroll\">")),
                Event::Start(Tag::Table(al)),
            ],
            Event::End(TagEnd::Table) => vec![
                Event::End(TagEnd::Table),
                Event::Html(CowStr::from("</div>")),
            ],
            other => vec![other],
        }
    });
    let mut out = String::new();
    html::push_html(&mut out, events);
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
}
