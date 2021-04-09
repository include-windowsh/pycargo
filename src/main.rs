use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;
fn create_dir(name: String) {
    match fs::create_dir_all(String::from(name)) {
        Err(_e) => {
            println!("can't create dir");
            std::process::exit(1);
        }
        Ok(_) => {}
    }
}
fn main() {
    let args: Vec<String> = std::env::args().collect();

    //if new poject
    if (match args.get(1) {
        Some(p) => p,
        None => "null",
    }) == String::from("new")
    {
        println!("Create new package: {}", args[2].clone());
        let poject_name = &match args.get(2) {
            None => {
                println!("dont give poject name");
                std::process::exit(1)
            }
            Some(p) => p,
        };
        create_dir(format!("./{}/src", poject_name));

        let mut main_py = fs::File::create(format!("./{}/src/main.py", poject_name)).unwrap();

        main_py.write_all("import lib".as_bytes()).unwrap();
        fs::File::create(format!("./{}/src/lib.py", poject_name)).unwrap();

        let mut cargo_file = fs::File::create(format!("./{}/Cargo.toml", poject_name)).unwrap();
        cargo_file
            .write_all((format!("[name]\n{}\n[version]\n0.1.0", poject_name)).as_bytes())
            .unwrap();
    }
    //if is cargo run
    else if (match args.get(1) {
        Some(p) => p,
        None => "",
    }) == &String::from("run")
    {
        let cargofile = match fs::File::open("Cargo.toml") {
            Ok(e) => e,
            Err(_t) => {
                println!("can not open file");
                std::process::exit(1)
            }
        };
        //        let cargo
        //        println!("{}", cargofile.read());
        let cargo_toml = BufReader::new(cargofile);
        let mut cargo: Vec<String> = Vec::new();
        for i in cargo_toml.lines() {
            cargo.push(i.unwrap())
        }
        if cargo.len() == 4 {
        } else {
            println!("it is not a Cargo file in pycargo");
            std::process::exit(1)
        }
        println!("info:in Cargofile:{:?}", cargo);
        let package_name = if cargo[0] == String::from("[name]") {
            cargo[1].clone()
        } else {
            std::string::String::new()
        };
        let package_version = if cargo[2] == String::from("[version]") {
            cargo[3].clone()
        } else {
            std::string::String::new()
        };
        println!(
            "Info:package_name:{},package_version:{}\n---------------------running--------------------",
            package_name, package_version
        );

        //python run
        std::process::Command::new("python3")
            .arg("./src/main.py")
            .status()
            .expect("no python or other error");
        //if is run [Start]
    } else if (match args.get(1) {
        Some(p) => p,
        None => "null",
    }) == String::from("build")
    {
        let cargofile = match fs::File::open("Cargo.toml") {
            Ok(e) => e,
            Err(_t) => {
                println!("can not open file");
                std::process::exit(1)
            }
        };
        let cargo_toml = BufReader::new(cargofile);
        let mut cargo: Vec<String> = Vec::new();
        for i in cargo_toml.lines() {
            cargo.push(i.unwrap())
        }
        if cargo.len() == 4 {
        } else {
            println!("it is not a Cargo file in pycargo");
            std::process::exit(1)
        }
        println!("\n\ninfo:in Cargofile:{:?}", cargo);
        let package_name = if cargo[0] == String::from("[name]") {
            cargo[1].clone()
        } else {
            std::string::String::new()
        };
        let package_version = if cargo[2] == String::from("[version]") {
            cargo[3].clone()
        } else {
            std::string::String::new()
        };
        println!(
            "info:package_name:{},package_version:{}\n---------------running--------------",
            package_name, package_version
        );
        match std::process::Command::new("pyinstaller")
            .arg("./src/main.py")
            .arg("-F")
            .arg(format!("--name={}", package_name))
            .arg("--distpath")
            .arg("./")
            .status()
        {
            Err(_e) => {
                println!("error maybe is you don't install pyinstaller(use 'pip install pyinstaller' to install)");
                std::process::exit(1)
            }
            Ok(_q) => {}
        }
        //don't finish rename[4.4 19:12]!!!!!!!!!!!!(finish4.5 14.06)

    }
    //if is run[End]

    //if noting input[Start]
    else {
        println!("pycargo Great!\nCreate new poject usage:\n    pycargo new [poject_name]\nRun poject:\n    pycargo run\nversion:")
    }
    //if nothing input[End]
    //println!("args:{:?}", args);
}

