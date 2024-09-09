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

  for (const expectIncluded of expected) {
    if (!root.value.includes(expectIncluded)) {
      throw new Error(`root.value should include ${expectIncluded}`);
    }
  }
  for (const notExpectIncluded of ["z", 0, { "a": 1 }]) {
    if (root.value.includes(notExpectIncluded)) {
      throw new Error(`root.value should not include ${notExpectIncluded}`);
    }
  }
}
