{
  // Test that the garbage collector works correctly
  const root = {};

  const nested1 = {
    value: 5,
    nestedObject: {
      value: 6,
      otherNest: new Map([
        [1, 2],
        [3, 4],
      ]),
    },
  };
  root.nested1 = nested1;

  const nested2 = {
    value: 7,
    otherNest: {
      value: 8,
    },
  };
  root.nested2 = nested2;

  setRoot("test-root", root);
  commit();
}

// END_FILE

{
  const root = getRoot("test-root");
  if (!root) {
    throw new Error("root should exist");
  }
  if (!root.nested1) {
    throw new Error("root.nested1 should exist");
  }
  if (!root.nested1.nestedObject) {
    throw new Error("root.nested1.nestedObject should exist");
  }
  if (!root.nested2) {
    throw new Error("root.nested2 should exist");
  }
  if (!root.nested2.otherNest) {
    throw new Error("root.nested2.otherNest should exist");
  }

  commit();
}

// END_FILE

{
  // delete the nested1
  const root = getRoot("test-root");
  if (!root) {
    throw new Error("root should exist");
  }

  delete root.nested1;

  setRoot("test-root", root);
  commit();
}

// END_FILE

{
  // confirm nested1 is gone but nested2 is still there
  const root = getRoot("test-root");
  if (!root) {
    throw new Error("root should exist");
  }
  if (root.nested1) {
    throw new Error("root.nested1 should not exist");
  }
  if (!root.nested2) {
    throw new Error("root.nested2 should exist");
  }
  if (!root.nested2.otherNest) {
    throw new Error("root.nested2.otherNest should exist");
  }

  commit();
}
