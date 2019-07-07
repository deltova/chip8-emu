use clap::{App, Arg, SubCommand};

pub struct Options {
    pub debug : bool,
    pub scale : u8,
    pub rom_path : String
}


pub fn parse_options() -> Option<Options> {
    let matches = App::new("CHIP8")
                        .version("1.0")
                        .author("Clement Magnard")
                        .about("Emulate Chip 8 behavior")
                        .arg(Arg::with_name("scale")
                                    .short("s")
                                    .long("scale")
                                    .value_name("SCALE")
                                    .help("Set scale size for the screen")
                                    .takes_value(true))
                        .arg(Arg::with_name("debug")
                                    .short("d")
                                    .long("debug")
                                    .help("Turn debugging information on"))
                        .arg(Arg::with_name("rom")
                                    .help("Input rom for the emulator")
                                    .index(1)
                                    .required(true))
                        .get_matches();

    let mut scale = 3;
    if let Some(o) = matches.value_of("scale") {
        scale = o.parse::<u8>().unwrap();
    }
    else {
        () 
    }

    let debug = matches.is_present("debug");

    let mut rom_path = String::from("");
    if let Some(c) = matches.value_of("rom") {
         rom_path = c.to_string();
    }
    else {
       () 
    }
    Some(Options{debug: debug, scale: scale, rom_path: rom_path})
}
