use std::env;
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

fn parse_xml_with_xml_rs(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Open the file and create a buffered reader
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut inside_peripheral = false;
    let mut wanted_peripheral = false;
    let mut inside_register = false;
    let mut inside_field = false;
    let mut inside_name = false;
    let mut inside_bit_offset = false;

    // Create an XML parser
    let parser = EventReader::new(reader);

    // Iterate through the events
    for event in parser {
        match event? {
            XmlEvent::StartElement {
                name, attributes, ..
            } => {
                // Handle start elements
                if name.local_name == "peripheral" {
                    inside_peripheral = true; // Entering a `field`
                }
                if name.local_name == "register" {
                    inside_register = true; // Entering a `field`
                }
                if name.local_name == "field" {
                    inside_field = true; // Entering a `field`
                }
                if name.local_name == "name" {
                    inside_name = true; // Entering a `field`
                }
                if name.local_name == "bitOffset" {
                    inside_bit_offset = true; // Entering a `field`
                }

                //println!("Start Element: {}", name.local_name);
                for _attr in attributes {
                    //println!("  Attribute: {} = {}", attr.name.local_name, attr.value);
                }
            }
            XmlEvent::EndElement { name } => {
                if name.local_name == "peripheral" {
                    if wanted_peripheral {
                        println!("}}");
                    }
                    inside_peripheral = false;
                    wanted_peripheral = false;
                }
                if name.local_name == "register" {
                    if wanted_peripheral {
                        println!("    }}");
                    }
                    inside_register = false;
                }
                if name.local_name == "field" {
                    inside_field = false;
                }
                if name.local_name == "name" {
                    inside_name = false;
                }
                if name.local_name == "bitOffset" {
                    inside_bit_offset = false;
                }
                // Handle end elements
                //println!("End Element: {}", name.local_name);
            }
            XmlEvent::Characters(content) => {
                // Handle text content
                if inside_peripheral && (!wanted_peripheral) && (!inside_register) && inside_name {
                    //println!("FOUND Peripheral: {}", content);
                    if content == "FLASH" {
                        wanted_peripheral = true;
                    }
                    if content == "RCC" {
                        wanted_peripheral = true;
                    }

                    if wanted_peripheral {
                        println!("pub mod {} {{", content);
                    }
                }
                if inside_peripheral
                    && wanted_peripheral
                    && inside_register
                    && (!inside_field)
                    && inside_name
                {
                    //println!("FOUND Register: {}", content);
                    println!("    pub mod {} {{", content.to_lowercase());
                }
                if inside_peripheral
                    && wanted_peripheral
                    && inside_register
                    && inside_field
                    && inside_name
                {
                    //println!("FOUND Field: {}", content);
                    println!("        pub const {} : u32 ", content);
                }
                if inside_peripheral
                    && wanted_peripheral
                    && inside_register
                    && inside_field
                    && inside_bit_offset
                {
                    //println!("FOUND Offset: {}", content);
                    println!("            = {};", content);
                }
                //println!("Text: {}", content);
            }
            _ => (), // Ignore other events
        }
    }

    Ok(())
}

fn main() {
    //let file_path = "STM32F405.svd";
    let args: Vec<String> = env::args().collect();

    // Validate that a file path argument was provided
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];


    println!("// DO NOT EDIT. This was generated by svd2rusty");
    println!("#![allow(unused)]");
    println!("#![allow(non_snake_case)]");
    println!("#![allow(non_upper_case_globals)]");

    if let Err(err) = parse_xml_with_xml_rs(file_path) {
        eprintln!("Error: {}", err);
    }
}
