const cloudstate = new Cloudstate("test-namespace");

const object = cloudstate.getRoot("test-root") || {
  counters: [
    {
      count: 0,
    },
  ],
};

object.counters[0].count += 1;
console.log(object);

cloudstate.setObject(object);
cloudstate.setRoot("test-root", object);
