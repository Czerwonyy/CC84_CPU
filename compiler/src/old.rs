/*
let me welcome you ladies and gentlemen
I would like to say hello

are you ready for some entertainment?
are you ready for this fucking shite?
*/

// this shit works thanks to duck tape and prayers

use core::panic;
use std::{env, fs::{self, File}, io::Write, process::exit};

mod other_args;
fn main()
{
    /*      Child Abuser
                ||
                ||
                ||
                ||
                \/     */
    let arg1: Vec<String> = env::args().collect();
    let mut count1: i8 = 0;
    let mut insertion = String::new();
    insertion.push(' '); // I don't know why tf this is here but yeah
    let mut file = String::new();
    for i in &arg1
    {
        if count1 == 1
        {
            file.push_str(i);
            let _ = file.trim();
        }
        count1 += 1;
    }
    if file == "-h" || file == "--help" // it doesn't help it's just so nobody dm's me
    {
        other_args::help(); // don't bother checking other file, it doesn't do shit
        exit(0);
    }
    let mut file_content = String::new();
    match fs::read_to_string(file)
    {
        Ok(content) => file_content.push_str(&content), // what kind of love have you got?
        Err(e) => println!("{}",e),
    }
    let mut output: Vec<String> = Vec::new();
    for i in file_content.lines()
    {
        let line: Vec<&str> = i.split(stringify!(,)).collect();
        if line[0].to_lowercase().trim() == "load a"
        {
            let num: i8 = line[1].trim().parse().expect("wrong number");
            let push_string = "0001 ".to_owned()+&format!("{:04b}",num);
            output.push(format!("{}{insertion}",push_string));
        }
        if line[0].to_lowercase().trim() == "load b"
        {
            let num: i8 = line[1].trim().parse().expect("wrong number");
            let push_string = "0010 ".to_owned()+&format!("{:04b}",num);
            output.push(format!("{}{insertion}",push_string)); // we 'bout to repeat it 'till we die, ain't we?
        }
        if line[0].to_lowercase().trim() == "load op"
        {
            let num: i8 = line[1].trim().parse().expect("wrong number");
            let bin_str = format!("{:04b}",num);
            let binary = &bin_str[bin_str.len().max(4)-4..]; // I was missing two dots like I'm missing her delightful eyes
            let push_string = "0011 ".to_owned()+&format!("{}",binary);
            output.push(format!("{}{insertion}",push_string));
        }
        if line[0].to_lowercase().trim() == "load mar" //it's like women, I can't fucking understand 'em
        {
            let num: i8 = line[1].trim().parse().expect("wrong address"); // yeah, wrong address, she should be at mines
            let push_string = format!("0100 {:04b}",num);
            output.push(format!("{}{insertion}",push_string));
        }
        if line[0].to_lowercase().trim() == "load ou"
        {
            output.push(format!("0101{insertion}"));
        }
        /* next couple of lines made me reconsider my life choices
         it made me think "what am I doing here?"
         next comparison made showed me that everything in life is dependent
         so if this code shows any errors then it is your fucking fault mate
         I don't give a flying fuck about syntax error
         if it ain't workin' then check your bloody syntax*/
        if line[0].to_lowercase().trim() == "mem in"
        {
            let num: i8 = line[1].trim().parse().expect("wrong number");
            let push_string = format!("0110 {:04b}",num);
            output.push(format!("{}{insertion}",push_string));
        }
        if line[0].to_lowercase().trim() == "mem ou"
        {
            output.push(format!("0111{insertion}"));
        }
        if line[0].to_lowercase().trim().chars().next() == Some('j')
        {
            let str: Vec<&str> = line[0].split_whitespace().collect();
            let num = u8::from_str_radix(str[1].trim(), 16).expect("wrong address");
            let push_string = format!("1000 {:04b}",num);
            output.push(format!("{}{insertion}",push_string));
        }
        if line[0].trim().to_lowercase() == "reset"
        {
            output.push(format!("1111{insertion}"));
        }
    }
    //println!("{:?}",output); // fuck this shit and loops
    // waaaaah waaaaah waaaaah
    // I'm a cold heartbreaker fit to burn and I'll rip your heart in two
    let mut let_me_hit = String::new();
    let mut elements: Vec<String> = Vec::new();
    for i in output
    {
        let element: Vec<String> = i.trim().split_whitespace().map(String::from).collect();
        elements.extend(element);
    }
    let mut count2: u8 = 0;
    let mut count3: u128 = 1;
    //print!("{:04X}: ",count3);
    let_me_hit.push_str(&format!("{:04X}:",count3));
    for i in elements
    {
        let num = u8::from_str_radix(&i, 2).expect("error"); // if you change it, it will go fuck
        // itself so be reasonable and do NOT give it to any woman
        let num_string = format!("{}",num);
        let fuck_ths_shit: u8 = num_string.parse().expect("error");
        //print!("{:04X} ",fuck_ths_shit);
        let_me_hit.push_str(&format!(" {:04X}",fuck_ths_shit));
        count2 += 1;
        if count2 == 16 // perfect 16 <3
        {
            //print!("\n{:04X}: ",count3);
            let_me_hit.push_str(&format!("\n{:04X}:",count3));
            count2 = 0;
            count3 += 1;
        }
    }
    if count2 != 0 || count2 != 16
    {
        for _ in 1..16-count2+1 // idk why but works only when +1 is here
        // so don't touch it
        {
            let_me_hit.push_str(" 0000");
        }
    }
    match File::create("output.save")
    {
        Ok(mut file) =>
        {
            println!("{}",let_me_hit);
            match file.write(format!("{}",let_me_hit).as_bytes())
            {
                Ok(_)=>{},
                Err(e)=>panic!("{}",e),
            }
        }
        Err(e)=>panic!("{}",e)
    }
}

/*
        / \
        | |
        | |
        | |
        | |
        | |
        | |
       /   \
       *****
        ***
         *
*/