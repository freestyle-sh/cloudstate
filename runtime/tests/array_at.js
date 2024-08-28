const base = ["a", "b", "c", "d", "e", "f"];

{
  const object = {
    value: [...base],
  };

  setRoot("test-root", object);
  commit();
}

{
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

  // check at for each item
  for (let i = 0; i < root.value.length; i++) {
    if (root.value.at(i) !== base[i]) {
      throw new Error(
        `value mismatch at index ${i}: ${root.value.at(i)} !== ${base[i]}`,
      );
    }
  }
}
