{
  const arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

  const obj = {
    value: arr,
  };

  setRoot("test-root", obj);
}

// END_FILE

{
  const root = getRoot("test-root");
  if (!root) {
    throw new Error("root should exist");
  }
  if (!root.value) {
    throw new Error("root.value should exist");
  }
  if (root.value.length !== 10) {
    throw new Error("root.value should have length 10");
  }

  const sliced = root.value.slice(2, 5);
  if (sliced.length !== 3) {
    throw new Error("sliced should have length 3");
  }
  if (sliced[0] !== 3) {
    throw new Error("sliced[0] should be 3");
  }
  if (sliced[1] !== 4) {
    throw new Error("sliced[1] should be 4");
  }
  if (sliced[2] !== 5) {
    throw new Error("sliced[2] should be 5");
  }
}
