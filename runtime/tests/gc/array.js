{
  // Test that the garbage collector works correctly
  const root = {
    nested: {
      value: [1, 2, 3, 4],
      value2: [5, 6, 7, 8, 9],
    },
  };

  setRoot("test-root", root);
  commit();
}

{
  const root = getRoot("test-root");
  if (!root) {
    throw new Error("root should exist");
  }
  if (!root.nested) {
    throw new Error("root.nested should exist");
  }
  if (!root.nested.value) {
    throw new Error("root.nested.value should exist");
  }
  if (!root.nested.value2) {
    throw new Error("root.nested.value2 should exist");
  }
  if (root.nested.value.length !== 4) {
    throw new Error("root.nested.value should have length 4");
  }
  if (root.nested.value2.length !== 5) {
    throw new Error("root.nested.value2 should have length 5");
  }

  commit();
}

{
  // delete the nested.value array, but keep the nested.value2 array, that way we can test that the garbage collector works correctly
  const root = getRoot("test-root");
  if (!root) {
    throw new Error("root should exist");
  }

  delete root.nested.value;

  commit();
}
