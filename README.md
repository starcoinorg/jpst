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

```
