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

{
  const object = {
    value: base,
  };

  setRoot("test-root", object);
  commit();
}

// END_FILE

{
  const expected = new Map([
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
  const expectedValues = [...expected.values()];

  const root = getRoot("test-root");
  if (!root) {
    throw new Error("root should exist");
  }
  if (!root.value) {
    throw new Error("root.value should exist");
  }

  let i = 0;
  for (const v of root.value.values()) {
    if (v.a !== expectedValues[i].a || v.b !== expectedValues[i].b) {
      throw new Error(
        `Expected ${JSON.stringify(expectedValues[i])}, got ${
          JSON.stringify(v)
        }`,
      );
    }
    i++;
  }
  if (i !== expectedValues.length) {
    throw new Error(`Expected ${expectedValues.length} values, got ${i}`);
  }
}
