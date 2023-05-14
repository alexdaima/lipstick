# 💄 Lipstick

Multi-language syntax highlighter. This repository tries to aggregate all the different repos that exist for performing
tree-sitter based syntax highlighting in a single place and applying relevant patches. While focussing on exporting functionality
to Node, it's also planned to export as Rust and WASM for other languages.

## Example

The following example shows how to highlight a string of code in Python and get the resulting HAST. The HAST can
then be iterated over in anyway you see fit, and apply your own styling.

### Javascript / Typescript

```typescript
const lipstick = require("lipstick-python");
const hast = lipstick.highlightHast("import math");
console.log("hast");
{
  type: "element",
  tagName: "span",
  properties: { "className": "source" },
  children: [
    {
      type: "element",
      tagName: "span",
      properties: { className: "include" },
      children: [{ type: "text", value: "import" }]
    },
    { type: "text", value: " " },
    {
      type: "element",
      tagName: "span",
      properties: { className: "variable" },
      children: [{ type: "text", value: "math" }]
    }
  ]
}
```

## Contributing

All languages within `parsers/<language>` are automatically populated from either
files within the `nvim-treesitter` repo or individually maintained tree-sitter repos.
The exact files copied can be seen in the `Makefile`.

Building each language can be done with:

```bash
# Python Example
make setup
make python
```
