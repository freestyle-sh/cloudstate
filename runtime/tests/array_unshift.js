{
  const arr = [1, 2, 3];

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
  if (root.value.length !== 3) {
    throw new Error("root.value should have length 3");
  }

  root.value.unshift(0);
  if (root.value.length !== 4) {
    throw new Error(
      "root.value should have length 4, but got " + root.value.length,
    );
  }
  if (root.value[0] !== 0) {
    throw new Error("root.value[0] should be 0");
  }
  //check last value
  if (root.value[3] !== 3) {
    throw new Error("root.value[3] should be 3");
  }
}
