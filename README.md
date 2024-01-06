# KMPM

KMPM (Knuth-Morris-Pratt algorithm) library. KMPM is one of most effective Character query algorithm.

## Crate

[crate](https://crates.io/crates/kmpm)

## Usage

Create new rust project.and add kmpm dependencies to Cargo.toml file.

Cargo.toml

```toml
[dependencies]
kmpm="0.2.0"
```

## Code example

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

//
// output >> matched index 6
//
// "hello world !"
//       "world"
//  ------^^^^^
//        |
//        #6
```

## License

[MIT](/LICENSE.MIT)