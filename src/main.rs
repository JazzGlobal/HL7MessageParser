use chrono;
use chrono::Local;
use std::env;
use std::env::args;
use std::fs;
use std::fs::File;
use std::io::Write;

pub mod hl7;

use hl7::hl7::{HL7_Field, HL7_Segment, HL7_Subfield};

fn main() {
    if args().len() < 2 {
        panic!("Expected an argument for FILE_PATH");
    }

    let file_path = args().nth(1).unwrap();
    let contents = fs::read_to_string(&file_path).expect(&*format!(
        "Error reading file \"{}\", does it exist?",
        file_path
    ));
    let lines = contents.lines();

    let mut hl7_message: Vec<HL7_Segment> = vec![];

    for line in lines {
        let fields = line.split('|');
        let mut new_segment = hl7::hl7::create_hl7_segment();
        let mut current_position = 1;

        for field in fields {
            if new_segment.header.is_empty() {
                new_segment.header = field.to_string();
                continue;
            }
            let subfields = field.split("^");
            let mut hl7_field = hl7::hl7::create_hl7_field();

            let mut sub_position = 1;
            for subfield in subfields {
                hl7_field.data = field.to_string();
                hl7_field.position = current_position.to_string();
                hl7_field.sub_fields.push(HL7_Subfield {
                    position: sub_position.to_string(),
                    data: subfield.to_string(),
                });
                sub_position += 1;
            }
            new_segment.contents.push(hl7_field);
            current_position += 1;
        }
        hl7_message.push(new_segment);
    }

    let mut output = "".to_string();

    for i in &hl7_message {
        println!("{}", &i.header);
        output.push_str(&"\n");
        for field in &i.contents {
            for subfield in &field.sub_fields {
                if subfield.data.is_empty() {
                    continue;
                }
                let x = format!(
                    "{} {}.{} : {}",
                    &i.header, field.position, subfield.position, subfield.data
                );
                println!("{}", &x);
                output.push_str(&"\n");
                output.push_str(&x);
            }
        }
    }
    let output_path = format!(
        "output_{}.txt",
        Local::now().format("%Y-%m-%d %H%M%S").to_string()
    );
    let mut file =
        File::create(&output_path).expect(&*format!("Could not create file at {}", output_path));
    file.write_all(output.as_bytes())
        .expect("Could not write to file.");
}
