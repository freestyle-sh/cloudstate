{
  const object = {
    count: 1,
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
  if (object.count !== 1) {
    throw new Error("object.count should be 1");
  }
}
