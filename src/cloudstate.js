function uuidv4() {
  return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, function (c) {
    var r = (Math.random() * 16) | 0,
      v = c == "x" ? r : (r & 0x3) | 0x8;
    return v.toString(16);
  });
}

globalThis.Cloudstate = class Cloudstate {
  objectIds = new Map();
  objects = new Map();

  // todo: remove
  roots = new Map();

  constructor(namespace) {
    if (typeof namespace !== "string") {
      throw new Error("namespace must be a string");
    }

    this.namespace = namespace;
  }

  getObject(id) {
    if (typeof id !== "string") throw new Error("id must be a string");

    const existingObject = this.objects.get(id);
    if (existingObject) return existingObject;

    const data = Deno.core.ops.op_cloudstate_object_get(namespace, id);
    if (!data) throw new Error("Object not found");

    const object = Deno.core.deserialize(data);

    this.objectIds.set(object, id);
    this.objects.set(id, object);

    return object;
  }

  setObject(object) {
    // if (typeof id !== "string") throw new Error("id must be a string");
    if (typeof object !== "object") throw new Error("object must be an object");

    const existingId = this.objectIds.get(object);

    if (!existingId) {
      const id = uuidv4();
      Deno.core.ops.op_cloudstate_object_set(
        this.namespace,
        id,
        Deno.core.serialize(object)
      );
      this.objectIds.set(object, id);
      this.objects.set(id, object);
    } else {
      Deno.core.ops.op_cloudstate_object_set(
        this.namespace,
        existingId,
        object
      );
    }
  }

  setRoot(object, alias) {
    if (typeof object !== "object") throw new Error("object must be an object");
    if (typeof alias !== "string") throw new Error("alias must be a string");

    const existingId = this.objectIds.get(object);
    if (!existingId) {
      throw new Error("object is not registered");
    }

    // todo: remove
    this.roots.set(alias, this.objectIds.get(object));
  }

  getRoot(alias) {
    if (typeof alias !== "string") throw new Error("alias must be a string");

    const id = this.roots.get(alias);
    if (!id) throw new Error("alias not found");

    return this.getObject(id);
  }
};
