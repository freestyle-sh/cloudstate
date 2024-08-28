const base = new Map([
  [
    "a",
    {
      a: 1,
      b: 2,
    },
  ],
  [
    "b",
    {
      a: 3,
      b: 4,
    },
  ],
  ["c", {}],
]);

const base_values = [...base.values()];

{
  const object = {
    value: base,
  };

  setRoot("test-root", object);
  commit();
}

{
  const root = getRoot("test-root");

  if (!root) throw new Error("root should exist");

  if (!root.value) throw new Error("root.value should exist");

  let i = 0;
  for (const v of root.value.values()) {
    if (v !== base_values[i]) {
      throw new Error("value does not match");
    }
    i++;
  }

  if (i !== base_values.length) {
    throw new Error(`Expected ${base_values.length} values, got ${i}`);
  }
}
