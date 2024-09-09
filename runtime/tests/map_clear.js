{
  const base = new Map([
    ["foo", 1],
    ["bar", 2],
    ["baz", 3],
  ]);
  const object = {
    value: base,
  };

  setRoot("test-root", object);
  commit();
}

// END_FILE

{
  const expected = new Map([
    ["foo", 1],
    ["bar", 2],
    ["baz", 3],
  ]);
  const object = getRoot("test-root");

  // verify map is set correctly
  if (object.value.size !== expected.size) {
    throw new Error(
      `Expected size to be ${expected.size}, got ${object.value.size}`,
    );
  }
  for (const [expectedKey, expectedVal] of expected.entries()) {
    if (!object.value.has(expectedKey)) {
      throw new Error(
        `object.value should have key ${JSON.stringify(expectedKey)}`,
      );
    }
    if (object.value.get(expectedKey) !== expectedVal) {
      throw new Error(
        `object.value.get(${JSON.stringify(expectedKey)}) should be ${
          JSON.stringify(
            expectedVal,
          )
        }, got ${JSON.stringify(object.value.get(expectedKey))}`,
      );
    }
  }

  // clear map
  object.value.clear();

  // verify map is cleared
  if (object.value.size !== 0) {
    throw new Error(`Expected size to be 0, got ${object.value.size}`);
  }
  for (const clearedKey of expected.keys()) {
    if (object.value.has(clearedKey)) {
      throw new Error(
        `object.value should not have key ${JSON.stringify(clearedKey)}`,
      );
    }
  }

  commit();
}

// END_FILE

{
  const object = getRoot("test-root");

  if (object.value.size !== 0) {
    throw new Error(`Expected size to be 0, got ${object.value.size}`);
  }
  for (const clearedKey of object.value.keys()) {
    throw new Error(
      `object.value should not have key ${JSON.stringify(clearedKey)}`,
    );
  }

  commit();
}
