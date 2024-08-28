{
  // Test that the garbage collector works correctly

  const root = {};

  const nested1 = {
    value: new Map([
      [1, 2],
      ["a", "b"],
    ]),
  };
  root.nested1 = nested1;

  setRoot("test-root", root);
  commit();
}

{
  // confirm that the object is still there
  const root = getRoot("test-root");
  if (!root) {
    throw new Error("root should exist");
  }
  if (!root.nested1) {
    throw new Error("root.nested1 should exist");
  }
  if (!root.nested1.value) {
    throw new Error("root.nested1.value should exist");
  }
  if (!root.nested1.value.get(1)) {
    throw new Error("root.nested1.value.get(1) should exist");
  }
  if (!root.nested1.value.get("a")) {
    throw new Error("root.nested1.value.get('a') should exist");
  }

  commit();
}

{
  // delete the nested1
  const root = getRoot("test-root");
  if (!root) {
    throw new Error("root should exist");
  }

  delete root.nested1.value;

  transaction.setObject(root);
  transaction.setRoot("test-root", root);

  commit();
}

{
  const root = getRoot("test-root");
  if (!root) {
    throw new Error("root should exist");
  }
  if (!root.nested1) {
    throw new Error("root.nested1 should exist");
  }
  if (root.nested1.value) {
    throw new Error("root.nested1.value should not exist");
  }

  commit();
}
