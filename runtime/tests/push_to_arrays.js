{
  const cloudstate = new Cloudstate("test-namespace");

  const transaction = cloudstate.createTransaction();

  const object = transaction.getRoot("objects-test-root") || {
    counters: [],
  };

  transaction.setObject(object);
  transaction.setRoot("objects-test-root", object);
  transaction.commit();
}

{
  const cloudstate = new Cloudstate("test-namespace");

  const transaction = cloudstate.createTransaction();

  const object = transaction.getRoot("objects-test-root");

  object.counters.push({
    count: 0,
  });

  transaction.commit();
}

{
  const cloudstate = new Cloudstate("test-namespace");

  const transaction = cloudstate.createTransaction();

  const object = transaction.getRoot("objects-test-root");

  if (!object) throw new Error("object should exist");
  if (object.counters.length !== 1) {
    throw new Error("object.counters should have length 1");
  }

  if (object.counters[0].count !== 0) {
    throw new Error("object.counters[0].count should be 0");
  }

  object.counters[0].count += 1;

  if (object.counters[0].count !== 1) {
    throw new Error("object.counters[0].count should be 1");
  }

  console.log("existing id should be here");
  transaction.setObject(object);

  transaction.commit();
}

{
  const cloudstate = new Cloudstate("test-namespace");

  const transaction = cloudstate.createTransaction();

  const object = transaction.getRoot("objects-test-root");

  if (!object) throw new Error("object should exist");
  if (object.counters.length !== 1) {
    throw new Error(
      "object.counters should have length 1 but has length " +
        object.counters.length
    );
  }

  if (object.counters[0].count !== 1) {
    throw new Error("object.counter[0].count should be 1");
  }

  transaction.commit();
}
