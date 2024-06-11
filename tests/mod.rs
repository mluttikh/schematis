use std::{fs::File, io::BufReader};

use schematis::Schema;

fn read_xsd(path: &str) -> Schema {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    Schema::from_reader(reader)
}

#[test]
fn deserialize_w3c_xml_schema() {
    let path = "tests/data/XMLSchema.xsd";
    let schema = read_xsd(path);
    assert_eq!(schema.annotations().len(), 8);
    assert_eq!(schema.attributes().len(), 0);
    assert_eq!(schema.simple_types().len(), 55);
    assert_eq!(schema.complex_types().len(), 35);
    assert_eq!(schema.attribute_groups().len(), 2);
    assert_eq!(schema.default_open_contents().len(), 0);
    assert_eq!(schema.elements().len(), 41);
    assert_eq!(schema.groups().len(), 12);
    assert_eq!(schema.imports().len(), 1);
    assert_eq!(schema.includes().len(), 0);
    assert_eq!(schema.notations().len(), 2);
    assert_eq!(schema.redefines().len(), 0);
    let simple_types = schema.simple_types();
    println!("{:#?}", simple_types);
    let simple_type = simple_types.first().unwrap();
    assert_eq!(simple_type.annotation().unwrap().namespace, None);
    println!("{:#?}", schema.complex_types().get(1).unwrap().annotation());
    println!("{:#?}", schema);
}

#[test]
fn deserialize_w3c_xml_schema_datatypes() {
    let path = "tests/data/XMLSchema-datatypes.xsd";
    let _schema = read_xsd(path);
}

#[test]
fn deserialize_w3c_ws_addr() {
    let path = "tests/data/ws-addr.xsd";
    let _schema = read_xsd(path);
}

#[test]
fn deserialize_oasis_t_1() {
    let path = "tests/data/t-1.xsd";
    let _schema = read_xsd(path);
}

#[test]
fn deserialize_oasis_b_2() {
    let path = "tests/data/b-2.xsd";
    let _schema = read_xsd(path);
}

#[test]
fn deserialize_oasis_br_2() {
    let path = "tests/data/br-2.xsd";
    let _schema = read_xsd(path);
}
