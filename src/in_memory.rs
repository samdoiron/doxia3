use std::collections::HashMap;
use crate::domain::{Title, Body, Page};
use crate::create_page::{CreatePageGateway};
use crate::list_pages::{ListPagesGateway, ListedPage};

struct InMemory {
    pages: HashMap<Title, Page>
}

impl InMemory {
    fn empty() -> Self {
        InMemory {
            pages: HashMap::new()
        }
    }
}

impl CreatePageGateway for InMemory {
    fn create_page(&mut self, page: Page) -> Result<(), &'static str> {
        self.pages.insert(page.title.clone(), page);
        Ok(())
    }
}

impl ListPagesGateway for InMemory {
    fn list_pages(&self) -> Result<Vec<ListedPage>, &'static str> {
        let listed_pages = self.pages.values().map(|domain_page| {
            ListedPage {
                title: domain_page.title.clone(),
                body: domain_page.body.clone(),
            }
        }).collect();
        Ok(listed_pages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_a_page() -> Result<(), &'static str> {
        let page = Page {
            title: Title::validate("Page title".to_owned()).unwrap(),
            body: Body::validate("Page body".to_owned()).unwrap(),
        };

        let expected_pages = vec! [
            ListedPage {
                title: page.title.clone(),
                body: page.body.clone(),
            }
        ];

        let mut in_memory = InMemory::empty();
        in_memory.create_page(page)?;
        let result = in_memory.list_pages();

        assert_eq!(Ok(expected_pages), result);
        Ok(())
    }
}
