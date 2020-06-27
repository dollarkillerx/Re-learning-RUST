use std::fs::{File, OpenOptions};
use std::error::Error;
use std::io::{Read, BufReader, BufRead, Write};

fn main() -> Result<(),Box<dyn Error>>{
   // readAll
    test1()?;
    // read byte
    test2()?;
    // read lines
    test3()?;
    // write file
    test4()?;

    // openOptions
    test5()?;
    Ok(())
}

const TEST_DIR:&'static str = "Cargo.toml";

// readAll
fn test1() -> Result<(),Box<dyn Error>> {
    let mut file_point = File::open(TEST_DIR)?;
    let mut body = String::new();
    file_point.read_to_string(&mut body)?;

    println!("body: {}",body);

    Ok(())
}

fn test2() -> Result<(),Box<dyn Error>> {
    let mut file_point = File::open(TEST_DIR)?;
    let mut buffer = vec![0;8];
    file_point.read(&mut buffer)?;
    println!("ppc: {}",String::from_utf8_lossy(&buffer));

    for i in 0..buffer.len() {
        println!("idx: {}  val: {}",i,buffer[i] as char);
    }

    Ok(())
}

fn test3() -> Result<(),Box<dyn Error>> {
    let mut file_point = File::open(TEST_DIR)?;
    let reader = BufReader::new(file_point);
    for i in reader.lines() {
        match i {
            Ok(msg) => {
                println!("msg: {}",msg);
            },
            Err(e) => {
                println!("e: {}",e);
            },
        }
    }

    Ok(())
}

const TAG_FILE:&str = "coco.txt";
fn test4() -> Result<(),Box<dyn Error>> {
    let mut point = File::create(TAG_FILE)?; // if file is null,rust create else open file
    point.write_all("thi.s is sdasad t".as_bytes())?;
    Ok(())
}

fn test5() -> Result<(),Box<dyn Error>> {
    {
        // read file
        let mut open = OpenOptions::new().read(true).open(TAG_FILE)?;
        let mut body = String::new();

        open.read_to_string(&mut body)?;

        println!("Body: {}",body);
    }

    // write file
    let mut open = OpenOptions::new().read(true).write(true).open(TAG_FILE)?;
    let mut body = String::new();
    body.push_str("Fuck Beach \n");
    body.push_str("Fuck Beach \n");
    body.push_str("Fuck Beach \n");
    body.push_str("Fuck Beach \n");

    open.write_all(body.as_bytes());

    Ok(())
}