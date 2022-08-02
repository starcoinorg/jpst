// Copyright (c) The Starcoin Core Contributors
// SPDX-License-Identifier: Apache-2.0

use jsonpath_plus::JsonPath;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Serialize;
use serde_json::{json, map::Entry, Map, Value};

#[derive(Debug, Clone, PartialEq)]
struct Match {
    name: String,
    start: usize,
    end: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Template {
    src: String,
    matches: Vec<Match>,
}

impl Template {
    pub(crate) fn new(template: &str, matches: Vec<Match>) -> Self {
        Template {
            src: template.to_string(),
            matches,
        }
    }
}

pub trait TemplateEngine {
    fn parse(tpl: &str) -> Template;
    fn render(ctx: &TemplateContext, tpl: &Template) -> String {
        let mut src = tpl.src.clone();
        let root = ctx.as_value();
        for m in &tpl.matches {
            let path = match JsonPath::compile(m.name.as_str()) {
                Ok(path) => path,
                Err(e) => {
                    eprintln!("Parse Error: {:?}", e);
                    continue;
                }
            };

            let value = path.find(&root).pop().cloned().unwrap_or_else(|| json!(""));
            let value = value_to_str(&value);
            src = src.replace(&tpl.src[m.start..m.end], value.as_str());
        }
        src
    }
}

fn value_to_str(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        _ => v.to_string(),
    }
}

pub struct ContextEntry<'a> {
    entry: Entry<'a>,
}

impl<'a> ContextEntry<'a> {
    pub fn set<V>(self, value: V)
    where
        V: Serialize,
    {
        let json_value = serde_json::to_value(value).expect("must be json value");
        match self.entry {
            Entry::Occupied(mut o) => {
                o.insert(json_value);
            }
            Entry::Vacant(v) => {
                v.insert(json_value);
            }
        }
    }

    pub fn append<V>(self, value: V)
    where
        V: Into<Value>,
    {
        match self.entry {
            Entry::Occupied(mut o) => {
                if o.get().is_array() {
                    o.get_mut()
                        .as_array_mut()
                        .expect("must be json array")
                        .push(value.into());
                } else {
                    o.insert(json!([o.get().clone(), value.into()]));
                }
            }
            Entry::Vacant(v) => {
                v.insert(json!([value.into()]));
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateContext {
    root: Map<String, Value>,
}

impl Default for TemplateContext {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateContext {
    pub fn new() -> Self {
        TemplateContext { root: Map::new() }
    }

    pub fn new_with_root<V>(root_value: V) -> Self
    where
        V: Serialize,
    {
        let value: Value = serde_json::to_value(root_value).expect("must be json value");
        let root = match value {
            Value::Object(m) => m,
            _ => {
                let mut m = Map::new();
                m.insert("value".to_string(), value);
                m
            }
        };

        TemplateContext { root }
    }

    pub fn entry<S>(&mut self, key: S) -> ContextEntry
    where
        S: Into<String>,
    {
        ContextEntry {
            entry: self.root.entry(key),
        }
    }

    pub fn as_value(&self) -> Value {
        Value::Object(self.root.clone())
    }
}

impl<T> From<T> for TemplateContext
where
    T: Serialize,
{
    fn from(t: T) -> Self {
        Self::new_with_root(t)
    }
}

impl From<&TemplateContext> for TemplateContext {
    fn from(c: &TemplateContext) -> Self {
        c.clone()
    }
}

static G_EXPR_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\{\{(.*?)\}\}").expect("build expr regex should success."));

pub struct RegexTemplateEngine {}

impl TemplateEngine for RegexTemplateEngine {
    fn parse(tpl: &str) -> Template {
        Template::new(
            tpl,
            G_EXPR_REGEX
                .find_iter(tpl)
                .map(|m| Match {
                    name: m
                        .as_str()
                        .trim_start_matches('{')
                        .trim_end_matches('}')
                        .trim()
                        .to_string(),
                    start: m.start(),
                    end: m.end(),
                })
                .collect(),
        )
    }
}

#[macro_export]
macro_rules! format_str {
    ($fmt:expr, $val:expr) => {{
        use $crate::TemplateEngine;
        let ctx = $val.into();
        let tpl = $crate::RegexTemplateEngine::parse($fmt);
        let s = $crate::RegexTemplateEngine::render(&ctx, &tpl);
        s
    }};
}

#[cfg(test)]
mod tests;
