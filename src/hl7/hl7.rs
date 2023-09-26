#[derive(Debug)]
pub struct Hl7Segment {
    pub header: String,
    pub contents: Vec<Hl7Field>,
}

impl Hl7Segment {
    /**
    Allows modification of a given field/subfield combinations within the HL7 Segment. Returns mutable self reference
    to allow method chaining.

    Example:
    segment.modify_field(1, 2, String::from("some data");

    OR

    segment.modify_field(1, 2, String::from("some data")
           .modify_field(1, 3, String::from("some additional data");
     */
    pub fn modify_field(
        &mut self,
        field_position: usize,
        sub_field_position: usize,
        new_data: String,
    ) -> &mut Hl7Segment {
        let mut field_position = field_position;
        let mut sub_field_position = sub_field_position;
        if field_position == 0 {
            field_position = 1;
        }
        if sub_field_position == 0 {
            sub_field_position = 1;
        }

        let field_to_modify = self.contents.get_mut(field_position - 1);
        if let Some(field_to_modify) = field_to_modify {
            let subfield_to_modify =
                field_to_modify.get_subfield_by_position(sub_field_position as i32);
            if let Some(subfield_to_modify) = subfield_to_modify {
                subfield_to_modify.data = new_data;
            } else {
                field_to_modify.sub_fields.push(Hl7Subfield {
                    position: sub_field_position.to_string(),
                    data: new_data,
                })
            }
        } else {
            let mut new_field = Hl7Field {
                position: field_position.to_string(),
                data: new_data.to_string(),
                sub_fields: vec![],
            };
            new_field.sub_fields.push(Hl7Subfield {
                position: sub_field_position.to_string(),
                data: new_data,
            });
            self.contents.push(new_field);
        }

        self
    }
}

#[derive(Debug)]
pub struct Hl7Field {
    pub position: String,
    pub data: String,
    pub sub_fields: Vec<Hl7Subfield>,
}

impl Hl7Field {
    pub fn get_subfield_by_position(
        &mut self,
        sub_field_position: i32,
    ) -> Option<&mut Hl7Subfield> {
        self.sub_fields
            .iter_mut()
            .find(|x| x.position == sub_field_position.to_string())
    }
}

#[derive(Debug)]
pub struct Hl7Subfield {
    pub position: String,
    pub data: String,
}

pub fn create_hl7_segment() -> Hl7Segment {
    Hl7Segment {
        header: "".to_string(),
        contents: vec![],
    }
}

pub fn create_hl7_field() -> Hl7Field {
    Hl7Field {
        position: "".to_string(),
        data: "".to_string(),
        sub_fields: vec![],
    }
}
