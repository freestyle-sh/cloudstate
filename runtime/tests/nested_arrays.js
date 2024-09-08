{
  const object = {
    value: [[1, 2, 3]],
  };

  setRoot("test-root", object);
  commit();
}

// END_FILE

{
  const object = getRoot("test-root");
  if (!object) {
    throw new Error("object should exist");
  }
  if (!object.value) {
    throw new Error("object.value should exist");
  }
  if (!object.value[0]) {
    throw new Error("object.value[0] should exist");
  }
  if (object.value[0].length !== 3) {
    throw new Error(
      `object.value[0].length should be 3, got ${
        JSON.stringify(
          object.value[0],
        )
      }`,
    );
  }
  if (object.value[0][0] !== 1) {
    throw new Error("object.value[0][0] should be 1");
  }
  if (object.value[0][1] !== 2) {
    throw new Error("object.value[0][1] should be 2");
  }
  if (object.value[0][2] !== 3) {
    throw new Error("object.value[0][2] should be 3");
  }
}
