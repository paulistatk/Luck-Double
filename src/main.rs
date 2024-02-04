extern crate headless_chrome;
use headless_chrome::Browser;
use std::collections::HashSet;
use std::collections::HashMap;
use anyhow::Result;

fn main() -> Result<()> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    tab.navigate_to("https://namegentool.com/pt/cyberpunk-name-generator")?;

    let elements = tab.wait_for_elements("[data-name]")?;

    let mut all_data = HashSet::new();
    for element in elements {
        if let Ok(Some(attributes)) = element.get_attributes() {
            let mut attr_map = HashMap::new();
            for attr in attributes {
                let parts: Vec<&str> = attr.split('=').collect();
                if parts.len() == 2 {
                    attr_map.insert(parts[0].to_string(), parts[1].to_string());
                }
            }
            if let Some(name) = attr_map.get("data-name") {
                all_data.insert(name.clone());
            }
        }
    }

    // Print all unique occurrences of 'data-name'
    println!("{:?}", all_data);

    // Rest of your code...

    Ok(())
}
