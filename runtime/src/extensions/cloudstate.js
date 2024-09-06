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

class CloudstateBlobReference {
  constructor(blobId) {
    this.blobId = blobId;
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
globalThis.CloudstateBlob = CloudstateBlobReference;

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
    console.log("Creating transaction");
    const id = uuidv4();
    Deno.core.ops.op_cloudstate_create_transaction(id, this.namespace);
    return new CloudstateTransaction(this.namespace, id, this.customClasses);
  }
};

/**
 * Maps object ids to their objects
 ** Keys: object ids - strings
 ** Values: objects themselves
 */
const objects = new Map();
const arrays = new Map();
const objectIds = new Map();
/** Maps the user defined id on classes to their objects */
const cloudstateObjects = new Map();

class CloudstateTransaction {
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
    console.log("hydrating some object");

    if (value instanceof CloudstateObjectReference) {
      Object.defineProperty(object, key, {
        get: () => {
          ensureTransaction();
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
          ensureTransaction();
          Object.defineProperty(object, key, {
            value: v,
          });
        },
      });
    }

    if (value instanceof CloudstateBlobReference) {
      const blob = new Blob();

      blob["text"] = async () => {
        ensureTransaction();
        return Deno.core.ops.op_cloudstate_blob_text(
          this.transactionId,
          value.blobId
        );
      };

      blob["arrayBuffer"] = async () => {
        ensureTransaction();
        return Deno.core.ops.op_cloudstate_blob_array_buffer(
          this.transactionId,
          value.blobId
        );
      };

      blob["bytes"] = async () => {
        ensureTransaction();
        return Deno.core.ops.op_cloudstate_blob_bytes(
          this.transactionId,
          value.blobId
        );
      };

      Object.defineProperty(blob, "size", {
        get: () => {
          ensureTransaction();
          return Deno.core.ops.op_cloudstate_blob_size(
            this.transactionId,
            value.blobId
          );
        },
      });

      Object.defineProperty(blob, "type", {
        get: () => {
          ensureTransaction();
          // TODO: MAKE TYPE
          return Deno.core.ops.op_cloudstate_blob_get_type(
            this.transactionId,
            value.blobId
          );
        },
      });

      // Object.defineProperty(object, key, {
      //   {
      //     value: blob,
      //   }

      // });
    }

    if (value instanceof CloudstateArrayReference) {
      Object.defineProperty(object, key, {
        value: this.getArray(value.objectId),
      });
    }

