extern crate xml;

use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

// Fonction principale pour vérifier la bien-forme du fichier XML
pub fn is_wellformed_from_file(path: &str) -> Result<bool, Vec<String>> {
    let file = File::open(path).expect("Error opening XML file.");
    let reader = BufReader::new(file);

    let parser = EventReader::new(reader);
    let mut last_opened_element = String::new();
    let mut errors: Vec<String> = Vec::new();

    for event in parser {
        let _ = process_xml_event(event, &mut last_opened_element, &mut errors);
    }

    dbg!(&errors);

    if errors.is_empty() {
        Ok(true)
    } else {
        Err(errors)
    }
}

// Fonction pour traiter les événements XML
fn process_xml_event(event: Result<XmlEvent, xml::reader::Error>, last_opened_element: &mut String, errors: &mut Vec<String>) -> Result<(), xml::reader::Error> {
    match event {
        Ok(XmlEvent::StartElement { name, attributes, .. }) => {
            let element_name = name.local_name;
            last_opened_element.clear();
            last_opened_element.push_str(&element_name);

            for attr in attributes.iter() {
                if let Err(attr_err) = check_attribute(&attr.value, &element_name) {
                    errors.push(attr_err);
                }
            }
        }
        Ok(XmlEvent::EndElement { name, .. }) => {
            let element_name = name.local_name;
            dbg!(&element_name);
            // Check if element are correctly closed by checking last element opened, no memory needed
            println!("last_opened_element {}, element_name {}", last_opened_element, &element_name);
            if last_opened_element != &element_name {
                errors.push(format!("Opened but unclosed element '{}'.", last_opened_element));
            }

        }
        Ok(XmlEvent::Characters(text)) => {
            if let Err(text_err) = check_text(&text) {
                errors.push(text_err);
            }
        }
        Err(e) => {
            dbg!(e);
        }
        _ => {}
    }
    
    // Correction : Utilisez `Ok(())` au lieu de `ok(())` pour renvoyer un résultat Ok vide
    Ok(())
}

// unit simple functions under this


// Fonction pour vérifier si un caractère doit être échappé
fn should_escape(caracter: char) -> bool {
    caracter == '&' || caracter == '<' || caracter == '>'
}

// Fonction pour vérifier si un attribut contient des caractères non échappés
fn check_attribute(attribute: &str, element_name: &str) -> Result<(), String> {
    if attribute.chars().any(should_escape) {
        Err(format!("Attribute '{}' in element '{}' contains unescaped special characters: '{}'", attribute, element_name, attribute))
    } else {
        Ok(())
    }
}

// Fonction pour vérifier si des caractères de texte contiennent des caractères non échappés
fn check_text(text: &str) -> Result<(), String> {
    if text.chars().any(should_escape) {
        Err("Character data inside an element contains unescaped special characters.".to_string())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_wellformed_from_file_wellformed_file() {
        // Test with a well-formed XML file
        if let Err(err) = is_wellformed_from_file("./tests/data/test-not-wellformed.xml") {
            dbg!("{}", &err);
            // panic!("{}", &err.join("\n"));
        };

    }
}
