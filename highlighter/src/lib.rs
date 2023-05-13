pub use tree_sitter_highlight::{Highlighter, HighlightConfiguration, HtmlRenderer, HighlightEvent};
use napi_derive::napi;
use napi::bindgen_prelude::*;
use std::collections::HashMap;


fn add_highlight_names(config: &HighlightConfiguration, highlight_names: &mut Vec<String>) {
  for name in config.query.capture_names() {
    if !highlight_names.contains(name) {
      highlight_names.push(name.clone());
    }
  }
}

fn get_attrs(highlight_names: &Vec<String>) -> (Vec<String>, Vec<String>) {
  let html_attrs: Vec<String> = highlight_names
    .iter()
    .map(|s| format!("class=\"{}\"", s.replace('.', " ")))
    .collect();

  let class_names: Vec<String> = highlight_names
    .iter()
    .map(|s| s.replace('.', " "))
    .collect();

  (html_attrs, class_names)
}

pub fn build_config_with_regex(config: &mut HighlightConfiguration) -> (HashMap<&'static str, HighlightConfiguration>, Vec<String>, Vec<String>) {
  let mut highlight_names = Vec::new();
  add_highlight_names(config, &mut highlight_names);

  let mut regex_config = HighlightConfiguration::new(
    tree_sitter_regex::language(),
    tree_sitter_regex::HIGHLIGHTS_QUERY,
    "",
    "",
  ).unwrap();
  add_highlight_names(&regex_config, &mut highlight_names);

  config.configure(&highlight_names);
  regex_config.configure(&highlight_names);

  let (html_attrs, class_names) = get_attrs(&highlight_names);
  let mut injections = HashMap::new();
  injections.insert("regex", regex_config);
  (injections, html_attrs, class_names)
}

pub fn tree_sitter_highlight(
  code: String,
  config: &HighlightConfiguration,
  html_attrs: &Vec<String>,
  _class_names: &Vec<String>,
  injections: &HashMap<&'static str, HighlightConfiguration>
) -> String {
  let mut highlighter = Highlighter::new();
  let highlights = highlighter.highlight(
    &config,
    code.as_bytes(),
    None,
    |lang| injections.get(lang)
  ).unwrap();

  let mut renderer = HtmlRenderer::new();
  renderer.render(highlights, code.as_bytes(), &|highlight| html_attrs[highlight.0].as_bytes()).unwrap();
  unsafe { String::from_utf8_unchecked(renderer.html) }
}

#[derive(Debug)]
#[napi(object)]
pub struct HastProperties {
  pub class_name: String
}

#[derive(Debug)]
#[napi(object)]
pub struct HastNode {
  #[napi(js_name = "type")]
  pub kind: String,
  pub tag_name: String,
  pub properties: HastProperties,
  pub children: Vec<Either<HastNode, HastTextNode>>
}

#[derive(Debug)]
#[napi(object)]
pub struct HastTextNode {
  #[napi(js_name = "type")]
  pub kind: String,
  pub value: String
}

pub fn tree_sitter_highlight_hast(
  code: String,
  config: &HighlightConfiguration,
  _html_attrs: &Vec<String>,
  class_names: &Vec<String>,
  injections: &HashMap<&'static str, HighlightConfiguration>
) -> HastNode {
  let mut highlighter = Highlighter::new();
  let highlights = highlighter.highlight(
    &config,
    code.as_bytes(),
    None,
    |lang| injections.get(lang)
  ).unwrap();

  let mut stack = Vec::new();
  stack.push(HastNode {
    kind: "element".into(),
    tag_name: "span".into(),
    properties: HastProperties {
      class_name: "source".into()
    },
    children: Vec::new()
  });

  for event in highlights {
    match event.unwrap() {
      HighlightEvent::HighlightStart(highlight) => {
        let node = HastNode {
          kind: "element".into(),
          tag_name: "span".into(),
          properties: HastProperties {
            class_name: class_names[highlight.0].clone()
          },
          children: Vec::new()
        };
        stack.push(node);
      }
      HighlightEvent::Source {start, end} => {
        let slice = &code[start..end];
        let parent = stack.last_mut().unwrap();
        if let Some(Either::B(text_node)) = parent.children.last_mut() {
          text_node.value.push_str(slice);
        } else {
          let text_node = HastTextNode {
            kind: "text".into(),
            value: slice.into()
          };
          parent.children.push(Either::B(text_node));
        }
      }
      HighlightEvent::HighlightEnd => {
        let node = stack.pop().unwrap();
        let parent = stack.last_mut().unwrap();
        parent.children.push(Either::A(node));
      }
    }
  }

  stack.pop().unwrap()
}