{
  class Counter {
    count = 0;
    increment() {
      this.count++;
    }
    getCount() {
      return this.count;
    }
  }
  class CounterManager {
    counters = [];

    createCounter() {
      const counter = new Counter();
      this.counters.push(counter);
      return counter;
    }
    getCounters() {
      return this.counters;
    }
    getCounter(index) {
      return this.counters[index];
    }
  }

  registerCustomClass(Counter);
  registerCustomClass(CounterManager);

  const root = {
    value: new CounterManager(),
  };

  setRoot("test-root", root);
  commit();
}

// END_FILE

{
  class Counter {
    count = 0;
    increment() {
      this.count++;
    }
    getCount() {
      return this.count;
    }
  }
  class CounterManager {
    counters = [];

    createCounter() {
      const counter = new Counter();
      this.counters.push(counter);
      return counter;
    }
    getCounters() {
      return this.counters;
    }
    getCounter(index) {
      return this.counters[index];
    }
  }

  registerCustomClass(Counter);
  registerCustomClass(CounterManager);

  const root = getRoot("test-root");
  const counter = root.value.createCounter();

  counter.increment();
  if (counter.getCount() !== 1) {
    throw new Error(`Expected count to be 1, got ${counter.getCount()}`);
  }

  commit();
}

// END_FILE

{
  class Counter {
    count = 0;
    increment() {
      this.count++;
    }
    getCount() {
      return this.count;
    }
  }
  class CounterManager {
    counters = [];

    createCounter() {
      const counter = new Counter();
      this.counters.push(counter);
      return counter;
    }
    getCounters() {
      return this.counters;
    }
    getCounter(index) {
      return this.counters[index];
    }
  }

  registerCustomClass(Counter);
  registerCustomClass(CounterManager);

  const root = getRoot("test-root");
  const counter = root.value.getCounter(0);

  counter.increment();
  if (counter.getCount() !== 2) {
    throw new Error(`Expected count to be 2, got ${counter.getCount()}`);
  }
}
