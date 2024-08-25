function uuidv4() {
  return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, function (c) {
    var r = (Math.random() * 16) | 0,
      v = c == "x" ? r : (r & 0x3) | 0x8;
    return v.toString(16);
  });
}

class CloudstateObjectReference {
  constructor(objectId, constructorName) {
    this.objectId = objectId;
    this.constructorName = constructorName;
  }
}

class CloudstateMapReference {
  constructor(objectId) {
    this.objectId = objectId;
  }
}

class CloudstateArrayReference {
  constructor(objectId) {
    this.objectId = objectId;
  }
}

globalThis.CloudstateMapReference = CloudstateMapReference;
globalThis.CloudstateObjectReference = CloudstateObjectReference;
globalThis.CloudstateArrayReference = CloudstateArrayReference;

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

globalThis.Cloudstate = class Cloudstate {
  /**
   * Maps references of objects to their ids, this works because object equality is based on reference
   ** Keys: objects themselves
   ** Values: object ids - strings
   */
  constructor(namespace, options) {
    this.namespace = namespace;
    this.customClasses = options?.customClasses || [];
  }

  createTransaction() {
    const id = uuidv4();
    Deno.core.ops.op_create_transaction(id, this.namespace);
    return new CloudstateTransaction(this.namespace, id, this.customClasses);
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
  arrayChanges = new Map();
  arrays = new Map();
  customClasses = [];

  constructor(namespace, transactionId, customClasses) {
    if (typeof namespace !== "string") {
      throw new Error("namespace must be a string");
    }

    this.namespace = namespace;
    this.transactionId = transactionId;
    this.customClasses = customClasses;
  }

  hydrate(object, key, value) {
    if (value instanceof CloudstateObjectReference) {
      Object.defineProperty(object, key, {
        get: () => {
          const object = this.getObject(value.objectId);

          if (value.constructorName) {
            const constructor = this.customClasses.find(
              (klass) => klass.name === value.constructorName
            );

            if (!constructor) {
              throw new Error(
                `Custom class ${value.constructorName} not found`
              );
            }

            Object.setPrototypeOf(object, constructor.prototype);
          }

          return object;
        },
        set: (v) => {
          Object.defineProperty(object, key, {
            value: v,
          });
        },
      });
    }

    if (value instanceof CloudstateArrayReference) {
      Object.defineProperty(object, key, {
        value: this.getArray(value.objectId),
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

        const object = Deno.core.ops.op_cloudstate_map_get(
          this.transactionId,
          this.namespace,
          value.objectId,
          key
        );
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

  commit() {
    console.log("Committing objects");
    // console.log(Array.from(this.objects.keys()));
    for (const value of this.objects.values()) {
      this.#setObject(value);
    }
    // console.log("Committing done");
    Deno.core.ops.op_commit_transaction(this.transactionId);
  }

  getObject(id) {
    if (typeof id !== "string") throw new Error("id must be a string");

    const existingObject = this.objects.get(id);
    if (existingObject) return existingObject;

    const object = Deno.core.ops.op_cloudstate_object_get(
      this.transactionId,
      this.namespace,
      id
    );

    if (object.__cloudstate__constructorName) {
      Object.setPrototypeOf(
        object,
        this.customClasses.find(
          (klass) => klass.name === object.__cloudstate__constructorName
        ).prototype
      );
      delete object.__cloudstate__constructorName;
    }

    if (!object) return undefined;

    for (const [key, value] of Object.entries(object)) {
      this.hydrate(object, key, value);
    }

    this.objectIds.set(object, id);
    this.objects.set(id, object);

    return object;
  }

  #setObject(object) {
    if (typeof object !== "object") throw new Error("object must be an object");

    const stack = [object];
    const visited = new Set();

    let rootObject = undefined;
    while (stack.length > 0) {
      const object = stack.pop();

      if (object instanceof Map) {
        console.log("object is a map");
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
      Object.setPrototypeOf(flatObject, object.constructor.prototype);

      console.log("flattening object");
      console.log(object);
      Object.entries(object);
      console.log("Object.entries done");
      for (let [key, value] of Object.entries(object)) {
        console.log("flattening object", key);
        if (isArray) key = parseInt(key);
        if (value === undefined) continue;

        if (isPrimitive(value)) {
          flatObject[key] = value;
        } else if (typeof value === "object") {
          let id = this.objectIds.get(value);
          if (!id) {
            id = uuidv4();
            this.objectIds.set(value, id);
          }

          if (value instanceof Map) {
            flatObject[key] = new CloudstateMapReference(id);
          } else if (value instanceof Array) {
            flatObject[key] = new CloudstateArrayReference(id);
          } else {
            flatObject[key] = new CloudstateObjectReference(
              id,
              [Object, Array, Map].includes(value.constructor)
                ? undefined
                : value.constructor.name
            );
          }

          if (!visited.has(value)) {
            visited.add(value);
            stack.push(value);
          }
        } else {
          throw new Error(`${typeof value} cannot be serialized`);
        }
      }
      console.log("flattening object done");

      if (flatObject instanceof Array) {
        flatObject.forEach((item, i) => {
          Deno.core.ops.op_cloudstate_array_set(
            this.transactionId,
            this.namespace,
            this.objectIds.get(object),
            i,
            item
          );
        });
      } else {
        const id = this.#exportObject(object, flatObject);
        if (!rootObject) {
          rootObject = id;
        }
      }
    }

    return rootObject;
  }

  getArray(id) {
    if (typeof id !== "string") throw new Error("id must be a string");

    const existingArray = this.arrays.get(id);
    if (existingArray) return existingArray;

    const array = new Proxy(
      {},
      {
        get: (_target, key) => {
          let index = parseInt(key);
          if (isNaN(index)) {
            switch (key) {
              case "length": {
                return Deno.core.ops.op_cloudstate_array_length(
                  this.transactionId,
                  id
                );
              }
              case "constructor": {
                return Array;
              }
              case "push": {
                return (...args) => {
                  let length = Deno.core.ops.op_cloudstate_array_length(
                    this.transactionId,
                    id
                  );

                  for (const arg of args) {
                    const value = isPrimitive(arg)
                      ? arg
                      : new CloudstateObjectReference(this.#setObject(arg));

                    Deno.core.ops.op_cloudstate_array_set(
                      this.transactionId,
                      this.namespace,
                      id,
                      length,
                      // todo: support nested arrays
                      value
                    );
                    length++;
                  }
                  return length;
                };
              }
              case "toJSON": {
                return () => {
                  const length = Deno.core.ops.op_cloudstate_array_length(
                    this.transactionId,
                    id
                  );
                  const result = [];
                  for (let i = 0; i < length; i++) {
                    result.push(array[i]);
                  }
                  return result;
                };
              }
              case "map": {
                return (fn) => {
                  let arr = [];
                  for (let i = 0; i < array.length; i++) {
                    arr.push(fn(array[i], i));
                  }
                  return arr;
                };
              }
              default: {
                throw new Error(`Array.${key} is not supported`);
              }
            }
          }

          const result = Deno.core.ops.op_cloudstate_array_get(
            this.transactionId,
            this.namespace,
            id,
            index
          );

          if (result instanceof CloudstateObjectReference) {
            return this.getObject(result.objectId);
          } else {
            return result;
          }
        },
        set: (_target, key, value) => {
          if (typeof key !== "number") return;

          Deno.core.ops.op_cloudstate_array_set(
            this.transactionId,
            this.namespace,
            id,
            key,
            // todo: support nested arrays
            isPrimitive(value)
              ? value
              : new CloudstateObjectReference(this.#setObject(value))
          );
        },
      }
    );

    Object.setPrototypeOf(array, Array.prototype);

    this.arrays.set(id, array);
    this.objectIds.set(array, id);

    return array;
  }

  #exportObject(object, data) {
    console.log("exporting object");
    const existingId = this.objectIds.get(object);
    if (!existingId) {
      const id = uuidv4();
      Deno.core.ops.op_cloudstate_object_set(
        this.transactionId,
        this.namespace,
        id,
        data
      );
      this.objectIds.set(object, id);
      this.objects.set(id, object);
      return id;
    } else {
      console.log("existingId", existingId);
      Deno.core.ops.op_cloudstate_object_set(
        this.transactionId,
        this.namespace,
        existingId,
        data
      );
      return existingId;
    }
  }

  setRoot(alias, object) {
    if (typeof alias !== "string") throw new Error("alias must be a string");
    if (typeof object !== "object") throw new Error("object must be an object");

    const id = this.#setObject(object);

    Deno.core.ops.op_cloudstate_object_root_set(
      this.transactionId,
      this.namespace,
      alias,
      id
    );
  }

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