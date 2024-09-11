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

  // Set root to a class directly
  setRoot("test-root", new Counter());
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

  // Check if root still exists
  const counter = getRoot("test-root");
  if (!counter) {
    throw new Error("class instance should exist");
  }
  if (counter.count !== 0) {
    throw new Error("counter.count should be 0");
  }

  // Change the data to test in memory changes
  counter.increment();
  if (!(counter instanceof Counter)) {
    throw new Error("counter should be an instance of Counter");
  }
  if (counter.count !== 1) {
    throw new Error("counter.count should be 1");
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

  // Check if root still exists
  const counter = getRoot("test-root");
  if (!counter) {
    throw new Error("class instance should exist");
  }
  if (counter.count !== 1) {
    throw new Error("counter.count should be 1");
  }
  commit();
}
