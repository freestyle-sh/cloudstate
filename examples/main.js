const cloudstate = new Cloudstate("test-namespace");

const object = cloudstate.getRoot("test-root") || {
  counter: {
    count: 0,
  },
};

console.log(object.counter.count);
object.counter.count += 1;

cloudstate.setObject(object);
cloudstate.setRoot("test-root", object);
