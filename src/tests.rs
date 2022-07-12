// Copyright (c) The Starcoin Core Contributors
// SPDX-License-Identifier: Apache-2.0

use super::*;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u8,
}
#[test]
fn test_format_str() {
    let person = Person {
        name: "alice".to_string(),
        age: 18,
    };
    let result = format_str!("name:{{$.name}},age:{{$.age}}", &person);
    assert_eq!("name:alice,age:18".to_string(), result);

    let result = format_str!("{{$.value}}", "alice");
    assert_eq!("alice".to_string(), result);

    let result = format_str!("{{$.value}}", 1);
    assert_eq!("1".to_string(), result);

    let mut ctx = Context::new();
    ctx.entry("name").set("alice");
    ctx.entry("age").set(18);
    let result = format_str!("name:{{$.name}},age:{{$.age}}", &ctx);
    assert_eq!("name:alice,age:18".to_string(), result);
}

#[test]
fn readme_example() {
    let json_value = json!({
        "my": {
            "name": "alice",
            "age": 18,
        },
        "friends": [
            {
                "name": "bob",
                "age": 18,
            },
            {
                "name": "tom",
                "age": 20,
            },
        ],
    });

    assert_eq!(
        "Hello, alice!".to_string(),
        format_str!("Hello, {{$.my.name}}!", &json_value)
    );

    assert_eq!(
        "Hello, bob!".to_string(),
        format_str!("Hello, {{$.friends[0].name}}!", &json_value)
    );

    assert_eq!(
        "Hello, tom!".to_string(),
        format_str!("Hello, {{$.friends[-1].name}}!", &json_value)
    );

    assert_eq!(
        "Hello, tom!".to_string(),
        format_str!("Hello, {{$.friends[?(@.age > 18)].name}}!", &json_value)
    );
}
