mod values;
mod round;
mod key;
mod trans;

#[allow(dead_code)]
#[derive(Default)]
struct StepData {
    plain:u64,
    cipher:u64,
    decrypted:u64,
    l:[[u32;values::ROUND+1];2],
    r:[[u32;values::ROUND+1];2],
    k:[[u64;values::ROUND];2],
    c:[[u32;values::ROUND+1];2],
    d:[[u32;values::ROUND+1];2],
}

#[allow(dead_code)]
fn encrypt(message:u64, key:u64, round:usize, cipher:&mut u64, _stepdata:&mut StepData)->i32{
    println!("*****Encrypt*****");
    println!("plain = {:064b}",message);
    let mut l:u32 = 0; let mut r:u32 = 0;
    let mut newl:u32 = 0; let mut newr:u32 = 0;
    trans::trans_IP(message, &mut l, &mut r);
    let mut c:u32 = 0; let mut d:u32 = 0;
    let mut newc:u32 = 0; let mut newd:u32 = 0;
    key::init_key(key, &mut c, &mut d);
    #[cfg(feature="DEBUG_WHOLE")]
    {
        _stepdata.plain = message;
        _stepdata.l[0][0] = l;
        _stepdata.r[0][0] = r;
        _stepdata.c[0][0] = c;
        _stepdata.d[0][0] = d;
    }
    let mut newk:u64 = 0;
    for i in 0..round {
        key::key_manage(i, c, d, &mut newk, &mut newc, &mut newd);
        c = newc; d = newd;
        round::round(l, r, newk, &mut newl, &mut newr);
        l = newl; r = newr;
        #[cfg(feature="DEBUG_WHOLE")]
        {
            _stepdata.l[0][i+1] = newl;
            _stepdata.r[0][i+1] = newr;
            _stepdata.k[0][i] = newk;
            _stepdata.c[0][i+1] = newc;
            _stepdata.d[0][i+1] = newd;
        }
    }
    trans::trans_inv_IP(r, l, cipher);
    println!("cipher = {:064b}",*cipher);
    #[cfg(feature="DEBUG_WHOLE")]
    {
        _stepdata.cipher = *cipher;
    }
    println!("*****End Encrypt*****");
    return 0;
}

#[allow(dead_code)]
fn decrypt(cipher:u64, key:u64, round:usize, message:&mut u64, _stepdata:&mut StepData)->i32{
    println!("*****Decrypt*****");
    println!("cipher = {:064b}",cipher);
    let mut l:u32 = 0; let mut r:u32 = 0;
    let mut newl:u32 = 0; let mut newr:u32 = 0;
    trans::trans_IP(cipher, &mut r, &mut l);
    let mut c:u32 = 0; let mut d:u32 = 0;
    let mut newc:u32 = 0; let mut newd:u32 = 0;
    key::init_key(key, &mut c, &mut d);
    #[cfg(feature="DEBUG_WHOLE")]
    {
        _stepdata.l[1][round] = l;
        _stepdata.r[1][round] = r;
        _stepdata.c[1][round] = c;
        _stepdata.d[1][round] = d;
    }
    
    let mut newk:u64 = 0;
    for i in (0..round).rev() {
        key::inv_key_manage(i, c, d, &mut newk, &mut newc, &mut newd);
        c = newc; d = newd;
        round::round(r, l, newk, &mut newr, &mut newl);
        l = newl; r = newr;
        #[cfg(feature="DEBUG_WHOLE")]
        {
            _stepdata.l[1][i] = newl;
            _stepdata.r[1][i] = newr;
            _stepdata.k[1][i] = newk;
            _stepdata.c[1][i] = newc;
            _stepdata.d[1][i] = newd;
        }
    }
    trans::trans_inv_IP(l, r, message);
    println!("message = {:064b}",*message);
    #[cfg(feature="DEBUG_WHOLE")]
    {
        _stepdata.decrypted = *message;
    }
    println!("*****End Decrypt*****");
    return 0;
}

#[cfg(feature="DEBUG_WHOLE")]
#[allow(dead_code)]
fn print_step(stepdata:StepData, round:usize)->i32{
    println!("*****Result*****");
    println!("Round Value");
    print!("\t|              L               ||              R               |");
    println!("\t|              L               ||              R               |");
    for i in 0..round+1 {
        print!("Round{}",i);
        print!("\t{:032b}{:032b}",stepdata.l[0][i],stepdata.r[0][i]);
        print!("\t{:032b}{:032b}",stepdata.l[1][i],stepdata.r[1][i]);
        if stepdata.l[0][i] == stepdata.l[1][i] {
            print!("\tSame");
        }else {
            print!("\tDiff");
        }
        if stepdata.r[0][i] == stepdata.r[1][i] {
            println!("Same");
        }else {
            println!("Diff");
        }
    }
    print!("\t|              L               ||              R               |");
    println!("\t|              L               ||              R               |");

    println!("Key Value");
    for i in 0..round {
        print!("Key{}",i);
        print!("\t{:048b}\t{:048b}",stepdata.k[0][i],stepdata.k[1][i]);
        if stepdata.k[0][i] == stepdata.k[1][i] {
            println!("\tSame");
        }else {
            println!("\tDiff");
        }
    }

    println!("Final Result");
    println!("plain\t\t= {:064b}",stepdata.plain);
    println!("cipher\t\t= {:064b}",stepdata.cipher);
    println!("decrypted\t= {:064b}",stepdata.decrypted);
    if stepdata.plain == stepdata.decrypted {
        println!("Success : Correct message output!");
    }else {
        println!("Fail : The value of the output message is incorrect...");
    }
    println!("*****End Result*****");
    return 0;
}

fn main() {
    let mut message:u64 = 0b0001111101100000100000000000000110011101010011101100101010111000;
    let key:u64 = 0b000011000110100000000011110000001000010000001000;
    let mut cipher:u64 = 0;
    let mut stepdata = StepData::default();
    encrypt(message, key, values::ROUND, &mut cipher, &mut stepdata);
    decrypt(cipher, key, values::ROUND, &mut message, &mut stepdata);
    // print_step(stepdata, values::ROUND);
    return;
}