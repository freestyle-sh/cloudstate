{
  const cloudstate = new Cloudstate("test-namespace");

  const transaction = cloudstate.createTransaction();

  const object = transaction.getRoot("objects-test-root") || {
    counters: [
      {
        count: 0,
      },
    ],
  };

  object.counters[0].count += 1;

  transaction.setObject(object);
  transaction.setRoot("objects-test-root", object);
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

  if (object.counters[0].count !== 1) {
    throw new Error("object.counters[0].count should be 1");
  }

  transaction.commit();
}
