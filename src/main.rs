// #![allow(unused_variables)]
// #![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
#![cfg_attr(debug_assertions, allow(unused_variables, unused_assignments))]


use std::process::*;
// use std::process::Command;
use std::{io, env};
use std::path::{Path, PathBuf};
use std::collections::{HashMap, LinkedList};
use std::ffi::{OsStr, OsString};

#[allow(dead_code)]
fn print_typename<T>(_: T) { 
    println!("{}", std::any::type_name::<T>());
}

// TODO: auto generate outputfilepath from input filepath
// TODO: create command line arguments for inkscape from arg_pattern
// TODO: replace arguments with PLACEHOLDER
// TODO: check input != output




fn main() -> io::Result<()> {
    // `-> io::Reult<()>` for  `wait()?`

    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    // println!("{:?}", args.len());

    if args.len() != 2 {
        println!("Usage : {} input filename to convert to EMF", &args[0]);
        exit(1);         // std::process::exit(1);
    }

    let infile = &args[1];
    let (of, _) = make_outfilename(&infile);
    let outfile = of.to_string_lossy();
    // let outfile = of.to_string_lossy();
    // print_typename(&outfile); // -> &alloc::borrow::Cow<str>


    let availables : HashMap<&str, Vec<String>> = HashMap::from([
        (r"C:\Program Files\Inkscape\inkscape.exe", 
            vec![
                String::from(infile),
                "--export-emf".to_string(),
                outfile.to_string()
                ]),
        (r"C:\Program Files\Inkscape\bin\inkscape.exe", 
            vec![String::from(infile),
                "--export-filename".to_string(),
                outfile.to_string()
                ]),
    ]);

    // let availables : HashMap<char, Vec<String>> = HashMap::from([
    //     (r"C:\Program Files\Inkscape\inkscape.exe", 
    //         LinkedList::from([
    //             infile, 
    //             &String::from("--export-emf"), 
    //             &String::from(outfile.to_string_lossy())]) ),
    //     (r"C:\Program Files\Inkscape\bin\inkscape.exe", 
    //         LinkedList::from([
    //             infile, 
    //             &"--export-filename".to_string(), 
    //             &outfile.into_os_string().into_string().unwrap()]) ),
    // ]);

    print_typename(&availables);

    // let foundapp : &str;
    // let arg_pattern : &LinkedList<&String>;

    // for (key, val) in &availables {
    //     if Path::new(&key).exists() {
    //         foundapp = key;
    //         arg_pattern = val;
    //         break; // 見つかったらそくbreakするので、mutいらない。
    //     }
    // }


    // println!("{}", availables[r"C:\Program Files\Inkscape\inkscape.exe"][0]);


    let mut foundapp = "";
    let mut inkscape_args : Vec<String> = vec![];


    for (app, argvec) in availables.into_iter() {
        let pb = Path::new(app);
        if pb.exists() {
            // println!("found {}", key);
            foundapp = app;
            inkscape_args = argvec;

        }
    }

    if foundapp == "" {
        println!("Inkscape not found");
        exit(1);
    }


    let exit_status = if cfg!(target_os = "windows") {

        println!("convert to test.emf");

        Command::new(&foundapp)
                // .args([&infile, "--export-filename", &outfile])
                .args(inkscape_args)
                .spawn()
                .expect("failed to execute process")
                .wait()?

    } else {
        println!("Supports windows only");
        exit(1);
    };


    // println!("{}", exit_status);
    match exit_status.code() {
        Some(code) => {
            println!("Exit {}", code);
            exit(code);
        },
        None => println!("terminated")
    }


    exit(2); // should be unreached.

    // Ok(())
}


fn make_outfilename(infile: &String) -> (PathBuf, OsString) {
    let infile_path = Path::new(infile);
    let mut pb = PathBuf::from(infile_path);
    pb.set_extension("emf");
    let infile_ext = infile_path.extension().unwrap_or_else(|| OsStr::new(""));
    
    (pb, OsString::from(infile_ext))
}
