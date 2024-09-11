{
  class Counter {
    count = 0;
    increment() {
      this.count += 1;
    }
    decrement() {
      this.count -= 1;
    }
  }

  registerCustomClass(Counter);

  const object = {
    counter: new Counter(),
  };

  setRoot("test-root", object);
  commit();
}

// END_FILE

{
  class Counter {
    count = 0;
    increment() {
      this.count += 1;
    }
    decrement() {
      this.count -= 1;
    }
  }

  registerCustomClass(Counter);

  const object = getRoot("test-root");
  if (!object) {
    throw new Error("object should exist");
  }
  if (object.counter.count !== 0) {
    throw new Error("object.counter.count should be 0");
  }

  object.counter.increment();
  if (!(object.counter instanceof Counter)) {
    throw new Error("object.counter should be an instance of Counter");
  }
  if (object.counter.count !== 1) {
    throw new Error("object.counter.count should be 1");
  }

  commit();
}

// END_FILE

{
  class Counter {
    count = 0;
    increment() {
      this.count += 1;
    }
    decrement() {
      this.count -= 1;
    }
  }

  registerCustomClass(Counter);

  const object = getRoot("test-root");
  if (!object) {
    throw new Error("object should exist");
  }
  if (object.counter.count !== 1) {
    throw new Error("object.counter.count should be 1");
  }

  commit();
}
