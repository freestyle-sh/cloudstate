{
  const common = {
    value: 1,
  };
  const object = {
    a: common,
    b: common,
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
  if (!object.a) {
    throw new Error("object.a should exist");
  }
  if (!object.b) {
    throw new Error("object.b should exist");
  }
  if (object.a !== object.b) {
    throw new Error("object.a should be the same as object.b");
  }
  if (object.a.value !== 1) {
    throw new Error("object.a.value should be 1");
  }
  if (object.b.value !== 1) {
    throw new Error("object.b.value should be 1");
  }
}
