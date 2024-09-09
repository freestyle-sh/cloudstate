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
  const object = getRoot("test-root");
  object.value.set("f", "foxtrot");
  object.value.set("num", 42);

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
    ["f", "foxtrot"],
    ["num", 42],
  ]);
  const object = getRoot("test-root");
  const values = object.value.values();
  const arr = Array.from(values);

  console.log("arr:", arr);
  console.log("expected:", expected);
  const expectedArr = Array.from(expected.values());
  console.log("expectedArr:", expectedArr);

  if (arr.length !== expectedArr.length) {
    throw new Error(
      `Should have been arr.length = ${expectedArr.length}, got ${arr.length}`,
    );
  }
  for (let i = 0; i < arr.length; i++) {
    const value = arr[i];
    const expectedValue = expectedArr[i];
    if (value !== expectedValue) {
      throw new Error(
        `Should have been arr[${i}] = ${expectedValue}, got ${value}`,
      );
    }
  }
}
