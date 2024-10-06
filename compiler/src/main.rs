/* duck tape got fucked
we're left with prayers */

use std::{env, fs, io::Write};

fn main()
{
    // declaring some shit
    let args: Vec<String> = env::args().collect();
    let mut file_content = String::new();
    let mut output_raw: Vec<String> = Vec::new();
    
    // fs - fucking slaves, hebrews born to serve
    // to the pharaoh
    match fs::read_to_string(args[1].trim())
    {
        Ok(content)=>
        {
            file_content.push_str(&content);
        },
        Err(e) => panic!("{}",e),
    }

    for i in file_content.lines()
    {
        let line: Vec<&str> = i.trim().split(',').collect();
    
        if line[0].to_lowercase().trim() == "load a" // where's your crown king nothing?
        {
            let num: u8 = line[1].trim().parse().unwrap();
            output_raw.push(format!("{:02X}01",num));
        }
        if line[0].to_lowercase().trim() == "load b"
        {
            let num: u8 = line[1].trim().parse().unwrap();
            output_raw.push(format!("{:02X}02",num));
        }
        if line[0].to_lowercase().trim() == "load op"
        {
            let num: u8 = line[1].trim().parse().unwrap();
            output_raw.push(format!("{:02X}03",num));
        }
        if line[0].to_lowercase().trim() == "load mar"
        {
            let num: u8 = line[1].trim().parse().unwrap();
            output_raw.push(format!("{:02X}04",num));
        }
        if line[0].to_lowercase().trim() == "load ou"
        {
            output_raw.push("0005".to_string());
        }
        if line[0].to_lowercase().trim() == "mem in"
        // why doesn't romanian woman shave her pussy?
        // cuz it's the only fur she can afford
        //
        // and why can queen in chess move freely on the board?
        // 'cause it looks like a kitchen floor
        {
            let num: u8 = line[1].trim().parse().unwrap();
            output_raw.push(format!("{:02X}06",num));
        }
        if line[0].to_lowercase().trim() == "mem ou"
        {
            output_raw.push("0007".to_string());
        }
        if line[0].to_lowercase().trim().chars().next() == Some('j')
        {
            // jmp - jump from the bridge
            let line_jmp: Vec<&str> = line[0].split_whitespace().collect();
            let u_address = u8::from_str_radix(line_jmp[1], 16).unwrap();
            /* while reading line above you might me wondering "why tf would someone convert hex to decimal just
            to convert it to hex again", yes, I am fucking stupid but it is so ROM file is clean and fun to watch
            (so it doesn't look like your mother's pussy) */
            output_raw.push(format!("{:02X}08",u_address));
        }
        if line[0].to_lowercase().trim() == "reset"
        {
            output_raw.push("000F".to_string());
        }
    }
    // declaring some more shit like USA declared war on Iraq
    let mut count: u32 = 0;
    let mut lines: u32 = 0;
    let mut output = String::new();

    print!("{:04X}:",lines);
    output.push_str(&format!("{:04X}:",lines));

    for i in &output_raw // &fuck_my_life
    // idk how code below works, it's just so it doesn't fuck up your CPU
    {
        if count == 16
        {
            print!("\n");
            count = 0;
            lines += 1;
            // it looks like fucking chinese
            print!("{:04X}:",lines);
            output.push_str(&format!("{:04X}",lines));
        }
        print!(" {}",i);
        output.push_str(&format!(" {}",i));
        count += 1;
    }
    if count != 0 || count != 16
    {
        for _ in 1..16-count+1 // +1 again cuz why fucking not
        {
            print!(" 0000");
            output.push_str(" 0000");
        }
    }

    match fs::File::create("output.save") // in case ya'll wondering where it saves your 
    // pussy ass code
    {
        Ok(mut f)=>
        {
            let _ = f.write(format!("{output}").as_bytes());
        }
        Err(e)=>panic!("{}",e), // it panics just like you when talking to bitches
    }    
}