    if (value instanceof CloudstateMapReference) {
      Object.defineProperty(object, key, {
        value: this.getMap(value.objectId),
      });
    }
  }

  commit() {
    console.log("Committing transaction");
    for (const value of objects.values()) {
      this.#setObject(value);
    }
    Deno.core.ops.op_cloudstate_commit_transaction(this.transactionId);
  }

  getMap(objectId) {
    ensureTransaction();
    const map = new Map();
    const mapSet = map.set;
    const mapGet = map.get;

    objectIds.set(map, objectId);
    map["values"] = () => {
      ensureTransaction();
      let map_values = Deno.core.ops.op_cloudstate_map_values(
        this.transactionId,
        objectId
      );

      return map_values
        .map((value) => this.unpackFromReference(value))
        .values();
    };
    map["keys"] = () => {
      ensureTransaction();
      return Deno.core.ops
        .op_cloudstate_map_keys(this.transactionId, objectId)
        .values();
    };

    map["entries"] = () => {
      ensureTransaction();
      let entries = Deno.core.ops.op_cloudstate_map_entries(
        this.transactionId,
        objectId
      );

      return entries
        .map(([key, value]) => [key, this.unpackFromReference(value)])
        .values();
    };
    map["delete"] = (key) => {
      ensureTransaction();
      map.delete(key);
      return Deno.core.ops.op_cloudstate_map_delete(
        this.transactionId,
        this.namespace,
        objectId,
        key
      );
    };

    map["clear"] = () => {
      ensureTransaction();
      return Deno.core.ops.op_cloudstate_map_clear(
        this.transactionId,
        this.namespace,
        objectId
      );
    };

    map["forEach"] = (fn) => {
      ensureTransaction();
      const entries = map.entries();
      for (const entry of entries) {
        fn(entry[1], entry[0], map);
      }
    };

    map.has = (key) => {
      ensureTransaction();
      return Deno.core.ops.op_cloudstate_map_has(
        this.transactionId,
        this.namespace,
        objectId,
        key
      );
    };

    map.get = (key) => {
      ensureTransaction();
      const result = mapGet.apply(map, [key]);
      if (result) return result;

      const object = Deno.core.ops.op_cloudstate_map_get(
        this.transactionId,
        this.namespace,
        objectId,
        key
      );

      // TODO: is this invalid reference callback needed?
      return this.unpackFromReference(object, () => {
        mapSet.apply(map, [key, object]);
      });
    };
    map.set = (key, set_value) => {
      ensureTransaction();
      // todo: support nested arrays
      const val = isPrimitive(set_value)
        ? set_value
        : new CloudstateObjectReference(this.#setObject(set_value));

      Deno.core.ops.op_cloudstate_map_set(
        this.transactionId,
        this.namespace,
        objectId,
        key,
        val
      );

      mapSet.apply(map, [key, val]);
    };

    Object.defineProperty(map, "size", {
      get: () => {
        ensureTransaction();
        return Deno.core.ops.op_cloudstate_map_size(
          this.transactionId,
          objectId
        );
      },
    });

    console.log("GETTING MAP entries()", Array.from(map.entries()));
    console.log("GETTING MAP size", map.size);

    return map;
  }
  getObject(id) {
    ensureTransaction();
    if (typeof id !== "string") throw new Error("id must be a string");

    const existingObject = objects.get(id);
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
          (klass) =>
            klass.name === object.__cloudstate__constructorName.replace("_", "") // TODO: should fix underlying issue: CS classes are prefixed with underscore at ESM build
        ).prototype
      );
      delete object.__cloudstate__constructorName;
    }

    if (!object) return undefined;

    for (const [key, value] of Object.entries(object)) {
      this.hydrate(object, key, value);
    }

    objectIds.set(object, id);
    objects.set(id, object);

    if (object.id && this.customClasses.includes(object.constructor)) {
      cloudstateObjects.set(object.id, object);
    }

    return object;
  }
  packToReferenceOrPrimitive(value) {
    if (isPrimitive(value)) {
      return value;
    }
    if (value instanceof Object) {
      return new CloudstateObjectReference(this.#setObject(value));
    }
    if (value instanceof Array) {
      return new CloudstateArrayReference(this.#setObject(value));
    }
    if (value instanceof Map) {
      return new CloudstateMapReference(this.#setObject(value));
    }
    if (value instanceof Blob) {
      return new CloudstateBlobReference(this.#setObject(value));
    }
    if (
      value instanceof CloudstateObjectReference ||
      value instanceof CloudstateArrayReference ||
      value instanceof CloudstateMapReference ||
      value instanceof CloudstateBlobReference
    ) {
      console.error(
        `${value.objectId} is already a reference: packToReferenceOrPrimitive is unnecessary`
      );
      return value;
    }
    throw new Error(`${typeof value} cannot be serialized`);
  }
  unpackFromReference(reference, notValidCb) {
    if (reference instanceof CloudstateObjectReference) {
      return this.getObject(reference.objectId);
    }
    if (reference instanceof CloudstateArrayReference) {
      return this.getArray(reference.objectId);
    }
    if (reference instanceof CloudstateMapReference) {
      return this.getMap(reference.objectId);
    }
    // Object is not a valid Cloudstate reference
    if (notValidCb) {
      notValidCb.call();
    }
    return reference;
  }

  getCloudstate(id) {
    if (typeof id !== "string") throw new Error("id must be a string");
    if (cloudstateObjects.has(id)) {
      return cloudstateObjects.get(id);
    } else {
      const value = Deno.core.ops.op_cloudstate_cloudstate_get(
        this.transactionId,
        this.namespace,
        id
      );

      return this.getObject(value.objectId);
    }
  }

  #setObject(object) {
    if (typeof object !== "object") throw new Error("object must be an object");

    const stack = [object];
    const visited = new Set();

    let rootObject = undefined;
    while (stack.length > 0) {
      const object = stack.pop();

      if (object instanceof Map) {
        for (const [key, value] of object.entries()) {
          if (isPrimitive(value)) {
            Deno.core.ops.op_cloudstate_map_set(
              this.transactionId,
              this.namespace,
              objectIds.get(object),
              key,
              value
            );
          } else {
            const id = this.#setObject(value);
            Deno.core.ops.op_cloudstate_map_set(
              this.transactionId,
              this.namespace,
              objectIds.get(object),
              key,
              new CloudstateObjectReference(id)
            );
          }
        }

        continue;
      }

      const isArray = object instanceof Array;
      const flatObject = isArray ? [] : {};
      Object.setPrototypeOf(flatObject, object.constructor.prototype);

      console.log("flattening object");
      for (let [key, value] of Object.entries(object)) {
        // console.log("flattening object", key);
        if (isArray) key = parseInt(key);
        if (value === undefined) continue;

        if (isPrimitive(value)) {
          flatObject[key] = value;
        } else if (typeof value === "object") {
          let id = objectIds.get(value);
          if (!id) {
            id = uuidv4();
            objectIds.set(value, id);
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
            objectIds.get(object),
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
    ensureTransaction();
    if (typeof id !== "string") throw new Error("id must be a string");

    const existingArray = arrays.get(id);
    if (existingArray) return existingArray;
    const transactionId = this.transactionId;
    const array = new Proxy(
      {},
      {
        get: (_target, key) => {
          ensureTransaction();
          if (key === Symbol.iterator) {
            return function* () {
              let length = Deno.core.ops.op_cloudstate_array_length(
                transactionId,
                id
              );
              for (let i = 0; i < length; i++) {
                yield array[i];
              }
            };
          } else if (key === Symbol.toStringTag) {
            // TODO: handle console.log
          }
          let index = parseInt(key);

          if (isNaN(index)) {
            switch (key) {
              case "filter": {
                return (fn) => {
                  let arr = [];
                  for (let i = 0; i < array.length; i++) {
                    if (fn(array[i], i, array)) {
                      arr.push(array[i]);
                    }
                  }
                  return arr;
                };
              }
              case "toReversed": {
                return () => {
                  //TODO: LAZY-fy

                  let length = Deno.core.ops.op_cloudstate_array_length(
                    this.transactionId,
                    id
                  );
                  let reversed = [];
                  for (let i = length - 1; i >= 0; i--) {
                    reversed.push(array[i]);
                  }
                  return reversed;
                };
              }
              case "length": {
                return Deno.core.ops.op_cloudstate_array_length(
                  this.transactionId,
                  id
                );
              }
              case "reduce": {
                return (fn, initialValue) => {
                  let acc = initialValue;
                  for (let i = 0; i < array.length; i++) {
                    acc = fn(acc, array[i], i, array);
                  }
                  return acc;
                };
              }
              case "reverse": {
                return () => {
                  Deno.core.ops.op_cloudstate_array_reverse(
                    this.transactionId,
                    this.namespace,
                    id
                  );
                };
              }
              case "shift": {
                return () => {
                  return Deno.core.ops.op_cloudstate_array_shift(
                    this.transactionId,
                    this.namespace,
                    id
                  );
                };
              }
              case "some": {
                return (fn) => {
                  for (let i = 0; i < array.length; i++) {
                    if (fn(array[i], i, array)) return true;
                  }
                  return false;
                };
              }

              case "reduceRight": {
                return (fn, initialValue) => {
                  let acc = initialValue;
                  for (let i = array.length - 1; i >= 0; i--) {
                    acc = fn(acc, array[i], i, array);
                  }
                  return acc;
                };
              }

              case "constructor": {
                return Array;
              }
              case "at": {
                return (index) => {
                  return array[index];
                };
              }
              case "pop": {
                return () => {
                  console.log(
                    "popping",
                    this.transactionId,
                    this.namespace,
                    id
                  );
                  return Deno.core.ops.op_cloudstate_array_pop(
                    this.transactionId,
                    this.namespace,
                    id
                  );
                };
              }
              case "push": {
                return (...args) => {
                  let length = Deno.core.ops.op_cloudstate_array_length(
                    this.transactionId,
                    id
                  );

                  for (const arg of args) {
                    Deno.core.ops.op_cloudstate_array_set(
                      this.transactionId,
                      this.namespace,
                      id,
                      length,
                      // TODO: support nested arrays
                      this.packToReferenceOrPrimitive(arg)
                    );
                    length++;
                  }
                  return length;
                };
              }
              case "find": {
                return (fn) => {
                  for (let i = 0; i < array.length; i++) {
                    if (fn(array[i], i, array)) return array[i];
                  }
                  return undefined;
                };
              }
              case "sort": {
                return (fn) => {
                  Deno.core.ops.op_cloudstate_array_sort(
                    this.transactionId,
                    this.namespace,
                    id,
                    fn
                  );
                };
              }
              case "findIndex": {
                return (fn) => {
                  for (let i = 0; i < array.length; i++) {
                    if (fn(array[i], i, array)) return i;
                  }
                  return -1;
                };
              }
              case "findLast": {
                return (fn) => {
                  for (let i = array.length - 1; i >= 0; i--) {
                    if (fn(array[i], i, array)) return array[i];
                  }
                  return undefined;
                };
              }
              case "findLastIndex": {
                return (fn) => {
                  for (let i = array.length - 1; i >= 0; i--) {
                    if (fn(array[i], i, array)) return i;
                  }
                  return -1;
                };
              }
              case "indexOf": {
                return (value, fromIndex) => {
                  for (let i = fromIndex || 0; i < array.length; i++) {
                    if (array[i] === value) return i;
                  }
                  return -1;
                };
              }
              case "lastIndexOf": {
                return (value, fromIndex) => {
                  for (let i = fromIndex || array.length - 1; i >= 0; i--) {
                    if (array[i] === value) return i;
                  }
                  return -1;
                };
              }
              case "every": {
                return (fn) => {
                  let index = 0;
                  for (const item of array) {
                    if (!fn(item, index, array)) return false;
                    index++;
                  }
                  return true;
                };
              }
              case "join": {
                return (separator) => {
                  let str = "";
                  for (let i = 0; i < array.length; i++) {
                    str += array[i];
                    if (i < array.length - 1) str += separator;
                  }
                  return str;
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
              case "includes": {
                return (value) => {
                  const length = Deno.core.ops.op_cloudstate_array_length(
                    this.transactionId,
                    id
                  );
                  for (let i = 0; i < length; i++) {
                    if (array[i] === value) return true;
                  }
                  return false;
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
          return this.unpackFromReference(result);
        },
        set: (_target, key, value) => {
          ensureTransaction();
          console.log("SET ARRAY =>");
          if (typeof key !== "number") return;

          console.log("key", key);
          console.log("value instanceof Array", value instanceof Array);
          console.log("value instanceof Map", value instanceof Map);

          Deno.core.ops.op_cloudstate_array_set(
            this.transactionId,
            this.namespace,
            id,
            key,
            // TODO: support nested arrays
            this.packToReferenceOrPrimitive(value)
          );
        },
      }
    );

    Object.setPrototypeOf(array, Array.prototype);

    arrays.set(id, array);
    objectIds.set(array, id);

    return array;
  }

  #exportObject(object, data) {
    ensureTransaction();
    console.log("exporting object");
    const existingId = objectIds.get(object);
    if (!existingId) {
      const id = uuidv4();
      Deno.core.ops.op_cloudstate_object_set(
        this.transactionId,
        this.namespace,
        id,
        data
      );
      objectIds.set(object, id);
      objects.set(id, object);
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

    // TODO: unpack from reference?
    return this.getObject(id);
  }
}

// globalThis.CloudstateTransaction = CloudstateTransaction;
// globalThis.transaction = new CloudstateTransaction("namespace", "transaction");
globalThis.cloudstate = new Cloudstate("namespace", {});
// globalThis.transaction = globalThis.cloudstate.createTransaction();

function getRoot(...args) {
  ensureTransaction();
  return globalThis.transaction.getRoot(...args);
}

function getCloudstate(...args) {
  ensureTransaction();
  return globalThis.transaction.getCloudstate(...args);
}

function setRoot(...args) {
  ensureTransaction();
  return globalThis.transaction.setRoot(...args);
}

function ensureTransaction() {
  if (!globalThis.transaction) {
    globalThis.transaction = globalThis.cloudstate.createTransaction();
  }
}

function commit() {
  if (globalThis.transaction) {
    globalThis.transaction.commit();
    globalThis.transaction = undefined;
  } else {
    console.error("No transaction to commit");
  }
}

globalThis.getRoot = getRoot;
globalThis.setRoot = setRoot;
globalThis.commit = commit;
globalThis.getCloudstate = getCloudstate;
globalThis.ensureTransaction = ensureTransaction;
