{
  const base = new Map([
    ["a", 1],
    ["b", 2],
    ["c", 3],
    ["d", "a"],
    ["e", "b"],
    ["f", "c"],
  ]);
  const object = {
    value: base,
  };

  setRoot("test-root", object);
  commit();
}

// END_FILE

{
  const expected = new Map([
    ["a", 1],
    ["b", 2],
    ["c", 3],
    ["d", "a"],
    ["e", "b"],
    ["f", "c"],
  ]);
  const expectedValues = [...expected.values()];

  const root = getRoot("test-root");
  if (!root) {
    throw new Error("root should exist");
  }
  if (!root.value) {
    throw new Error("root.value should exist");
  }

  let i = 0;
  for (const v of root.value.values()) {
    if (v !== expectedValues[i]) {
      throw new Error("value does not match");
    }
    i++;
  }
  if (i !== expectedValues.length) {
    throw new Error("iterator length does not match");
  }
}
