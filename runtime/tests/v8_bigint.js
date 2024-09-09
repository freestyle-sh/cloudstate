{
  const base = BigInt("1234567890123456789012345678901234567890");
  const object = {
    value: base,
  };
  setRoot("test-root", object);
  commit();
}

// END_FILE

{
  const expected = BigInt("1234567890123456789012345678901234567890");
  const object = getRoot("test-root");
  if (!object) {
    throw new Error("object should exist");
  }
  if (!object.value) {
    throw new Error("object.value should exist");
  }
  if (typeof object.value !== "bigint") {
    throw new Error("object.value should be a bigint");
  }
  if (object.value !== expected) {
    throw new Error(
      `Expected ${JSON.stringify(expected)}, got ${
        JSON.stringify(object.value)
      }`,
    );
  }
}
