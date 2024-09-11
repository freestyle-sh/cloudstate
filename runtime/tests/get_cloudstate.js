{
  class CounterCS {
    id = "counter";
    count = 0;

    increment() {
      this.count += 1;
    }
  }

  registerCustomClass(CounterCS);

  const object = {
    counter: new CounterCS(),
  };

  setRoot("test-root", object);
  commit();
}

// END_FILE

{
  class CounterCS {
    id = "counter";
    count = 0;

    increment() {
      this.count += 1;
    }
  }

  registerCustomClass(CounterCS);

  const counter = getCloudstate("counter");
  if (!counter) {
    throw new Error("counter should exist");
  }
  if (counter.count !== 0) {
    throw new Error("counter.count should be 0");
  }
  if (counter instanceof CounterCS === false) {
    throw new Error("counter should be an instance of CounterCS");
  }
}
