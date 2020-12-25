extern crate async_std;
extern crate iref;
extern crate json_ld;
extern crate json;

use async_std::task;
use iref::{Iri, IriBuf};
use json_ld::{JsonContext, NoLoader, Document, Object, Reference};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	// The JSON-LD document to expand.
	let doc = json::parse(r#"
		{
			"@context": {
				"name": "http://xmlns.com/foaf/0.1/name"
			},
			"@id": "https://www.rust-lang.org",
			"name": "Rust Programming Language"
		}
	"#).unwrap();

	let iri = Iri::new("http://xmlns.com/foaf/0.1/name").unwrap();
	let c = JsonContext::new(Some(iri));

	task::block_on(async {
		// Expansion.
		let expanded_doc = doc.expand::<JsonContext, _>(&c, &mut NoLoader).await?;

		// Reference to the `name` property.
		let name_property = Reference::Id(IriBuf::new("http://xmlns.com/foaf/0.1/name").unwrap());

		// Iterate through the expanded objects.
		for object in expanded_doc {
			if let Object::Node(node) = object.as_ref() {
				println!("node: {}", node.id().unwrap()); // print the `@id`
				for name in node.get(&name_property) { // get the names.
					println!("name: {}", name.as_str().unwrap());
				}
			}
		}
		Ok(())
	})
}
