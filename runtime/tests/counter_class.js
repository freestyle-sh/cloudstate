{
  class Counter {
    constructor() {
      this.count = 0;
    }
    increment() {
      this.count++;
    }
    getCount() {
      return this.count;
    }
  }

  registerCustomClass(Counter);

  const root = {
    value: new Counter(),
  };

  setRoot("test-root", root);
  commit();
}

// END_FILE

{
  class Counter {
    constructor() {
      this.count = 0;
    }
    increment() {
      this.count++;
    }
    getCount() {
      return this.count;
    }
  }

  registerCustomClass(Counter);

  const root = getRoot("test-root");
  root.value.increment();
  if (root.value.getCount() !== 1) {
    throw new Error(`Expected count to be 1, got ${root.value.getCount()}`);
  }

  root.value.increment();
  if (root.value.getCount() !== 2) {
    throw new Error(`Expected count to be 2, got ${root.value.getCount()}`);
  }

  commit();
}
