const { readFileSync, writeFileSync } = require("fs");
const lipstick = require("./index");

test("Test Python", () => {
  const code = readFileSync(`./example.py`, "utf8");
  const output = lipstick.highlightHast(code);
  let savedOutput = {};
  try {
    savedOutput = JSON.parse(readFileSync(`./example.json`, "utf8"));
  } catch {
    writeFileSync("./example.json", JSON.stringify(output));
  }
  try {
    expect(output).toEqual(savedOutput);
  } catch (err) {
    writeFileSync("./debug.json", JSON.stringify(output));
    throw err;
  }
});
