/// # kmpm
/// 
/// KMP(Knuth-Morris-Pratt algorithm) library
/// 
/// ## kmpm_str
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
/// 
/// // matched index 6
/// //
/// // "hello world !" :text
/// //       "world"   :pattern
/// //  ------^^^^^
/// //        |
/// //        #6 -> return 6
/// ```


/// # kmpm_str 
/// 
/// return the first and only match found
/// 
/// ```
/// 
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
/// find all matching
/// 
/// allow duplicates
/// 
/// ```
/// // "abababa" :text
/// // "aba"     :pattern
/// //  ^^^
/// //  |"aba" <- This match is allowed
/// //  | ^^^
/// //  | |"aba"
/// //  | | ^^^
/// //  | | |
/// //  #0#2#4 -> return [0,2,4]
/// ```
/// 
/// ## Example
/// 
/// ```
/// ```
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
/// find all matching
/// 
/// not allow duplicates (nad)
/// 
/// ```
/// // "abababa" :text
/// // "aba"     :pattern :cursor_start = 0
/// //  ^^^
/// //  |"aba" <- This matching is **not** allowed
/// //  +-***
/// //  |  "aba"
/// //  +---^^^
/// //  |   |
/// //  #0  #4 -> return [0,4]
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
