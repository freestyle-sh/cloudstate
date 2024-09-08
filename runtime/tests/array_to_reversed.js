{
  const base = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  const object = {
    value: base,
  };
  setRoot("test-root", object);
  commit();
}

// END_FILE

{
  const expectedArr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  const expectedReversedArr = expectedArr.toReversed();

  const object = getRoot("test-root");
  // verify array is set correctly
  if (object.value.length !== expectedArr.length) {
    throw new Error(
      `Expected length to be ${expectedArr.length}, got ${object.value.length}`,
    );
  }
  for (let i = 0; i < expectedArr.length; i++) {
    if (object.value[i] !== expectedArr[i]) {
      throw new Error(
        `Expected ${JSON.stringify(expectedArr[i])}, got ${
          JSON.stringify(
            object.value[i],
          )
        }`,
      );
    }
  }

  const reversedArr = object.value.toReversed();
  if (reversedArr.length !== expectedReversedArr.length) {
    throw new Error(
      `Expected length to be ${expectedReversedArr.length}, got ${reversedArr.length}`,
    );
  }
  for (let i = 0; i < expectedReversedArr.length; i++) {
    if (reversedArr[i] !== expectedReversedArr[i]) {
      throw new Error(
        `Expected ${JSON.stringify(expectedReversedArr[i])}, got ${
          JSON.stringify(
            reversedArr[i],
          )
        }`,
      );
    }
  }
}
