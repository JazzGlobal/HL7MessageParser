use std::env;
use std::env::args;
use std::fs;
use std::fs::File;
use std::io::Write;
use chrono;
use chrono::Local;

#[derive(Debug)]
struct HL7_Segment {
    pub header: String,
    pub contents: Vec<HL7_Field>,
}

#[derive(Debug)]
struct HL7_Field {
    position: String,
    data: String,
    sub_fields: Vec<HL7_Subfield>,
}

#[derive(Debug)]
struct HL7_Subfield {
    position: String,
    data: String,
}

fn create_hl7_segment() -> HL7_Segment {
    HL7_Segment {
        header: "".to_string(),
        contents: vec![],
    }
}

fn create_hl7_field() -> HL7_Field {
    HL7_Field {
        position: "".to_string(),
        data: "".to_string(),
        sub_fields: vec![],
    }
}

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
        let mut new_segment = create_hl7_segment();
        let mut current_position = 1;

        for field in fields {
            if new_segment.header.is_empty() {
                new_segment.header = field.to_string();
                continue;
            }
            let subfields = field.split("^");
            let mut hl7_field = create_hl7_field();

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

    for i in hl7_message {
        println!("{}", i.header);
        output.push_str(&"\n");
        for field in i.contents {
            for subfield in field.sub_fields {
                if subfield.data.is_empty() {
                    continue;
                }
                let x = format!("{} {}.{} : {}",
                                i.header, field.position, subfield.position, subfield.data);
                println!("{}", &x);
                output.push_str(&"\n");
                output.push_str(&x);
            }
        }
    }
    let output_path = format!("output_{}.txt", Local::now().format("%Y-%m-%d %H%M%S").to_string());
    let mut file = File::create(&output_path).expect(&*format!("Could not create file at {}", output_path));
    file.write_all(output.as_bytes()).expect("Could not write to file.");
}