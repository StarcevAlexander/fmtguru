#[derive(Clone, Debug, PartialEq)]
enum Segment {
    Literal(String),
    Key(String),
}

#[derive(Clone, Debug)]
pub struct Template {
    segments: Vec<Segment>,
}

impl Template {
    pub fn new(source: impl Into<String>) -> Self {
        let source = source.into();
        let mut segments = Vec::new();
        let mut literal = String::new();
        let mut chars = source.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '{' {
                if chars.peek() == Some(&'{') {
                    chars.next();
                    literal.push('{');
                    continue;
                }
                if !literal.is_empty() {
                    segments.push(Segment::Literal(std::mem::take(&mut literal)));
                }
                let mut key = String::new();
                for k in chars.by_ref() {
                    if k == '}' {
                        break;
                    }
                    key.push(k);
                }
                segments.push(Segment::Key(key));
            } else if c == '}' && chars.peek() == Some(&'}') {
                chars.next();
                literal.push('}');
            } else {
                literal.push(c);
            }
        }
        if !literal.is_empty() {
            segments.push(Segment::Literal(literal));
        }
        Self { segments }
    }

    pub fn keys(&self) -> Vec<&str> {
        self.segments
            .iter()
            .filter_map(|s| match s {
                Segment::Key(k) => Some(k.as_str()),
                _ => None,
            })
            .collect()
    }

    pub fn render(&self, named: &[(&str, &str)]) -> String {
        let mut out = String::new();
        for segment in &self.segments {
            match segment {
                Segment::Literal(text) => out.push_str(text),
                Segment::Key(key) => match named.iter().find(|(k, _)| k == key) {
                    Some((_, value)) => out.push_str(value),
                    None => {
                        out.push('{');
                        out.push_str(key);
                        out.push('}');
                    }
                },
            }
        }
        out
    }

    pub fn render_positional(&self, values: &[&str]) -> String {
        let mut out = String::new();
        for segment in &self.segments {
            match segment {
                Segment::Literal(text) => out.push_str(text),
                Segment::Key(key) => {
                    let index: Option<usize> = key.parse().ok();
                    match index.and_then(|i| values.get(i)) {
                        Some(value) => out.push_str(value),
                        None => {
                            out.push('{');
                            out.push_str(key);
                            out.push('}');
                        }
                    }
                }
            }
        }
        out
    }
}

impl std::fmt::Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for segment in &self.segments {
            match segment {
                Segment::Literal(text) => f.write_str(text)?,
                Segment::Key(key) => {
                    f.write_str("{")?;
                    f.write_str(key)?;
                    f.write_str("}")?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn named_placeholders_render() {
        let template = Template::new("Hello {name}, you are {age}");
        let out = template.render(&[("name", "Ivan"), ("age", "30")]);
        assert_eq!(out, "Hello Ivan, you are 30");
    }

    #[test]
    fn missing_keys_are_kept_verbatim() {
        let template = Template::new("Hi {name} and {unknown}");
        let out = template.render(&[("name", "Ivan")]);
        assert_eq!(out, "Hi Ivan and {unknown}");
    }

    #[test]
    fn double_braces_escape() {
        let template = Template::new("{{literal}} stays, {value} renders");
        let out = template.render(&[("value", "x")]);
        assert_eq!(out, "{literal} stays, x renders");
    }

    #[test]
    fn positional_rendering() {
        let template = Template::new("{0} vs {1}");
        let out = template.render_positional(&["first", "second"]);
        assert_eq!(out, "first vs second");
    }

    #[test]
    fn keys_are_listed() {
        let template = Template::new("{a} and {b}");
        assert_eq!(template.keys(), vec!["a", "b"]);
    }

    #[test]
    fn display_reconstructs_source() {
        let template = Template::new("Hi {name}!");
        assert_eq!(template.to_string(), "Hi {name}!");
    }
}
