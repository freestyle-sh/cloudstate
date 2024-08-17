const cloudstate = new Cloudstate("test-namespace");

const object = cloudstate.getRoot("test-root") || { count: 0 };

object.count++;

cloudstate.setObject(object);
cloudstate.setRoot("test-root", object);
