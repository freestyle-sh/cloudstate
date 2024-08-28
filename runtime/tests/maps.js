{
  const object = {
    counters: new Map([["a", 0]]),
  };

  const count = object.counters.get("a");
  object.counters.set("a", count + 1);

  setRoot("test-root", object);
  commit();
}

{
  const object = getRoot("test-root");
  if (!object) {
    throw new Error("object should exist");
  }
  if (object.counters.get("a") !== 1) {
    throw new Error("object.counters.get('a') should be 1");
  }
}
