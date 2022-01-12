use std::{
    fs::File,
    io::{prelude::Write, stdin, stdout, Read, Result},
};

fn main() -> Result<()> {
    let mut u_input = String::new();
    print!("Input a ciphertext: ");
    stdout().flush()?;
    stdin().read_line(&mut u_input)?;
    let mut hashtext = String::new();
    print!("Input a hash phrase: ");
    stdout().flush()?;
    stdin().read_line(&mut u_input)?;
    let u_input = u_input.as_bytes();
    let hashtext = hashtext.as_bytes();
    let mut file_final = Vec::new();
    for chr in hashtext.iter().enumerate() {
        let e_char: u16 = (*chr.1 as u16 * u_input[(u_input.len() - 1) % (chr.0 + 1)] as u16) - 1;
        file_final.push(e_char);
    }
    let mut file_handle = File::create(".env")?;
    let file_final = file_final.into_iter().map(|x| x.to_le_bytes());
    let file_final = file_final.fold(vec![], |mut acc, val| {
        acc.extend(val);
        acc
    });
    println!("{:?}", file_final);
    file_handle.write(&file_final)?;
    let mut file_handle = File::open(".env")?;
    decrypt(&mut file_handle, &u_input)?;
    drop(file_handle);
    Ok(())
}

fn decrypt(file_handle: &mut File, codephrase: &[u8]) -> Result<()> {
    let mut file = Vec::new();
    file_handle.read_to_end(&mut file)?;
    let mut file_final: Vec<char> = Vec::new();
    for (chr_num, enc_char) in file.clone().iter().enumerate() {
        println!("{}, {}", chr_num, enc_char);
    }
    println!("{:?}", file_final);
    Ok(())
}
