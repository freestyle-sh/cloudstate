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

  let i = 0;
  for (const v of root.value) {
    if (v !== base[i]) {
      throw new Error(`value mismatch at index ${i}: ${v} !== ${base[i]}`);
    }
    i++;
  }

  if (i !== base.length) {
    throw new Error("value iterator does not match");
  }

  commit();
}
