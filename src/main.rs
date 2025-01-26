use std::env;
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

//use std::num;

//use num_traits::Num;

use convert_case::{Case, Casing};

fn parse_xml_with_xml_rs(
    file_path: &str,
    wanted: &[String],
    pass: u8,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut inside_peripheral = false;
    let mut wanted_peripheral = false;
    let mut inside_register = false;
    let mut inside_interrupt = false;
    let mut inside_field = false;
    let mut inside_enumerated_value = false;
    let mut inside_name = false;
    let mut inside_bit_offset = false;
    let mut inside_address_offset = false;
    let mut inside_base_ddress = false;
    let mut reg_name = "".to_string();

    let mut last_name = "BAD_NAME".to_string();
    let mut address_offset: i32 = 0;
    let mut prev_address_offset: i32 = 0;
    let mut reserved: i32 = 0;
    let mut register_name = "bad".to_string();

    let parser = EventReader::new(reader);

    for event in parser {
        match event? {
            XmlEvent::StartElement {
                name, attributes, ..
            } => {
                // Handle start elements
                if name.local_name == "peripheral" {
                    inside_peripheral = true;
                    reserved = 1;
                }
                if name.local_name == "baseAddress" {
                    inside_base_ddress = true;
                }
                if name.local_name == "register" {
                    inside_register = true;
                }
                if name.local_name == "interrupt" {
                    inside_interrupt = true;
                }
                if name.local_name == "field" {
                    inside_field = true;
                    inside_enumerated_value = false;
                }
                if name.local_name == "enumeratedValue" {
                    inside_enumerated_value = true;
                }
                if name.local_name == "name" {
                    inside_name = true;
                }
                if name.local_name == "bitOffset" {
                    inside_bit_offset = true;
                }
                if name.local_name == "addressOffset" {
                    inside_address_offset = true;
                }

                //println!("Start Element: {}", name.local_name);
                for _attr in attributes {
                    //println!("  Attribute: {} = {}", attr.name.local_name, attr.value);
                }
            }
            XmlEvent::EndElement { name } => {
                if name.local_name == "peripheral" {
                    if wanted_peripheral {
                        if pass == 1 {
                            println!("}}");
                        }
                        if pass == 2 {
                            println!("}}");
                        }
                    }
                    inside_peripheral = false;
                    wanted_peripheral = false;
                }

                if name.local_name == "baseAddress" {
                    inside_base_ddress = false;
                }

                if name.local_name == "register" {
                    if wanted_peripheral {
                        if pass == 1 {
                            println!("    }}");
                        }
                    }
                    inside_register = false;

                    if inside_peripheral && wanted_peripheral && (!inside_field) && (pass == 2) {
                        //println!("// ADDRESS prv={} curr={}", prev_address_offset,address_offset);

                        if (address_offset != 0 )  && (prev_address_offset + 4 > address_offset) {
                            println!("// BAD INPUT expected addr {} but got {}", prev_address_offset + 4, address_offset);
                        }

                        while prev_address_offset + 4 < address_offset {
                            println!("    reserved{} : u32 ,", reserved);
                            reserved += 1;
                            prev_address_offset += 4;
                        }

                        if register_name.to_ascii_lowercase() == "ccmr1_input".to_string() {
                            register_name = "ccmr1".to_string();
                        }
                        if register_name.to_ascii_lowercase() == "ccmr1_output".to_string() {
                            register_name = "".to_string();
                        }
                        if register_name.to_ascii_lowercase() == "ccmr2_input".to_string() {
                            register_name = "ccmr2".to_string();
                        }
                        if register_name.to_ascii_lowercase() == "ccmr2_output".to_string() {
                            register_name = "".to_string();
                        }

                        if register_name.len() > 0 {
                            println!("    pub {} : u32 ,", register_name.to_ascii_lowercase());
                            prev_address_offset = address_offset;
                        }
                    }
                }

                if name.local_name == "interrupt" {
                    inside_interrupt = false;
                }

                if name.local_name == "field" {
                    inside_field = false;
                }
                if name.local_name == "enumeratedValue" {
                    inside_enumerated_value = false;
                }
                if name.local_name == "name" {
                    inside_name = false;
                }
                if name.local_name == "bitOffset" {
                    inside_bit_offset = false;
                }
                if name.local_name == "addressOffset" {
                    inside_address_offset = false;
                }

                //println!("End Element: {}", name.local_name);
            }
            XmlEvent::Characters(mut content) => {
                if inside_peripheral
                    && (!inside_interrupt)
                    && (!wanted_peripheral)
                    && (!inside_register)
                    && inside_name
                {
                    content = content.to_ascii_uppercase();

                    for w in wanted {
                        // skip the - ones on pass 3
                        if w.starts_with('-') {
                            if pass == 3 {
                                continue;
                            }
                            if content == w[1..].to_string() {
                                wanted_peripheral = true;
                            }
                        }
                        if w.starts_with('+') {
                            if pass != 3 {
                                continue;
                            }
                            if content == w[1..].to_string() {
                                wanted_peripheral = true;
                            }
                        }
                        if content == *w {
                            wanted_peripheral = true;
                        }
                    }

                    if false {
                        if pass == 3 {
                            println!("//FOUND Peripheral: {}", content);
                        }
                    }

                    if wanted_peripheral {
                        let mut name = content.clone();
                        if name == "USART1" {
                            name = "USART".to_string();
                        }
                        if name == "USART2" {
                            name = "USART".to_string();
                        }
                        if name == "USART6" {
                            name = "USART".to_string();
                        }
                        if name == "DMA2" {
                            name = "DMA".to_string();
                        }
                        if name == "TIM1" {
                            name = "TIM_ADV".to_string();
                        }

                        if name == "TIM2" {
                            name = "TIM_GEN".to_string();
                        }
                        if name == "GPIOA" {
                            name = "GPIO".to_string();
                        }
                        if name == "GPIOB" {
                            name = "GPIO".to_string();
                        }
                        if name == "GPIOC" {
                            name = "GPIO".to_string();
                        }

                        //reg_name = name.to_ascii_uppercase().clone() + "Reg";
                        reg_name = name.to_case(Case::Pascal).clone() + "Reg";

                        if pass == 1 {
                            println!("");
                            println!("pub mod {} {{", name);
                        }
                        if pass == 2 {
                            println!("");
                            println!("#[repr(C)]");
                            println!("pub struct {} {{", reg_name);
                        }
                        if pass == 3 {
                            println!("");
                            println!("pub const {} :  *mut {} ", content, reg_name);
                        }
                    }
                }
                if inside_peripheral
                    && wanted_peripheral
                    && inside_register
                    && (!inside_field)
                    && inside_name
                {
                    //println!("FOUND Register: {}", content);
                    if pass == 1 {
                        println!("    pub mod {} {{", content.to_ascii_lowercase());
                    }
                    if pass == 2 {
                        register_name = content.clone();
                    }
                }
                if inside_peripheral
                    && wanted_peripheral
                    && inside_register
                    && inside_field
                    && inside_name
                    && !inside_enumerated_value
                {
                    if pass == 1 {
                        last_name = content.clone();
                        //println!("        pub const {} : u8 ", content);
                    }
                }

                if inside_peripheral
                    && wanted_peripheral
                    && inside_register
                    && !inside_field
                    && inside_address_offset
                    && !inside_enumerated_value
                {
                    let v = i32::from_str_radix(content.trim_start_matches("0x"), 16);
                    match v {
                        Ok(val) => {
                            address_offset = val;
                            //println!("// ### parse {}, got {}", content, val);
                        }
                        Err(err) => {
                            println!("// failed to parse {}, got {}", content, err);
                        }
                    }
                }

                if inside_peripheral
                    && wanted_peripheral
                    && inside_register
                    && inside_field
                    && inside_bit_offset
                {
                    //println!("FOUND Offset: {}", content);

                    //inside_field = false;

                    if pass == 1 {
                        println!("        pub const {} : u8 ", last_name);
                        println!("            = {};", content);
                    }
                }

                if inside_peripheral && wanted_peripheral && inside_base_ddress {
                    //println!("FOUND Offset: {}", content);
                    if pass == 3 {
                        println!(" = {} as *mut {};", content, reg_name);
                    }
                }

                //println!("Text: {}", content);
            }
            _ => (), // Ignore other events
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path: &str = &args[1];
    let wanted = &args[2..];

    println!("// DO NOT EDIT. This was generated by svd2rusty");
    println!(
        "//! This module provides definitions for various hardware registers and their fields."
    );
    println!("");
    println!("#![allow(unused)]");
    println!("#![allow(non_snake_case)]");
    println!("#![allow(non_upper_case_globals)]");

    if let Err(err) = parse_xml_with_xml_rs(file_path, wanted, 1) {
        eprintln!("Error: {}", err);
    }

    if let Err(err) = parse_xml_with_xml_rs(file_path, wanted, 2) {
        eprintln!("Error: {}", err);
    }

    if let Err(err) = parse_xml_with_xml_rs(file_path, wanted, 3) {
        eprintln!("Error: {}", err);
    }
}
