use crate::types::{TaxonId, TaxonomyTree};
use std::collections::HashMap;

pub fn tree(tree: &TaxonomyTree) -> String {
    let mut children: HashMap<TaxonId, Vec<TaxonId>> = HashMap::new();
    let names: HashMap<TaxonId, String> = tree
        .nodes
        .iter()
        .map(|record| (record.id, sanitize_label(&record.name)))
        .collect();

    for edge in &tree.edges {
        children
            .entry(edge.parent.id)
            .or_default()
            .push(edge.child.id);
    }

    for child_ids in children.values_mut() {
        child_ids.sort_unstable();
    }

    format!("{};", render_node(tree.root_id, &children, &names))
}

fn render_node(
    id: TaxonId,
    children: &HashMap<TaxonId, Vec<TaxonId>>,
    names: &HashMap<TaxonId, String>,
) -> String {
    let label = names.get(&id).cloned().unwrap_or_else(|| id.to_string());
    match children.get(&id) {
        Some(child_ids) if !child_ids.is_empty() => {
            let rendered = child_ids
                .iter()
                .map(|child_id| render_node(*child_id, children, names))
                .collect::<Vec<_>>()
                .join(",");
            format!("({rendered}){label}")
        }
        _ => label,
    }
}

fn sanitize_label(label: &str) -> String {
    label
        .chars()
        .map(|ch| match ch {
            '\'' | '"' | '(' | ')' | ',' | ':' | ';' => '_',
            ch if ch.is_whitespace() => '_',
            ch => ch,
        })
        .collect()
}
