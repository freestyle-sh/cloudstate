function uuidv4() {
  return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, function (c) {
    var r = (Math.random() * 16) | 0;
    var v = c == "x" ? r : (r & 0x3) | 0x8;
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
globalThis.CloudstateBlobReference = CloudstateBlobReference;

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

const objects = new Map();
const arrays = new Map();
const objectIds = new Map();
const cloudstateObjects = new Map();

const customClasses = [];

function hydrate(object, key, value) {
  if (value instanceof CloudstateObjectReference) {
    Object.defineProperty(object, key, {
      get: () => {
        const object = getObject(value.objectId);

        if (value.constructorName) {
          const constructor = customClasses.find(
            (klass) => klass.name === value.constructorName,
          );

          if (!constructor) {
            throw new Error(`Custom class ${value.constructorName} not found`);
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

  if (value instanceof CloudstateBlobReference) {
    const blob = new Blob();

    blob["text"] = async () => {
      return Deno.core.ops.op_cloudstate_blob_get_data(value.blobId);
    };

    blob["arrayBuffer"] = async () => {
      const text = Deno.core.ops.op_cloudstate_blob_get_data(value.blobId);
      const encoder = new TextEncoder();
      return encoder.encode(text).buffer;
    };

    blob["bytes"] = async () => {
      const text = Deno.core.ops.op_cloudstate_blob_get_data(value.blobId);
      const encoder = new TextEncoder();
      return encoder.encode(text);
    };

    Object.defineProperty(blob, "size", {
      get: () => {
        return Deno.core.ops.op_cloudstate_blob_get_size(value.blobId);
      },
    });

    Object.defineProperty(blob, "type", {
      get: () => {
        return Deno.core.ops.op_cloudstate_blob_get_type(value.blobId);
      },
    });

    Object.defineProperty(object, key, {
      value: blob,
    });

    objectIds.set(blob, value.blobId);
    objects.set(value.blobId, blob);
  }

  if (value instanceof CloudstateArrayReference) {
    Object.defineProperty(object, key, {
      value: getArray(value.objectId),
    });
  }

  if (value instanceof CloudstateMapReference) {
    Object.defineProperty(object, key, {
      value: getMap(value.objectId),
    });
  }
}

function commit() {
  // console.log("commiting");
  // const length = Array.from(objects.values()).length;
  // let i = 0;
  for (const value of objects.values()) {
    // if (i++ % 100 === 0) {
    //   console.log(`${i}/${length}`);
    // }
    setObject(value);
  }
  Deno.core.ops.op_cloudstate_commit_transaction();
}

function getMap(objectId) {
  const map = new Map();
  const mapSet = map.set;
  const mapGet = map.get;

  objectIds.set(map, objectId);
  map[Symbol.iterator] = () => {
    return Deno.core.ops.op_cloudstate_map_entries(objectId).values();
  };
  map["values"] = () => {
    let map_values = Deno.core.ops.op_cloudstate_map_values(objectId);
    return map_values.map((value) => unpackFromReference(value)).values();
  };
  map["keys"] = () => {
    return Deno.core.ops.op_cloudstate_map_keys(objectId).values();
  };

  map["entries"] = () => {
    let entries = Deno.core.ops.op_cloudstate_map_entries(objectId);
    return entries
      .map(([key, value]) => [key, unpackFromReference(value)])
      .values();
  };
  map["delete"] = (key) => {
    map.delete(key);
    return Deno.core.ops.op_cloudstate_map_delete(objectId, key);
  };

  map["clear"] = () => {
    return Deno.core.ops.op_cloudstate_map_clear(objectId);
  };

  map["forEach"] = (fn) => {
    const entries = map.entries();
    for (const entry of entries) {
      fn(entry[1], entry[0], map);
    }
  };

  map.has = (key) => {
    return Deno.core.ops.op_cloudstate_map_has(objectId, key);
  };

  map.get = (key) => {
    const result = mapGet.apply(map, [key]);
    if (result) return result;

    const object = Deno.core.ops.op_cloudstate_map_get(objectId, key);
    return unpackFromReference(object, () => {
      mapSet.apply(map, [key, object]);
    });
  };

  map.set = (key, set_value) => {
    const val = isPrimitive(set_value)
      ? set_value
      : new CloudstateObjectReference(setObject(set_value));

    Deno.core.ops.op_cloudstate_map_set(objectId, key, val);
    mapSet.apply(map, [key, val]);
  };

  Object.defineProperty(map, "size", {
    get: () => {
      return Deno.core.ops.op_cloudstate_map_size(objectId);
    },
  });

  return map;
}

function getObject(id) {
  if (typeof id !== "string") throw new Error("id must be a string");

  const existingObject = objects.get(id);
  if (existingObject) return existingObject;

  const object = Deno.core.ops.op_cloudstate_object_get(id);

  let klass = undefined;
  if (object.__cloudstate__constructorName) {
    klass = customClasses.find(
      (klass) =>
        klass.name === object.__cloudstate__constructorName.replace("_", ""),
    );
    if (!klass) {
      throw new Error(
        `Custom class ${object.__cloudstate__constructorName} not found`,
      );
    }
    delete object.__cloudstate__constructorName;
  }

  if (!object) return undefined;

  // for (const [key, value] of Object.entries(object)) {
  //   hydrate(object, key, value);
  // }

  // objectIds.set(object, id);
  // objects.set(id, object);

  // if (object.id && customClasses.includes(object.constructor)) {
  //   cloudstateObjects.set(object.id, object);
  // }

  const proxyObject = new Proxy(
    {},
    {
      getPrototypeOf() {
        return klass?.prototype || Object.prototype;
      },
      ownKeys(target) {
        let object = Deno.core.ops.op_cloudstate_object_get(id);
        delete object["__cloudstate__constructorName"];
        return Object.keys(object);
      },
      getOwnPropertyDescriptor(target, key) {
        return {
          enumerable: true,
          configurable: true,
          value: proxyObject[key],
        };
      },
      get(target, key) {
        // to make sure we have the latest values
        // todo: only load the keys we need
        const object = Deno.core.ops.op_cloudstate_object_get(id);

        delete object["__cloudstate__constructorName"];

        for (const [key, value] of Object.entries(object)) {
          hydrate(object, key, value);
        }

        if (key === "__proto__") {
          return Object.prototype;
        }

        if (key === "constructor") {
          return klass;
        }

        if (klass?.prototype && key in klass?.prototype) {
          return klass.prototype[key].bind(proxyObject);
        }

        if (key in Object.prototype) {
          return Object.prototype[key].bind(proxyObject);
        }

        return object[key];
      },
      set(target, key, value) {
        console.log("key", typeof key, key);
        const packed = packToReferenceOrPrimitive(value);
        console.log("packed", typeof packed, packed);
        Deno.core.ops.op_cloudstate_object_set_property(
          id,
          key,
          packed,
        );
        console.log("set", typeof key, key);

        return true;
      },
    },
  );

  objects.set(id, proxyObject);
  objectIds.set(proxyObject, id);

  return proxyObject;
}

function packToReferenceOrPrimitive(value) {
  if (isPrimitive(value)) {
    return value;
  }
  if (value instanceof Array) {
    console.log("isArray");
    return new CloudstateArrayReference(setObject(value));
  }
  if (value instanceof Map) {
    return new CloudstateMapReference(setObject(value));
  }
  if (value instanceof Blob) {
    return new CloudstateBlobReference(setObject(value));
  }
  if (
    value instanceof CloudstateObjectReference ||
    value instanceof CloudstateArrayReference ||
    value instanceof CloudstateMapReference ||
    value instanceof CloudstateBlobReference
  ) {
    console.error(
      `${value.objectId} is already a reference: packToReferenceOrPrimitive is unnecessary`,
    );
    return value;
  }
  if (value instanceof Object) {
    return new CloudstateObjectReference(setObject(value));
  }
  throw new Error(`${typeof value} cannot be serialized`);
}

function unpackFromReference(reference, notValidCb) {
  if (reference instanceof CloudstateObjectReference) {
    return getObject(reference.objectId);
  }
  if (reference instanceof CloudstateArrayReference) {
    return getArray(reference.objectId);
  }
  if (reference instanceof CloudstateMapReference) {
    return getMap(reference.objectId);
  }
  if (notValidCb) {
    notValidCb.call();
  }
  return reference;
}

function getCloudstate(id) {
  if (typeof id !== "string") throw new Error("id must be a string");
  if (cloudstateObjects.has(id)) {
    return cloudstateObjects.get(id);
  } else {
    const value = Deno.core.ops.op_cloudstate_cloudstate_get(id);
    if (!value) {
      console.error("Cloudstate object not found", id);
      return undefined;
    }
    return getObject(value.objectId);
  }
}

function setObject(object, visited = new Set()) {
  if (typeof object !== "object") throw new Error("object must be an object");
  visited.add(object);
  const stack = [object];

  let rootObject = undefined;
  while (stack.length > 0) {
    const object = stack.pop();

    if (object instanceof Blob) {
      let id = objectIds.get(object);
      if (!id) {
        id = uuidv4();
        objectIds.set(object, id);
        objects.set(id, object);

        object.text().then((text) => {
          Deno.core.ops.op_cloudstate_blob_set(id, object.type, text);
          console.log("set blob " + id);
        });
      }

      if (!rootObject) {
        rootObject = id;
      }

      visited.add(object);

      continue;
    }

    if (object instanceof Map) {
      for (const [key, value] of object.entries()) {
        if (isPrimitive(value)) {
          Deno.core.ops.op_cloudstate_map_set(
            objectIds.get(object),
            key,
            value,
          );
        } else {
          const id = setObject(value, visited);
          Deno.core.ops.op_cloudstate_map_set(
            objectIds.get(object),
            key,
            new CloudstateObjectReference(id),
          );
        }
      }

      // todo: why does this break things?
      // if (!rootObject) {
      //   rootObject = id;
      // }

      visited.add(object);

      continue;
    }

    const isArray = object instanceof Array;
    const flatObject = isArray ? [] : {};

    if (object.constructor) {
      Object.setPrototypeOf(flatObject, object.constructor.prototype);
    }

    for (let [key, value] of Object.entries(object)) {
      if (isArray) key = parseInt(key);
      if (value === undefined) continue;

      if (isPrimitive(value)) {
        flatObject[key] = value;
      } else if (typeof value === "object") {
        let id = objectIds.get(value);

        if (!id) {
          id = uuidv4();
          objectIds.set(value, id);
          objects.set(id, value);

          if (value instanceof Blob) {
            value.text().then((text) => {
              Deno.core.ops.op_cloudstate_blob_set(id, value.type, text);
              console.log("set blob " + id);
            });
          }
        }

        if (value instanceof Map) {
          flatObject[key] = new CloudstateMapReference(id);
        } else if (value instanceof Array) {
          flatObject[key] = new CloudstateArrayReference(id);
        } else if (value instanceof Blob) {
          flatObject[key] = new CloudstateBlobReference(id);
        } else {
          flatObject[key] = new CloudstateObjectReference(
            id,
            [Object, Array, Map].includes(value.constructor)
              ? undefined
              : value.constructor?.name,
          );
        }

        if (value instanceof Blob) continue;

        if (!visited.has(value)) {
          visited.add(value);
          stack.push(value);
        }
      } else {
        throw new Error(
          `property ${key} of type ${typeof value} on object ${object} cannot be serialized`,
        );
      }
    }

    if (flatObject instanceof Array) {
      let id = objectIds.get(object);
      if (!id) {
        id = uuidv4();
        objectIds.set(object, id);
      }

      flatObject.forEach((item, i) => {
        Deno.core.ops.op_cloudstate_array_set(id, i, item);
      });
      if (!rootObject) {
        rootObject = id;
      }
    } else {
      const id = exportObject(object, flatObject);
      if (!rootObject) {
        rootObject = id;
      }
    }
  }

  return rootObject;
}

function getArray(id) {
  if (typeof id !== "string") throw new Error("id must be a string");

  const existingArray = arrays.get(id);
  if (existingArray) return existingArray;
  const array = new Proxy(
    {},
    {
      get: (_target, key) => {
        if (key === Symbol.iterator) {
          return function* () {
            let length = Deno.core.ops.op_cloudstate_array_length(id);
            for (let i = 0; i < length; i++) {
              yield array[i];
            }
          };
        }

        let index = parseInt(key);

        if (isNaN(index)) {
          return handleArrayMethods(key, array, id);
        }

        const result = Deno.core.ops.op_cloudstate_array_get(id, index);
        return unpackFromReference(result);
      },
      set: (_target, key, value) => {
        if (typeof key !== "number") return;
        Deno.core.ops.op_cloudstate_array_set(
          id,
          key,
          packToReferenceOrPrimitive(value),
        );
      },
    },
  );

  Object.setPrototypeOf(array, Array.prototype);

  arrays.set(id, array);
  objectIds.set(array, id);

  return array;
}

function exportObject(object, data) {
  const existingId = objectIds.get(object);
  if (!existingId) {
    const id = uuidv4();
    Deno.core.ops.op_cloudstate_object_set(id, data);
    objectIds.set(object, id);
    objects.set(id, object);
    return id;
  } else {
    Deno.core.ops.op_cloudstate_object_set(existingId, data);
    return existingId;
  }
}

function setRoot(alias, object) {
  if (typeof alias !== "string") throw new Error("alias must be a string");
  if (typeof object !== "object") throw new Error("object must be an object");

  const id = setObject(object);
  Deno.core.ops.op_cloudstate_object_root_set(alias, id);
}

function getRoot(alias) {
  if (typeof alias !== "string") throw new Error("alias must be a string");

  const id = Deno.core.ops.op_cloudstate_object_root_get(alias);
  if (!id) return undefined;
  return getObject(id);
}

function handleArrayMethods(key, array, id) {
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
        let length = Deno.core.ops.op_cloudstate_array_length(id);
        let reversed = [];
        for (let i = length - 1; i >= 0; i--) {
          reversed.push(array[i]);
        }
        return reversed;
      };
    }
    case "length": {
      return Deno.core.ops.op_cloudstate_array_length(id);
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
        Deno.core.ops.op_cloudstate_array_reverse(id);
      };
    }
    case "shift": {
      return () => {
        return Deno.core.ops.op_cloudstate_array_shift(id);
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
        return Deno.core.ops.op_cloudstate_array_pop(id);
      };
    }
    case "push": {
      return (...args) => {
        let length = Deno.core.ops.op_cloudstate_array_length(id);
        for (const arg of args) {
          Deno.core.ops.op_cloudstate_array_set(
            id,
            length,
            packToReferenceOrPrimitive(arg),
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
        Deno.core.ops.op_cloudstate_array_sort(id, fn);
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
        const length = Deno.core.ops.op_cloudstate_array_length(id);
        const result = [];
        for (let i = 0; i < length; i++) {
          result.push(array[i]);
        }
        return result;
      };
    }
    case "includes": {
      return (value) => {
        const length = Deno.core.ops.op_cloudstate_array_length(id);
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
    case "entries": {
      return function* () {
        for (let i = 0; i < array.length; i++) {
          yield [i, array[i]];
        }
      };
    }
    case "then": {
      // I think this is called when sometime tries to await an array
      return undefined;
    }
    default: {
      throw new Error(`Array.${key} is not supported`);
    }
  }
}

function registerCustomClass(klass) {
  customClasses.push(klass);
}

globalThis.getRoot = getRoot;
globalThis.setRoot = setRoot;
globalThis.commit = commit;
globalThis.getCloudstate = getCloudstate;
globalThis.registerCustomClass = registerCustomClass;
