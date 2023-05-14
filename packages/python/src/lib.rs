use napi_derive::napi;
use lazy_static::lazy_static;
use std::collections::HashMap;
extern crate highlighter;
use highlighter::{HighlightConfiguration, build_config_with_regex};

lazy_static! {

  static ref CONFIG: (HighlightConfiguration, Vec<String>, Vec<String>, HashMap<&'static str, HighlightConfiguration>) = {
    let mut config = HighlightConfiguration::new(
        tree_sitter_python::language(),
        tree_sitter_python::HIGHLIGHTS_QUERY,
        tree_sitter_python::INJECTIONS_QUERY,
        tree_sitter_python::LOCALS_QUERY
    ).unwrap();
    let (injections, html_attrs, class_names) = build_config_with_regex(&mut config);
    return (config, html_attrs, class_names, injections)
  };
}

#[napi]
fn highlight(code: String) -> String {
    let (config, html_attrs, class_names, injections) = &*CONFIG;
    return highlighter::tree_sitter_highlight(code, config, html_attrs, class_names, injections);
}

#[napi]
fn highlight_hast(code: String) -> highlighter::HastNode {
    let (config, html_attrs, class_names, injections) = &*CONFIG;
    return highlighter::tree_sitter_highlight_hast(code, config, html_attrs, class_names, injections);
}