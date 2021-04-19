# fmt-iter

[![Test](https://github.com/KSXGitHub/fmt-iter/workflows/Test/badge.svg)](https://github.com/KSXGitHub/fmt-iter/actions?query=workflow%3ATest)
[![Crates.io Version](https://img.shields.io/crates/v/fmt-iter?logo=rust)](https://crates.io/crates/fmt-iter)

Print all items from an iterator.

## Usage Examples

### Print all items from an array slice

```rust
use fmt_iter::FmtIter;
println!("{}", FmtIter::from(&[0, 12, 345]));
```

_Expected Output:_

```txt
012345
```

### Repeat a certain character multiple times

```rust
use fmt_iter::repeat;
println!("{}", repeat('x', 5));
```

_Expected Output:_

```txt
xxxxx
```

## License

[MIT](https://git.io/JOgrR) © [Hoàng Văn Khải](https://ksxgithub.github.io/).
