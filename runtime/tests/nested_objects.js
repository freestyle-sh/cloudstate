{
  const object = {
    counter: {
      count: 0,
    },
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
  if (object.counter.count !== 0) {
    throw new Error("object.counter.count should be 0");
  }

  object.counter.count += 1;

  commit();
}

// END_FILE

{
  const object = getRoot("test-root");
  if (!object) {
    throw new Error("object should exist");
  }
  if (object.counter.count !== 1) {
    throw new Error("object.counter.count should be 1");
  }

  commit();
}
