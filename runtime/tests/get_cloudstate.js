class CounterCS {
  id = "counter";
  count = 0;

  increment() {
    this.count += 1;
  }
}

{
  globalThis.cloudstate.customClasses = [CounterCS];

  const object = {
    counter: new CounterCS(),
  };

  setRoot("test-root", object);
}

commit();

{
  const counter = getCloudstate("counter");
  console.log("counter", counter);
  if (!counter) throw new Error("counter should exist");
  if (counter.count !== 0) throw new Error("counter.count should be 0");
  if (counter instanceof CounterCS === false) {
    throw new Error("counter should be an instance of CounterCS");
  }
}
