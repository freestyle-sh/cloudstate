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
  const expectedEntries = [...expected.entries()];

  const root = getRoot("test-root");
  if (!root) {
    throw new Error("root should exist");
  }
  if (!root.value) {
    throw new Error("root.value should exist");
  }
  const actualEntries = [...root.value.entries()];
  if (actualEntries.length !== expectedEntries.length) {
    throw new Error(
      `root.value should have length ${expectedEntries.length}, got ${actualEntries.length}`,
    );
  }
  for (const [expectedKey, expectedVal] of expectedEntries) {
    const actualVal = root.value.get(expectedKey);
    if (actualVal !== expectedVal) {
      throw new Error(
        `root.value.get(${JSON.stringify(expectedKey)}) should be ${
          JSON.stringify(expectedVal)
        }, got ${JSON.stringify(actualVal)}`,
      );
    }
  }
}
