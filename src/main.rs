// Base64のエンコード/デコード処理を作る
fn main() { // 適当な文字列をBase64に変換して結果を表示して、デコード結果を表示 --- (*1)    
    let s = "hello!";
    let s_b64 = base64_encode(s);
    println!("{} => {}", s, s_b64);
    println!("{} => {}", s_b64, String::from_utf8(base64_decode(&s_b64)).unwrap()); 

    let s = "Rust";
    let s_b64 = base64_encode(s);
    println!("{} => {}", s, s_b64);
    println!("{} => {}", s_b64, String::from_utf8(base64_decode(&s_b64)).unwrap()); 

    let s = "1234567890";
    let s_b64 = base64_encode(s);
    println!("{} => {}", s, s_b64);
    println!("{} => {}", s_b64, String::from_utf8(base64_decode(&s_b64)).unwrap());  
            
    let s = "生姜焼き定食";
    let s_b64 = base64_encode(s);
    println!("{} => {}", s, s_b64);   
    println!("{} => {}", s_b64, String::from_utf8(base64_decode(&s_b64)).unwrap()); 

    let s = "文字列をBase64に変換して、デコードして復元する";
    let s_b64 = base64_encode(s);
    println!("{} => {}", s, s_b64);
    println!("{} => {}", s_b64, String::from_utf8(base64_decode(&s_b64)).unwrap()); 

}
// Base64エンコードを行う関数 --- (*2)
fn base64_encode(in_str: &str) -> String {
    // Base64の変換テーブルを1文字ずつに区切る --- (*3)
    let t = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let table: Vec<char> = t.chars().collect::<Vec<char>>();
    // 変換結果を保持する文字列 --- (*4)
    let mut result = String::new();
    // 入力文字列をバイト列に変換 --- (*5)
    let bin8 = in_str.as_bytes();
    // 繰り返し24bitごと(3文字ずつ)に処理する --- (*6)
    let cnt = bin8.len() / 3;
    //Rustらしくclosureを使ってみる
    (0..cnt)
        .into_iter()
        .for_each(|i| {            
            let n = i * 3; // 3文字(24bit)ずつ処理 --- (*7)
            let b24 = ((bin8[n+0] as usize) << 16) +
                      ((bin8[n+1] as usize) <<  8) +
                      (bin8[n+2] as usize);
            result.push(table[(b24 >> 18) & 0x3f]); // 6bitずつ変換 --- (*8)
            result.push(table[(b24 >> 12) & 0x3f]);
            result.push(table[(b24 >>  6) & 0x3f]);
            result.push(table[(b24) & 0x3f]);
        });
    /*
    for i in 0..cnt {
        let n = i * 3; // 3文字(24bit)ずつ処理 --- (*7)
        let b24 = ((bin8[n+0] as usize) << 16) +
                  ((bin8[n+1] as usize) <<  8) +
                  (bin8[n+2] as usize);
        //println!("{:?}", b24);
        //          ((bin8[n+2] as usize) <<  0);
        result.push(table[(b24 >> 18) & 0x3f]); // 6bitずつ変換 --- (*8)
        result.push(table[(b24 >> 12) & 0x3f]);
        result.push(table[(b24 >>  6) & 0x3f]);
        result.push(table[(b24) & 0x3f]);
       // result.push(table[(b24 >>  0) & 0x3f]);
    }*/
    // 3バイトずつに割り切れなかった余りの部分を処理 --- (*9)
    match bin8.len() % 3 {
        1 => {
            let b24 = (bin8[cnt*3] as usize) << 16;
            result.push(table[(b24 >> 18) & 0x3f]);
            result.push(table[(b24 >> 12) & 0x3f]);
            result.push_str("==");
        },
        2 => {
            let b24 = ((bin8[cnt*3+0] as usize) << 16) +
                      ((bin8[cnt*3+1] as usize) << 8);
            result.push(table[(b24 >> 18) & 0x3f]);
            result.push(table[(b24 >> 12) & 0x3f]);
            result.push(table[(b24 >>  6) & 0x3f]);
            result.push('=');
        },
        _ => {},
    }
    result
}
// Base64デコードを行う関数 --- (*2)
fn base64_decode(in_str: &str) -> Vec<u8> {
    // Base64の変換テーブルを1文字ずつに区切る --- (*3)
    let t = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    // 変換結果を保持する Vec<u8> --- (*4)
    let mut result = vec![];
    let cnt = in_str.len() / 4;
    //Rustらしくclosureを使ってみる
    (0..cnt)
        .into_iter()
        .for_each(|i| {            
            let str_dec = &in_str[i*4..(i*4)+4];     
            let b24 = (t.find(str_dec.chars().nth(0).unwrap_or('A')).unwrap_or(0) << 18) +
            (t.find(str_dec.chars().nth(1).unwrap_or('A')).unwrap_or(0) << 12) +
            (t.find(str_dec.chars().nth(2).unwrap_or('A')).unwrap_or(0) << 6) +
            (t.find(str_dec.chars().nth(3).unwrap_or('A')).unwrap_or(0)) ;    
            result.push((b24 >>16 & 0xff) as u8);
            result.push((b24 >>8 & 0xff) as u8);
            result.push((b24  & 0xff) as u8); 
        });
    /*
    for i in 0..cnt{
        let str_dec = &in_str[i*4..(i*4)+4];     
        let b24 = (t.find(str_dec.chars().nth(0).unwrap_or('A')).unwrap_or(0) << 18) +
        (t.find(str_dec.chars().nth(1).unwrap_or('A')).unwrap_or(0) << 12) +
        (t.find(str_dec.chars().nth(2).unwrap_or('A')).unwrap_or(0) << 6) +
        (t.find(str_dec.chars().nth(3).unwrap_or('A')).unwrap_or(0)) ;    
       
        result.push((b24 >>16 & 0xff) as u8);
        result.push((b24 >>8 & 0xff) as u8);
        result.push((b24  & 0xff) as u8); 
    }  */     
    result
 }
