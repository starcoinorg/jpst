use jsonpath::Selector;
use multimap::MultiMap;
use regex::Regex;
use serde_json::{json, Value};

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
    pub(crate) fn new(template: &str, matchs: Vec<Match>) -> Self {
        Template {
            src: template.to_string(),
            matches: matchs,
        }
    }
}

pub trait TemplateEngine {
    fn parse(tpl: &str) -> Template;
    fn render(ctx: &Context, tpl: &Template) -> String;
}

pub struct Context {
    value: MultiMap<String, Value>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            value: MultiMap::new(),
        }
    }
    pub fn insert<T: Into<Value>>(&mut self, key: &str, value: T) {
        self.value.insert(key.to_string(), value.into());
    }

    pub fn to_value(&self) -> Value {
        let mut root = Value::Object(serde_json::Map::new());
        for (key, values) in self.value.iter_all() {
            if values.len() == 1 {
                root.as_object_mut()
                    .unwrap()
                    .insert(key.to_string(), values[0].clone());
            } else {
                let mut array = Value::Array(Vec::new());
                for value in values {
                    array.as_array_mut().unwrap().push(value.clone());
                }
                root.as_object_mut().unwrap().insert(key.to_string(), array);
            }
        }
        root
    }
}

pub struct RegexTemplateEngine {}

impl TemplateEngine for RegexTemplateEngine {
    fn parse(tpl: &str) -> Template {
        let regex = Regex::new(r"\{\{([^}]*)\}\}").unwrap();

        Template::new(
            tpl,
            regex
                .find_iter(tpl)
                .map(|m| Match {
                    name: m
                        .as_str()
                        .strip_prefix("{{")
                        .unwrap()
                        .strip_suffix("}}")
                        .unwrap()
                        .trim()
                        .to_string(),
                    start: m.start(),
                    end: m.end(),
                })
                .collect(),
        )
    }
    fn render(ctx: &Context, tpl: &Template) -> String {
        let mut src = tpl.src.clone();
        for m in &tpl.matches {
            let root = ctx.to_value();

            let selector = Selector::new(m.name.as_str()).unwrap();
            let value = selector.find(&root).next().unwrap().as_str().unwrap();
            src = src.replace(&tpl.src[m.start..m.end], value);
        }
        src
    }
}

#[test]
fn test_single() {
    let tpl = "Hello, {{$.name}}!";
    let mut ctx = Context::new();
    ctx.insert("name", "alice");
    let tpl = RegexTemplateEngine::parse(tpl);
    let result = RegexTemplateEngine::render(&ctx, &tpl);
    assert_eq!("Hello, alice!".to_owned(), result);
}

#[test]
fn test_multi() {
    let tpl = "Hello, {{$.name[1]}}!";
    let mut ctx = Context::new();
    ctx.insert("name", "alice");
    ctx.insert("name", "bob");
    let tpl = RegexTemplateEngine::parse(tpl);
    let result = RegexTemplateEngine::render(&ctx, &tpl);
    assert_eq!("Hello, bob!".to_owned(), result);
}

#[test]
fn test_embed_obj() {
    let tpl = "Hello, {{$.friends[1].name}}!";
    let mut ctx = Context::new();
    ctx.insert(
        "friends",
        json!({
                "name": "alice",
                "age": 18,
            }
        ),
    );
    ctx.insert(
        "friends",
        json!({
                "name": "bob",
                "age": 20,
            }
        ),
    );
    let tpl = RegexTemplateEngine::parse(tpl);
    let result = RegexTemplateEngine::render(&ctx, &tpl);
    assert_eq!("Hello, bob!".to_owned(), result);
}
