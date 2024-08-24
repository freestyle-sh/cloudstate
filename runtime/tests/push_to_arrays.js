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

  transaction.commit();
}
