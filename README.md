# KMPM

[![github]](https://github.com/Tom-game-project/kmpm)&ensp;[![crates-io]](https://crates.io/crates/kmpm)&ensp;[![docs-rs]](https://docs.rs/kmpm/latest/kmpm/)

[github]: https://img.shields.io/badge/github-30363d?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-253323?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-2a2a2a?style=for-the-badge&labelColor=555555&logo=docs.rs

KMPM (Knuth-Morris-Pratt algorithm) library. KMPM is one of effective character query algorithm.

If the length of the text is n and the length of the pattern is m, the KMP algorithm processes in O(n+m) time

## Usage

Create new rust project,and add kmpm dependencies to Cargo.toml file.

Cargo.toml

```toml
[dependencies]
kmpm="0.2"
```

## Code Example

main.rs

```rs
use kmpm::kmpm_str;

fn main(){
  let text =    "hello world !";
  let pattern = "world";
  let ctr = kmpm_str(text, pattern);
  match ctr {
      Some(cursor)=>{
          println!("matched index {}",cursor)
      }
      None=>{
          println!("\"{}\" does not match",pattern);
      }
  }
}

```

```text
matched index 6

========================

"hello world !"
      "world"
 ------^^^^^
       |
       #6
```

## License

[MIT](https://github.com/Tom-game-project/kmpm/blob/master/LICENSE.MIT)
