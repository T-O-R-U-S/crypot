use std::{
    io::{
        Read,
        Result,
        stdin,
        stdout,
        prelude::Write,
    },
    fs::{
        File
    },
};

fn main() -> Result<()> {
    let mut u_input = String::new();
    print!("Input a codephrase: ");
    stdout().flush()?;
    stdin().read_line(&mut u_input)?;
    let u_input = u_input.as_bytes();
    let mut file_handle = File::open(".env")?;
    let mut file = Vec::new();
    file_handle.read_to_end(&mut file)?;
    let mut file_final = Vec::new();
    for chr in file.iter().enumerate() {
        let e_char = (*chr.1 * u_input[(u_input.len()) % (chr.0+1)])+u_input[(u_input.len()) % (chr.0+1)];
        file_final.push(e_char);
    }
    let mut file_handle = File::create(".env")?;
    file_handle.write(&file_final)?;
    let mut file_handle = File::open(".env")?;
    decrypt(&mut file_handle,&u_input)?;
    drop(file_handle);
    Ok(())
}

fn decrypt(file_handle: &mut File, codephrase:&[u8]) -> Result<()> {
    let mut file = Vec::new();
    file_handle.read_to_end(&mut file)?;
    let mut file_final:Vec<char> = Vec::new();
    for (chr_num, enc_char) in file.clone().iter().enumerate() {
        println!("{}, {}", chr_num, enc_char);
    }
    println!("{:?}", file_final);
    Ok(())
}
