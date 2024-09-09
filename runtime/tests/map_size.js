const base = new Map([
  ["a", 1],
  ["b", 2],
  ["c", 3],
  ["d", "a"],
  ["e", "b"],
  ["f", "c"],
]);
const baseSize = base.size;

{
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
  const expectedSize = expected.size;

  const root = getRoot("test-root");
  if (!root) {
    throw new Error("root should exist");
  }
  if (!root.value) {
    throw new Error("root.value should exist");
  }
  if (root.value.size !== expectedSize) {
    throw new Error(
      `root.value should have size ${expectedSize}, got ${root.value.size}`,
    );
  }
}
