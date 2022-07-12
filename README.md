# JPST

JSON Path String Template

```rust

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

```
