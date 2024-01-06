//! # kmpm
//! 
//! [![github]](https://github.com/Tom-game-project/kmpm)&ensp;[![crates-io]](https://crates.io/crates/kmpm)&ensp;[![docs-rs]](https://docs.rs/kmpm/latest/kmpm/)
//! 
//! [github]: https://img.shields.io/badge/github-30363d?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-253323?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-2a2a2a?style=for-the-badge&labelColor=555555&logo=docs.rs
//! 
//! KMPM (Knuth-Morris-Pratt algorithm) library. KMPM is one of effective character query algorithm.
//! 
//! If the length of the text is n and the length of the pattern is m, the KMP algorithm processes in O(n+m) time
//! 


/// # kmpm_str 
/// 
/// return the first and only match found
/// 
/// ```
/// use kmpm::kmpm_str;
/// 
/// fn main(){
///   let text =    "hello world !";
///   let pattern = "world";
///   let ctr = kmpm_str(text, pattern);
///   match ctr {
///       Some(cursor)=>{
///           println!("matched index {}",cursor)
///       }
///       None=>{
///           println!("\"{}\" does not match",pattern);
///       }
///   }
/// }
/// ```
/// 
/// ```text
/// matched index 6
/// 
/// ========================
/// 
/// "hello world !" :text
///       "world"   :pattern
///  ------^^^^^
///        |
///        #6 -> return 6
/// ```
pub fn kmpm_str(text:&str,pattern:&str)->Option<usize>{
    let sm_list = skipmap(pattern);
    let mut cursor:usize= 0;
    let mut prev_skip_step:usize = 0;
    let endpoint = text.chars().count() - pattern.chars().count();
    loop{
        let mut skipstep:usize = 1;
        let mut elseflag = false;
        for i in 0..pattern.chars().count()-prev_skip_step{
            if text.chars().nth(cursor+prev_skip_step+i)!=pattern.chars().nth(prev_skip_step+i){
                skipstep=sm_list[i];
                prev_skip_step = skipstep;
                elseflag=true;
                break;
            }
        }
        if !elseflag{
            return Some(cursor)
        }
        if cursor>endpoint{
            return None
        }
        else {
            cursor += skipstep;
        }
    }
}


/// # kmpm_str_all
/// 
/// find all matching.allow duplicates.
/// 
/// ## Example
/// 
/// ```
/// use kmpm::kmpm_str_all;
/// 
/// fn main(){
///   let text = "abababa";
///   let pattern = "aba";
///   let match_list = kmpm_str_all(text,pattern);
///   println!("output {:?}",match_list);
/// }
/// ```
/// 
/// ```text
/// output [0,2,4]
/// 
/// ========================
/// 
/// "abababa" :text
/// "aba"     :pattern
///  ^^^
///  |"aba" <- This matching is allowed
///  | ^^^
///  | |"aba"
///  | | ^^^
///  | | |
///  #0#2#4 -> return [0,2,4]
/// ```
/// 
pub fn kmpm_str_all(text:&str,pattern:&str)->Vec<usize>{
    let mut rarr:Vec<usize> = Vec::new();
    let sm_list = skipmap(pattern);
    let mut cursor:usize= 0;
    let mut prev_skip_step:usize = 0;
    let endpoint = text.chars().count() - pattern.chars().count();
    loop{
        let mut skipstep:usize = 1;
        let mut elseflag = false;
        for i in 0..pattern.chars().count()-prev_skip_step{
            if text.chars().nth(cursor+prev_skip_step+i)!=pattern.chars().nth(prev_skip_step+i){
                skipstep=sm_list[i];
                prev_skip_step = skipstep;
                elseflag = true;
                break;
            }
        }
        if !elseflag{
            rarr.push(cursor);
        }
        if cursor>endpoint{
            return rarr
        }else {
            cursor += skipstep;
        }
    }
}

/// # kmpm_str_nad
/// 
/// find all matching.not allow duplicates (nad).
/// 
/// ## Example
/// 
/// ```
/// use kmpm::kmpm_str_nad;
/// 
/// fn main(){
///   
/// }
/// ```
/// 
/// ```text
/// "abababa" :text
/// "aba"     :pattern :cursor_start = 0
///  ^^^
///  |"aba" <- This matching is **not** allowed
///  +-***
///  |  "aba"
///  +---^^^
///  |   |
///  #0  #4 -> return [0,4]
/// ```
/// 
pub fn kmpm_str_nad(text:&str,pattern:&str,cursor_start:usize)->Vec<usize>{
    todo!()
}


fn dup(txt0:&Vec<char>,txt1:&Vec<char>,gap:usize)->bool{
    let diflen = txt0.len()-gap;
    let dig_txt0 = &txt0[gap..];
    let dig_txt1 = &txt1[..diflen];
    dig_txt0==dig_txt1
}

fn capable_gap(txt0:&Vec<char>,txt1:&Vec<char>)->usize{
    let txt0_len = txt0.len();
    if txt0_len==0{
        return 1;
    }
    for i in 1..txt0_len{
        if dup(&txt0, &txt1, i){
            return i;
        }
    }
    txt0_len
}

fn skipmap(txt:&str)->Vec<usize>{
    let txt_vec:Vec<char>= txt.chars().collect();
    let arr:Vec<usize> = (0..txt.chars().count())
    .map(
        |i|capable_gap(
            &txt_vec[..i].to_vec(),
            &txt_vec
            )
        )
    .collect();
    return arr;
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test00(){
        let text =   "hello world";
        let pattern ="world";
        let ctr = kmpm_str(text, pattern);
        match ctr {
            Some(cursor)=>{
                assert!(cursor==6);
                println!("matched index {}",cursor)
            }
            None=>{
                println!("\"{}\" does not match",pattern);
            }
        }
    }

    #[test]
    fn test01(){
        let text =   "........................失敗...fosososos";
        let pattern ="失敗";
        let ctr = kmpm_str(text, pattern);
        match ctr {
            Some(cursor)=>{
                assert!(cursor==24);
                println!("matched index {}",cursor)
            }
            None=>{
                println!("\"{}\" does not match",pattern);
            }
        }
    }

    #[test]
    fn test02() {
        let text="hello こんにちは world 世界";
        let pattern = "こんにちは";

        let ctr = kmpm_str(text, pattern).unwrap();
        assert!(ctr==6);
    }

    #[test]
    fn test03(){
        let text="hello こんにちは world 世界";
        let pattern = "2024";

        assert!(kmpm_str(text, pattern).is_none());
    }

    #[test]
    fn test04(){
        let text="abababa";
        let pattern = "aba";

        println!("{:?}",kmpm_str_all(text, pattern));

    }
}
