use treexml::Document;

fn to_xml() -> Result<(), Error> {
    let doc_raw = reader("input/ui.xml")?;
    let doc = Document::parse(doc_raw.as_bytes()).unwrap();
    let root = doc.root.unwrap();

    println!("{:#?}", root);

    Ok(())
}
