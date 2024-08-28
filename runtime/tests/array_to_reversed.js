const base_array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
const reversed_array = base_array.toReversed();

{
  const object = {
    value: base_array,
  };

  setRoot("test-root", object);
  commit();
}

{
  const object = getRoot("test-root");

  //verify array is set correctly
  if (object.value.length !== 10) {
    throw new Error(`Expected length to be 10, got ${object.value.length}`);
  }

  for (let i = 0; i < object.value.length; i++) {
    if (object.value[i] !== base_array[i]) {
      throw new Error(
        `Expected ${JSON.stringify(base_array[i])}, got ${JSON.stringify(
          object.value[i]
        )}`
      );
    }
  }

  const reversed_new_array = object.value.toReversed();

  if (reversed_new_array.length !== 10) {
    throw new Error(
      `Expected length to be 10, got ${reversed_new_array.length}`
    );
  }

  for (let i = 0; i < reversed_new_array.length; i++) {
    if (reversed_new_array[i] !== reversed_array[i]) {
      throw new Error(
        `Expected ${JSON.stringify(reversed_array[i])}, got ${JSON.stringify(
          reversed_new_array[i]
        )}`
      );
    }
  }
}
