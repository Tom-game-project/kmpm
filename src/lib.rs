
fn kmpm(text:&str,pattern:&str)->Option<usize>{
    let sm_list = skipmap(pattern);
    let mut cursor:usize= 0;
    let mut prev_skip_step:usize = 0;
    let endpoint = text.chars().count()-pattern.chars().count();
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

fn dup(txt0:&str,txt1:&str,gap:usize)->bool{
    let diflen = txt0.chars().count()-gap;
    let dig_txt0 = &txt0[gap..];
    let dig_txt1 = &txt1[..diflen];
    dig_txt0==dig_txt1
}

fn capable_gap(txt0:&str,txt1:&str)->usize{
    let txt0_len = txt0.chars().count();
    if txt0_len==0{
        return 1;
    }
    for i in 1..txt0_len{
        if dup(txt0, txt1, i){
            return i;
        }
    }
    txt0_len
}

fn skipmap(txt:&str)->Vec<usize>{
    let arr:Vec<usize> = (0..txt.chars().count())
    .map(
        |i|capable_gap(
            &txt[..i],
            txt
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
        let text =    "hello world";
        let pattern ="world";
        let ctr = kmpm(text, pattern);
        match ctr {
            Some(cursor)=>{
                println!("matcched index {}",cursor)
            }
            None=>{
                println!("\"{}\" does not match",pattern);
            }
        }
    }

    #[test]
    fn test01(){
        let text =    "文字列照会に失敗した場合いくつ飛ばせるかどうかを調べます";
        let pattern ="失敗";
        let ctr = kmpm(text, pattern);
        match ctr {
            Some(cursor)=>{
                println!("matcched index {}",cursor)
            }
            None=>{
                println!("\"{}\" does not match",pattern);
            }
        }
    }
}
