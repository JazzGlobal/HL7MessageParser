#[derive(Debug)]
pub struct HL7_Segment {
    pub header: String,
    pub contents: Vec<HL7_Field>,
}

impl HL7_Segment {
    /// Allows modification of a given field/subfield combinations within the HL7 Segment.
    pub fn modify_field(&mut self, field_position: usize, sub_field_position: usize, new_data: String) -> &mut HL7_Segment {
        if sub_field_position - 1 < 0 {
            println!("Cannot modify a subfield that is lower than 1.");
            return self;
        }

        if field_position - 1 < 0 {
            println!("Cannot modify a field that is lower than 1.");
            return self;
        }

        let mut field_to_modify = self.contents.get_mut(field_position - 1);
        if let Some(field_to_modify) = field_to_modify {
            let mut subfield_to_modify = field_to_modify.sub_fields.get_mut(sub_field_position - 1);
            if let Some(subfield_to_modify) = subfield_to_modify {
                subfield_to_modify.data = new_data;
            }
            else {
                field_to_modify.sub_fields.push(HL7_Subfield { position: sub_field_position.to_string(), data: new_data.to_string() })
            }
        }
        else {
            let mut new_field = HL7_Field {
                position: field_position.to_string(),
                data: new_data.to_string(),
                sub_fields: vec![],
            };
            new_field.sub_fields.push(HL7_Subfield { position: sub_field_position.to_string(), data: new_data.to_string() });
        }

        self
    }
}

#[derive(Debug)]
pub struct HL7_Field {
    pub position: String,
    pub data: String,
    pub sub_fields: Vec<HL7_Subfield>,
}

#[derive(Debug)]
pub struct HL7_Subfield {
    pub position: String,
    pub data: String,
}

pub fn create_hl7_segment() -> HL7_Segment {
    HL7_Segment {
        header: "".to_string(),
        contents: vec![],
    }
}

pub fn create_hl7_field() -> HL7_Field {
    HL7_Field {
        position: "".to_string(),
        data: "".to_string(),
        sub_fields: vec![],
    }
}