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
  const expectedKeys = [...expected.keys()];

  const root = getRoot("test-root");
  if (!root) {
    throw new Error("root should exist");
  }
  if (!root.value) {
    throw new Error("root.value should exist");
  }

  let i = 0;
  for (const v of root.value.keys()) {
    if (v !== expectedKeys[i]) {
      throw new Error("value does not match");
    }
    i++;
  }

  if (i !== expectedKeys.length) {
    throw new Error("iterator length does not match");
  }
}
