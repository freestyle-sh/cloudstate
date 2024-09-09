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

  let i = 0;
  for (const v of root.value) {
    if (v !== expected[i]) {
      throw new Error(`value mismatch at index ${i}: ${v} !== ${expected[i]}`);
    }
    i++;
  }

  if (i !== expected.length) {
    throw new Error("value iterator does not match");
  }

  commit();
}
