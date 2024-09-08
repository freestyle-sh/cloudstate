{
  const base = [4, 7, 2, 6, 1, 3, 5, 10, 8, 9];
  const object = {
    value: [...base],
  };
  setRoot("test-root", object);
  commit();
}

// END_FILE

{
  const sortFn = (a, b) => b - a;
  const base = [4, 7, 2, 6, 1, 3, 5, 10, 8, 9];

  const root = getRoot("test-root");
  if (!root) {
    throw new Error("root should exist");
  }
  if (!root.value) {
    throw new Error("root.value should exist");
  }
  if (root.value.length !== base.length) {
    throw new Error(`root.value should have length ${base.length}`);
  }
  if (!root.value.every((value, index) => value === base[index])) {
    throw new Error("root.value should be equal to base");
  }

  // sort in place
  root.value.sort(sortFn);
  const expectedArr = [...base].reverse();
  if (!root.value.every((value, index) => value === expectedArr[index])) {
    throw new Error("root.value should be equal to expectedArr");
  }

  commit();
}

// END_FILE

{
  const sortFn = (a, b) => b - a;
  const base = [4, 7, 2, 6, 1, 3, 5, 10, 8, 9];
  const expectedArr = base.sort(sortFn);

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
}
