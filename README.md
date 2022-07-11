# JPST

JSON Path String Template

```rust
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
```
