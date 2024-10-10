{
  const arr = [1, 2, 3, 4, 5];
  const object = {
    value: arr,
  };

  setRoot("test-root", object);
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
  if (root.value.length !== 5) {
    throw new Error("root.value should have length 5");
  }

  root.value.splice(2, 2);
  if (root.value.length !== 3) {
    throw new Error(
      "root.value should have length 3, but got " + root.value.length,
    );
  }
  if (root.value[0] !== 1) {
    throw new Error("root.value[0] should be 1");
  }
  //check last value
  if (root.value[2] !== 5) {
    throw new Error("root.value[2] should be 5");
  }
}
