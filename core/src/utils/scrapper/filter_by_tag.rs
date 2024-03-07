use ego_tree::NodeRef;
use scraper::Node;

pub trait FilterByTag<'a> {
    fn filter_by_tag(self, tag: &'a str) -> impl Iterator<Item = NodeRef<'a, Node>>;
}

impl<'a, T> FilterByTag<'a> for T
where
    T: Iterator<Item = NodeRef<'a, Node>>,
{
    fn filter_by_tag(self, tag: &'a str) -> impl Iterator<Item = NodeRef<'a, Node>> {
        self.filter(move |x| {
            if let Some(elem) = x.value().as_element() {
                elem.name() == tag
            } else {
                false
            }
        })
    }
}
