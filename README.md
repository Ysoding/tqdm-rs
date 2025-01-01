### Type-Driven API Design in Rust

learn from [video](https://www.youtube.com/watch?v=bnnacleqg6k)

```rust
fn expensive_op() {
    thread::sleep(Duration::from_secs(1));
}

fn main() {
    let delims = ('{', '}');
    let v = [1, 2, 3, 4];
    for _ in (0..).progress() {
        expensive_op();
    }

    // v.iter().progress().with_delims(delims).with_bound()
    for _ in v.iter().progress().with_bound().with_delims(delims) {
        expensive_op();
    }
}
```