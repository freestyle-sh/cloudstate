{
  const base = ["a", "b", "c", "d", "e", "f"];
  const object = {
    value: [...base],
  };

  setRoot("test-root", object);
  commit();
}

// END_FILE

{
  const expected = ["a", "b", "c", "d", "e", "f"];
  const root = getRoot("test-root");
  if (!root) {
    throw new Error("root should exist");
  }
  if (!root.value) {
    throw new Error("root.value should exist");
  }
  if (root.value.length !== expected.length) {
    throw new Error(`root.value should have length ${expected.length}`);
  }

  // check at for each item
  for (let i = 0; i < root.value.length; i++) {
    if (root.value.at(i) !== expected[i]) {
      throw new Error(
        `value mismatch at index ${i}: ${root.value.at(i)} !== ${expected[i]}`,
      );
    }
  }
}
