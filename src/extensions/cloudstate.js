function uuidv4() {
  return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, function (c) {
    var r = (Math.random() * 16) | 0,
      v = c == "x" ? r : (r & 0x3) | 0x8;
    return v.toString(16);
  });
}

class CloudstateReference {
  constructor(objectId) {
    this.objectId = objectId;
  }
}

SuperJSON.registerCustom(
  {
    isApplicable: (v) => v instanceof CloudstateReference,
    serialize: (v) => v.objectId,
    deserialize: (v) => new CloudstateReference(v),
  },
  "cloudstate-reference"
);

globalThis.Cloudstate = class Cloudstate {
  objectIds = new Map();
  objects = new Map();

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

    const data = Deno.core.ops.op_cloudstate_object_get(this.namespace, id);
    if (!data) return undefined;

    const object = SuperJSON.parse(data);

    for (const [key, value] of Object.entries(object)) {
      if (value instanceof CloudstateReference) {
        Object.defineProperty(object, key, {
          get: () => {
            return this.getObject(value.objectId);
          },
          set: (v) => {
            Object.defineProperty(object, key, {
              value: v,
            });
          },
        });
      }
    }

    this.objectIds.set(object, id);
    this.objects.set(id, object);

    return object;
  }

  setObject(object) {
    if (typeof object !== "object") throw new Error("object must be an object");

    const stack = [object];
    const visited = new Set();

    while (stack.length > 0) {
      const object = stack.pop();

      const flatObject = object instanceof Array ? [] : {};
      for (const [key, value] of Object.entries(object)) {
        if (!value) continue;

        if (
          value === null ||
          typeof value === "number" ||
          typeof value === "string" ||
          typeof value === "boolean" ||
          typeof value === "bigint" ||
          typeof value === "undefined" ||
          value?.constructor === Date ||
          value?.constructor === RegExp ||
          value?.constructor === URL ||
          value?.constructor === Error
        ) {
          flatObject[key] = value;
        } else if (typeof value === "object") {
          if (![Object, Array].includes(value.constructor)) {
            throw new Error(`${value.constructor.name} cannot be serialized`);
          }

          let id = this.objectIds.get(value);
          if (!id) {
            id = uuidv4();
            this.objectIds.set(value, id);
          }

          flatObject[key] = new CloudstateReference(id);

          Object.defineProperty(object, key, {
            get: () => this.getObject(id),
            set: (v) => {
              Object.defineProperty(object, key, {
                value: v,
              });
            },
          });

          if (!visited.has(value)) {
            visited.add(value);
            stack.push(value);
          }
        } else {
          throw new Error(`${typeof value} cannot be serialized`);
        }
      }

      this.#exportObject(object, flatObject);
    }
  }

  #exportObject(object, data) {
    const existingId = this.objectIds.get(object);
    if (!existingId) {
      const id = uuidv4();
      Deno.core.ops.op_cloudstate_object_set(
        this.namespace,
        id,
        SuperJSON.stringify(data)
      );
      this.objectIds.set(object, id);
      this.objects.set(id, object);
      return id;
    } else {
      Deno.core.ops.op_cloudstate_object_set(
        this.namespace,
        existingId,
        SuperJSON.stringify(data)
      );
      return existingId;
    }
  }

  setRoot(alias, object) {
    if (typeof alias !== "string") throw new Error("alias must be a string");
    if (typeof object !== "object") throw new Error("object must be an object");

    const existingId = this.objectIds.get(object);
    if (!existingId) {
      throw new Error("object is not registered");
    }

    Deno.core.ops.op_cloudstate_object_root_set(
      this.namespace,
      alias,
      existingId
    );
  }

  // // alternative behavior for setRoot
  // setRoot(alias, object) {
  //   const id = this.setObject(object);
  //   Deno.core.ops.op_cloudstate_object_root_set(this.namespace, alias, id);
  //   return id;
  // }

  getRoot(alias) {
    if (typeof alias !== "string") throw new Error("alias must be a string");

    const id = Deno.core.ops.op_cloudstate_object_root_get(
      this.namespace,
      alias
    );

    if (!id) return undefined;

    return this.getObject(id);
  }
};
