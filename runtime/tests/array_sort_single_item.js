{
  const base = [1];
  const object = {
    value: [...base],
  };
  setRoot("test-root", object);
  commit();
}

// END_FILE

{
  const sortFn = (a, b) => b - a;
  const sortedExpectedArr = [1];

  const root = getRoot("test-root");
  if (!root) {
    throw new Error("root should exist");
  }
  if (!root.value) {
    throw new Error("root.value should exist");
  }
  if (root.value.length !== expectedArr.length) {
    throw new Error(`root.value should have length ${base.length}`);
  }
  if (!root.value.every((value, index) => value === expectedArr[index])) {
    throw new Error("root.value should be equal to expectedArr");
  }

  // sort the array in place
  root.value.sort(sortFn);
  if (
    !root.value.every((value, index) => value === sortedExpectedArr[index])
  ) {
    throw new Error("root.value should be equal to reversedExpectedArr");
  }

  commit();
}

// END_FILE

{
  const expectedArr = [1];

  // testing undefined sort function
  const root = getRoot("test-root");
  if (!root) {
    throw new Error("root should exist");
  }
  if (!root.value) {
    throw new Error("root.value should exist");
  }
  if (root.value.length !== expectedArr.length) {
    throw new Error(`root.value should have length ${expectedArr.length}`);
  }
  root.value.sort();

  if (!root.value.every((value, index) => value === expectedArr[index])) {
    throw new Error("root.value should be equal to expectedArr");
  }
}
