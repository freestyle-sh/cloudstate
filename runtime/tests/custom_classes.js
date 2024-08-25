class Counter {
  count = 0;

  increment() {
    this.count += 1;
  }

  decrement() {
    this.count -= 1;
  }
}

{
  const cloudstate = new Cloudstate("test-namespace", {
    customClasses: [Counter],
  });

  const transaction = cloudstate.createTransaction();

  const object = {
    counter: new Counter(),
  };

  transaction.setRoot("test-root", object);
  transaction.commit();
}

{
  const cloudstate = new Cloudstate("test-namespace", {
    customClasses: [Counter],
  });

  const transaction = cloudstate.createTransaction();

  const object = transaction.getRoot("test-root");

  if (!object) throw new Error("object should exist");

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

  transaction.commit();
}

{
  const cloudstate = new Cloudstate("test-namespace", {
    customClasses: [Counter],
  });

  const transaction = cloudstate.createTransaction();

  const object = transaction.getRoot("test-root");

  if (!object) throw new Error("object should exist");

  if (object.counter.count !== 1) {
    throw new Error("object.counter.count should be 1");
  }

  transaction.commit();
}
