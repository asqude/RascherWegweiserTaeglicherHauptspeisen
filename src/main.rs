use scraper::{Html, Selector, ElementRef};

fn main() {
    let url = "https://www.studierendenwerk-aachen.de/speiseplaene/academica-w.html";
    let response = reqwest::blocking::get(url).expect("Failed to fetch menu");
    let html_content = response.text().expect("Failed to read html");
    let document = Html::parse_document(&html_content);

    // Search for the active headline which indicates the current day
    let headline_selector = Selector::parse(".active-headline").unwrap();
    
    // Find the panel associated with the active headline
    let mut target_panel: Option<ElementRef> = None;

    if let Some(headline) = document.select(&headline_selector).next() {
        // The menu table is usually in the "next sibling" div of the header's parent structure
        // We traverse siblings to find the next 'div'
        if let Some(parent) = headline.parent() {
            let mut found_headline = false;
            for child in parent.children() {
                // Wait until we pass the headline node
                if child.id() == headline.id() {
                    found_headline = true;
                    continue;
                }
                // The first div after the active headline is our target panel
                if found_headline {
                    if let Some(el) = ElementRef::wrap(child) {
                        if el.value().name() == "div" {
                            target_panel = Some(el);
                            break;
                        }
                    }
                }
            }
        }
    }

    if let Some(panel) = target_panel {
        let tr_selector = Selector::parse("tr").unwrap();
        let cat_selector = Selector::parse(".menue-category").unwrap();
        let desc_selector = Selector::parse(".menue-desc").unwrap();

        for tr in panel.select(&tr_selector) {
            let category_text = tr.select(&cat_selector)
                .next()
                .map(|el| el.text().collect::<String>())
                .unwrap_or_default();

            if category_text.contains("Klassiker") {
                 if let Some(desc_el) = tr.select(&desc_selector).next() {
                     // Collect text from all elements except <sup> tags
                     let mut parts: Vec<String> = Vec::new();
                     
                     fn collect_text_no_sup(node: ElementRef, parts: &mut Vec<String>) {
                         for child in node.children() {
                             if let Some(el) = ElementRef::wrap(child) {
                                 // Skip sup elements (contain allergen codes)
                                 if el.value().name() == "sup" {
                                     continue;
                                 }
                                 collect_text_no_sup(el, parts);
                             } else if let Some(text) = child.value().as_text() {
                                 let t = text.trim();
                                 if !t.is_empty() {
                                     parts.push(t.to_string());
                                 }
                             }
                         }
                     }
                     
                     collect_text_no_sup(desc_el, &mut parts);
                     
                     // Join the parts and clean up
                     let raw_text = parts.join(" ");
                     let clean_text = raw_text.replace("+", "").trim().to_string();
                     
                     // Split by "|" and keep first part + everything after "|"
                     let pipe_parts: Vec<&str> = clean_text.split('|').collect();
                     let formatted = if pipe_parts.len() >= 2 {
                         let first = pipe_parts[0].split_whitespace().next().unwrap_or("");
                         let rest: Vec<&str> = pipe_parts[1..].iter()
                             .map(|s| s.split_whitespace().next().unwrap_or(""))
                             .filter(|s| !s.is_empty())
                             .collect();
                         if rest.is_empty() {
                             first.to_string()
                         } else {
                             format!("{} | {}", first, rest.join(" | "))
                         }
                     } else {
                         clean_text.split_whitespace().next().unwrap_or("").to_string()
                     };
                     
                     println!("{}", formatted);
                     return;
                 }
            }
        }
        println!("No Klassiker found.");
    } else {
        eprintln!("active-headline not found.");
    }
}
