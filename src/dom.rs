use std::collections::HashMap;
use std::fmt;

pub struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}

impl Node {
    pub fn new_text(text: String) -> Self {
        Self {
            children: vec![],
            node_type: NodeType::Text(text),
        }
    }

    pub fn new_elem(tag_name: String, attributes: AttrMap, children: Vec<Node>) -> Self {
        Self {
            children,
            node_type: NodeType::Element(ElementData {
                tag_name,
                attributes,
            }),
        }
    }

    fn p_tree(&self, indent: u8) -> String {
        match &self.node_type {
            NodeType::Text(text) => with_indent(indent, text.clone()),
            NodeType::Element(el_data) => {
                let mut element_string = with_indent(indent, format!("<{}", el_data.tag_name));

                // Sort keys alphabetically
                let mut v: Vec<(&String, &String)> = (&el_data.attributes).into_iter().collect();
                v.sort_by(|x, y| x.0.cmp(&y.0));

                for (attribute_name, attribute_value) in v {
                    element_string += &format!(" {}=\"{}\"", attribute_name, attribute_value);
                }

                element_string += ">";

                if self.children.len() > 0 {
                    element_string += "\n";
                    for child in &self.children {
                        element_string += &child.p_tree(indent + 1);
                        element_string += "\n";
                    }
                    element_string += &with_indent(indent, format!("</{}>", el_data.tag_name));
                } else {
                    element_string += &format!("</{}>", el_data.tag_name);
                }

                element_string
            }
        }
    }
}

fn with_indent(indent: u8, string: String) -> String {
    format!("{0: >1$}{2}", "", indent as usize * 2, string)
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.p_tree(0))
    }
}

enum NodeType {
    Text(String),
    Element(ElementData),
}

pub struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

pub type AttrMap = HashMap<String, String>;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_with_0_indent() {
        let res = with_indent(0, "test".to_string());
        assert_eq!(res, "test".to_string());
    }
    #[test]
    fn test_with_3_indent() {
        let res = with_indent(3, "test".to_string());
        assert_eq!(res, "      test".to_string());
    }

    #[test]
    fn test_display() {
        let el = ElementData {
            tag_name: "p".to_string(),
            attributes: HashMap::new(),
        };

        let node = Node {
            node_type: NodeType::Element(el),
            children: vec![],
        };

        assert_eq!(format!("{}", node), "<p></p>");
    }
    #[test]
    fn test_display_simple_nest() {
        let inner_el = ElementData {
            tag_name: "span".to_string(),
            attributes: HashMap::new(),
        };

        let inner_node = Node {
            node_type: NodeType::Element(inner_el),
            children: vec![],
        };

        let outer_el = ElementData {
            tag_name: "p".to_string(),
            attributes: HashMap::new(),
        };

        let outer_node = Node {
            node_type: NodeType::Element(outer_el),
            children: vec![inner_node],
        };

        assert_eq!(format!("{}", outer_node), "<p>\n  <span></span>\n</p>");
    }
    #[test]
    fn test_display_basic() {
        let mut attributes = HashMap::new();

        attributes.insert("width".to_string(), "100%".to_string());

        let el = ElementData {
            tag_name: "p".to_string(),
            attributes,
        };

        let inner_node = Node {
            node_type: NodeType::Text("Lorem Ipsum...".to_string()),
            children: vec![],
        };

        let top_node = Node {
            node_type: NodeType::Element(el),
            children: vec![inner_node],
        };

        assert_eq!(
            format!("{}", top_node),
            "<p width=\"100%\">\n  Lorem Ipsum...\n</p>"
        );
    }

    #[test]
    fn test_display_tree() {
        let inner_nodes: Vec<Node> = (0..3)
            .map(|i| {
                let text_node = Node::new_text((i + 1).to_string());
                let mut attrs = HashMap::new();
                attrs.insert("align".to_string(), "left".to_string());
                let node = Node::new_elem("li".to_string(), attrs, vec![text_node]);
                node
            })
            .collect();

        let middle_node = Node::new_elem("ul".to_string(), HashMap::new(), inner_nodes);

        let mut outer_attrs = HashMap::new();
        outer_attrs.insert("width".to_string(), "100%".to_string());
        outer_attrs.insert("onclick".to_string(), "func".to_string());

        let outer_node = Node::new_elem("div".to_string(), outer_attrs, vec![middle_node]);

        println!("{}", outer_node);

        assert_eq!(
            format!("{}", outer_node),
            "<div onclick=\"func\" width=\"100%\">\n  <ul>\n    <li align=\"left\">\n      1\n    </li>\n    <li align=\"left\">\n      2\n    </li>\n    <li align=\"left\">\n      3\n    </li>\n  </ul>\n</div>"
        );
    }
}
