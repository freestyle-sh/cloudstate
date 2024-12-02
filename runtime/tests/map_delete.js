{
  const base = new Map([
    ["a", "alpha"],
    ["b", "beta"],
    ["c", "charlie"],
    ["d", "delta"],
    ["e", "echo"],
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
    ["a", "alpha"],
    ["b", "beta"],
    ["c", "charlie"],
    ["d", "delta"],
    ["e", "echo"],
  ]);

  const object = getRoot("test-root");
  if (!object) {
    throw new Error("object should exist");
  }
  if (!object.value) {
    throw new Error("object.value should exist");
  }
  if (!(object.value instanceof Map)) {
    throw new Error("object.value should be a Map");
  }
  if (object.value.size !== expected.size) {
    throw new Error(
      `object.value should have size ${expected.size}, got ${object.value.size}`,
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
        `object.value.get(${
          JSON.stringify(
            expectedKey,
          )
        }) should be ${JSON.stringify(expectedVal)}, got ${
          JSON.stringify(
            object.value.get(expectedKey),
          )
        }`,
      );
    }
  }

  const deletedVal = object.value.delete("a");
  if (!deletedVal) {
    throw new Error(
      `Expected ${
        JSON.stringify(
          "a",
        )
      } to be deleted (return value should be truthy)`,
    );
  }
  commit();
}

// END_FILE

{
  const expected = new Map([
    ["b", "beta"],
    ["c", "charlie"],
    ["d", "delta"],
    ["e", "echo"],
  ]);

  const object = getRoot("test-root");
  if (!object) {
    throw new Error("object should exist");
  }
  if (!object.value) {
    throw new Error("object.value should exist");
  }
  if (!(object.value instanceof Map)) {
    throw new Error("object.value should be a Map");
  }
  if (object.value.size !== expected.size) {
    throw new Error(
      `object.value should have size ${expected.size}, got ${object.value.size}`,
    );
  }
  if (object.value.has("a")) {
    throw new Error(`object.value should not have key 'a'`);
  }

  // delete key that doesn't exist
  const deletedVal = object.value.delete("z");
  if (deletedVal) {
    throw new Error(`Expected falsy because key 'z' does not exist`);
  }

  commit();
}
