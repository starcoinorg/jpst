use super::*;

#[test]
fn tests() {
    let mut ctx = Context::new();
    ctx.entry("my").set(json!({"name": "alice"}));
    ctx.entry("friends").append(json!({
            "name": "bob",
            "age": 18,
        }
    ));
    ctx.entry("friends").append(json!({
            "name": "tom",
            "age": 20,
        }
    ));

    test_case(&ctx, case("Hello, {{$.my.name}}!", "Hello, alice!"));
    //failed
    test_case(&ctx, case("Hello, {{name}}!", "Hello, {{name}}!"));
    //contains space
    test_case(&ctx, case("Hello, {{ $.my.name }}!", "Hello, alice!"));
    test_case(&ctx, case("Hello, {{$.friends[0].name}}!", "Hello, bob!"));
    test_case(&ctx, case("Hello, {{$.friends[-1].name}}!", "Hello, tom!"));
    test_case(&ctx, case("Age: {{$.friends[0].age}}", "Age: 18"));

    test_case(
        &ctx,
        case(
            "Hello, {{$.friends[0]}}!",
            "Hello, {\"age\":18,\"name\":\"bob\"}!",
        ),
    );
}

#[derive(Debug, Clone, PartialEq)]
pub struct Case {
    pub tpl: &'static str,
    pub expect: &'static str,
}

fn case(tpl: &'static str, expect: &'static str) -> Case {
    Case { tpl, expect }
}

fn test_case(ctx: &Context, case: Case) {
    let tpl = RegexTemplateEngine::parse(case.tpl);
    let result = RegexTemplateEngine::render(ctx, &tpl);
    assert_eq!(case.expect.to_owned(), result, "tpl: {}", case.tpl);
}
