const cloudstate = new Cloudstate("test-namespace");

const transaction = cloudstate.createTransaction();

const object = transaction.getRoot("maps-test-root") || {
  counters: new Map([["a", 0]]),
};

const count = object.counters.get("a");
object.counters.set("a", count + 1);

console.log(object.counters.get("a"));

transaction.setObject(object);
transaction.setRoot("test-root", object);
transaction.commit();
