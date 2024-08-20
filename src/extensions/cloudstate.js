function uuidv4() {
  return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, function (c) {
    var r = (Math.random() * 16) | 0,
      v = c == "x" ? r : (r & 0x3) | 0x8;
    return v.toString(16);
  });
}

class CloudstateObjectReference {
  constructor(objectId) {
    this.objectId = objectId;
  }
}

class CloudstateMapReference {
  constructor(objectId) {
    this.objectId = objectId;
  }
}

function isPrimitive(value) {
  return (
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
  );
}

SuperJSON.registerCustom(
  {
    isApplicable: (v) => v instanceof CloudstateObjectReference,
    serialize: (v) => v.objectId,
    deserialize: (v) => new CloudstateObjectReference(v),
  },
  "cloudstate-object-reference"
);

SuperJSON.registerCustom(
  {
    isApplicable: (v) => v instanceof CloudstateMapReference,
    serialize: (v) => v.objectId,
    deserialize: (v) => new CloudstateMapReference(v),
  },
  "cloudstate-map-reference"
);

globalThis.Cloudstate = class Cloudstate {
  /**
   * Maps references of objects to their ids, this works because object equality is based on reference
   ** Keys: objects themselves
   ** Values: object ids - strings
   */
  constructor(namespace) {
    this.namespace = namespace;
  }

  createTransaction() {
    const id = uuidv4();
    Deno.core.ops.op_create_transaction(id, this.namespace);
    return new CloudstateTransaction(this.namespace, id);
  }

  getTestsObject() {
    return Deno.core.ops.op_cloudstate_get_test_object();
  }
};

class CloudstateTransaction {
  objectIds = new Map();
  /**
   * Maps object ids to their objects
   ** Keys: object ids - strings
   ** Values: objects themselves
   */
  objects = new Map();
  mapChanges = new Map();

  constructor(namespace, transactionId) {
    if (typeof namespace !== "string") {
      throw new Error("namespace must be a string");
    }

    this.namespace = namespace;
    this.transactionId = transactionId;
  }

  commit() {
    Deno.core.ops.op_commit_transaction(this.transactionId);
  }

  getObject(id) {
    if (typeof id !== "string") throw new Error("id must be a string");

    const existingObject = this.objects.get(id);
    if (existingObject) return existingObject;

    const data = Deno.core.ops.op_cloudstate_object_get(
      this.transactionId,
      this.namespace,
      id
    );
    if (!data) return undefined;

    const object = SuperJSON.parse(data);

    for (const [key, value] of Object.entries(object)) {
      if (value instanceof CloudstateObjectReference) {
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

      if (value instanceof CloudstateMapReference) {
        const map = new Map();
        const mapSet = map.set;
        const mapGet = map.get;
        const changeMap = new Map();
        this.mapChanges.set(map, changeMap);
        this.objectIds.set(map, value.objectId);

        map.get = (key) => {
          const result = mapGet.apply(map, [key]);
          if (result) return result;

          const data = Deno.core.ops.op_cloudstate_map_get(
            this.transactionId,
            this.namespace,
            value.objectId,
            key
          );
          const object = SuperJSON.parse(data);
          mapSet.apply(map, [key, object]);
          return object;
        };
        map.set = (key, value) => {
          mapSet.apply(map, [key, value]);
          changeMap.set(key, value);
        };

        Object.defineProperty(object, key, {
          get: () => {
            return map;
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

      if (object instanceof Map) {
        const changes = this.mapChanges.get(object) || object;
        for (const [key, value] of changes.entries()) {
          if (isPrimitive(value)) {
            Deno.core.ops.op_cloudstate_map_set(
              this.transactionId,
              this.namespace,
              this.objectIds.get(object),
              key,
              value
            );
          }
        }
        continue;
      }

      const isArray = object instanceof Array;
      const flatObject = isArray ? [] : {};
      for (let [key, value] of Object.entries(object)) {
        if (isArray) key = parseInt(key);
        if (value === undefined) continue;

        if (isPrimitive(value)) {
          flatObject[key] = value;
        } else if (typeof value === "object") {
          if (![Object, Array, Map].includes(value.constructor)) {
            throw new Error(`${value.constructor.name} cannot be serialized`);
          }

          let id = this.objectIds.get(value);
          if (!id) {
            id = uuidv4();
            this.objectIds.set(value, id);
          }

          if (value instanceof Map) {
            flatObject[key] = new CloudstateMapReference(id);
          } else {
            flatObject[key] = new CloudstateObjectReference(id);
          }

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
        this.transactionId,
        this.namespace,
        id,
        SuperJSON.stringify(data)
      );
      this.objectIds.set(object, id);
      this.objects.set(id, object);
      return id;
    } else {
      Deno.core.ops.op_cloudstate_object_set(
        this.transactionId,
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
      this.transactionId,
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
      this.transactionId,
      this.namespace,
      alias
    );

    if (!id) return undefined;

    return this.getObject(id);
  }
}
