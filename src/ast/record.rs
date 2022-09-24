#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Record {
    name: String,
    fields: Vec<Field>,
    line_number: usize,
}

pub const RECORD: &str = "record";

impl Record {
    fn new(name: &str, fields: Vec<Field>, line_number: usize) -> Record {
        Record {
            name: name.to_string(),
            fields,
            line_number,
        }
    }

    pub(crate) fn is_record(section: &ftd::p11::Section) -> bool {
        section.kind.as_ref().map_or(false, |s| s.eq(RECORD))
    }

    pub(crate) fn from_p1(section: &ftd::p11::Section, doc_id: &str) -> ftd::ast::Result<Record> {
        if !Self::is_record(section) {
            return ftd::ast::parse_error(
                format!("Section is not record section, found `{:?}`", section),
                doc_id,
                section.line_number,
            );
        }
        let fields = get_fields_from_headers(&section.headers, doc_id)?;
        Ok(Record::new(
            section.name.as_str(),
            fields,
            section.line_number,
        ))
    }

    #[allow(dead_code)]
    pub fn add_field(
        self,
        name: &str,
        kind: &str,
        value: Option<String>,
        line_number: usize,
    ) -> Record {
        let mut record = self;
        record
            .fields
            .push(Field::new(name, kind, value, line_number));
        record
    }
}

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Field {
    name: String,
    kind: String,
    value: Option<String>,
    line_number: usize,
}

impl Field {
    pub(crate) fn from_header(header: &ftd::p11::Header, doc_id: &str) -> ftd::ast::Result<Field> {
        match header {
            ftd::p11::Header::KV(ftd::p11::header::KV {
                line_number,
                key,
                kind,
                value,
            }) => {
                if let Some(kind) = kind {
                    Ok(Field::new(key, kind, value.to_owned(), *line_number))
                } else {
                    ftd::ast::parse_error(
                        format!("Can't find kind for record field: `{:?}`", key),
                        doc_id,
                        *line_number,
                    )
                }
            }
            ftd::p11::Header::Section(_) => unimplemented!(),
        }
    }

    fn new(name: &str, kind: &str, value: Option<String>, line_number: usize) -> Field {
        Field {
            name: name.to_string(),
            kind: kind.to_string(),
            value,
            line_number,
        }
    }
}

pub(crate) fn get_fields_from_headers(
    headers: &ftd::p11::Headers,
    doc_id: &str,
) -> ftd::ast::Result<Vec<Field>> {
    let mut fields: Vec<Field> = Default::default();
    for header in headers.0.iter() {
        fields.push(Field::from_header(header, doc_id)?);
    }
    Ok(fields)
}
