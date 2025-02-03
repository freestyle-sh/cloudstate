function span(name, callback) {
  let spanName = "op_tracing_span_" + name;

  if (Deno.core.ops[spanName]) {
    Deno.core.ops[spanName]();
  } else {
    throw new Error(
      `Span ${spanName} not found. This span must be added in the cloudstate::js_spans module.`,
    );
  }

  const result = callback();

  Deno.core.ops.op_tracing_span_finish();

  return result;
}

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

// list of objects that we know came from cloudstate and aren't new objects
const trackedObjects = new Set();

const customClasses = [];

function hydrate(object, key, value) {
  return span("hydrate", () => {
    if (value instanceof CloudstateObjectReference) {
      Object.defineProperty(object, key, {
        get: () => {
          const object = getObject(value.objectId);

          if (value.constructorName) {
            const constructor = customClasses.find(
              (klass) => klass.name === value.constructorName,
            );

            if (!constructor) {
              throw new Error(
                `Custom class ${value.constructorName} not found`,
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

    if (value instanceof CloudstateBlobReference) {
      const blob = new Blob();

      blob["text"] = async () => {
        return Deno.core.ops.op_cloudstate_blob_get_text(value.blobId);
      };

      blob["arrayBuffer"] = async () => {
        /* get_data now returns Array Buffer  */
        const buffer = Deno.core.ops.op_cloudstate_blob_get_array_buffer(
          value.blobId,
        );
        return buffer;
      };

      blob["slice"] = (start, end, type) => {
        if (start < 0 || end < 0) {
          throw new Error("start and end must be positive");
        }
        let arrBuffer = Deno.core.ops.op_cloudstate_blob_slice(
          value.blobId,
          start,
          end,
        );
        return new Blob([arrBuffer], { type: type });
      };

      blob["bytes"] = async () => {
        const blob = Deno.core.ops.op_cloudstate_blob_get_uint8array(
          value.blobId,
        );
        return blob;
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
  });
}

function commit() {
  return span("commit", () => {
    for (const value of objects.values()) {
      setObject(value);
    }
    Deno.core.ops.op_cloudstate_commit_transaction();
  });
}

function getMap(objectId) {
  return span("get_map", () => {
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

    trackedObjects.add(map);

    return map;
  });
}

function getObject(id) {
  return span("get_object", () => {
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
          const packed = packToReferenceOrPrimitive(value);
          Deno.core.ops.op_cloudstate_object_set_property(
            id,
            key,
            packed,
          );

          return true;
        },
      },
    );

    objects.set(id, proxyObject);
    objectIds.set(proxyObject, id);

    return proxyObject;
  });
}

function packToReferenceOrPrimitive(value) {
  return span("pack_to_reference_or_primitive", () => {
    if (isPrimitive(value)) {
      return value;
    }
    if (value instanceof Array) {
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
  });
}

function unpackFromReference(reference, notValidCb) {
  return span("unpack_from_reference", () => {
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
  });
}

function getCloudstate(id) {
  return span("get_cloudstate", () => {
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
  });
}

function setObject(object, visited = new Set()) {
  return span("set_object", () => {
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

          object.arrayBuffer().then(async (buffer) => {
            Deno.core.ops.op_cloudstate_blob_set(
              id,
              object.type,
              buffer,
            );
          });
        }

        if (!rootObject) {
          rootObject = id;
        }

        visited.add(object);

        continue;
      }

      if (object instanceof Map) {
        if (trackedObjects.has(object)) {
          continue;
        }

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
              value.arrayBuffer().then(async (buffer) => {
                Deno.core.ops.op_cloudstate_blob_set(
                  id,
                  value.type,
                  buffer,
                );
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
  });
}

function getArray(id) {
  return span("get_array", () => {
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

          if (key === Symbol.toStringTag) {
            return `Array(${array.length})`;
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
  });
}

function exportObject(object, data) {
  return span("export_object", () => {
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
  });
}

function setRoot(alias, object) {
  return span("set_root", () => {
    if (typeof alias !== "string") throw new Error("alias must be a string");
    if (typeof object !== "object") throw new Error("object must be an object");

    const id = setObject(object);
    Deno.core.ops.op_cloudstate_object_root_set(alias, id);
  });
}

function getRoot(alias) {
  return span("get_root", () => {
    if (typeof alias !== "string") throw new Error("alias must be a string");

    const id = Deno.core.ops.op_cloudstate_object_root_get(alias);
    if (!id) return undefined;
    return getObject(id);
  });
}

function handleArrayMethods(key, array, id) {
  switch (key) {
    case "filter": {
      return (fn) => {
        return span("array_filter", () => {
          let arr = [];
          for (let i = 0; i < array.length; i++) {
            if (fn(array[i], i, array)) {
              arr.push(array[i]);
            }
          }
          return arr;
        });
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
        if (index < 0) {
          return array[array.length + index];
        } else {
          return array[index];
        }
      };
    }
    case "pop": {
      return () => {
        const raw = Deno.core.ops.op_cloudstate_array_pop(id);
        return unpackFromReference(raw);
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
        for (let i = 0; i < array.length; i++) {
          for (let j = i + 1; j < array.length; j++) {
            if (fn(array[i], array[j]) > 0) {
              let temp = array[i];
              array[i] = array[j];
              array[j] = temp;
            }
          }
        }
      };
    }
    case "forEach": {
      return (fn) => {
        for (let i = 0; i < array.length; i++) {
          fn(array[i], i, array);
        }
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
    case "unshift": {
      return (...args) => {
        let length = Deno.core.ops.op_cloudstate_array_length(id);
        for (let i = length - 1; i >= 0; i--) {
          Deno.core.ops.op_cloudstate_array_set(
            id,
            i + args.length,
            array[i],
          );
        }

        for (let i = 0; i < args.length; i++) {
          Deno.core.ops.op_cloudstate_array_set(
            id,
            i,
            packToReferenceOrPrimitive(args[i]),
          );
        }

        return length + args.length;
      };
    }
    case "slice": {
      return (start, end) => {
        let arr = [];
        for (let i = start; i < end; i++) {
          arr.push(array[i]);
        }
        return arr;
      };
    }

    case "splice": {
      return (start, deleteCount, ...items) => {
        return span("array_splice", () => {
          const length = Deno.core.ops.op_cloudstate_array_length(id);
          const deleted = [];
          for (let i = start; i < start + deleteCount; i++) {
            deleted.push(array[i]);
          }

          const newLength = length - deleteCount + items.length;
          for (let i = length - 1; i >= start + deleteCount; i--) {
            Deno.core.ops.op_cloudstate_array_set(
              id,
              i + items.length - deleteCount,
              array[i],
            );
          }

          for (let i = 0; i < items.length; i++) {
            Deno.core.ops.op_cloudstate_array_set(
              id,
              start + i,
              packToReferenceOrPrimitive(items[i]),
            );
          }

          // remove from the end
          for (let i = length - 1; i >= newLength; i--) {
            Deno.core.ops.op_cloudstate_array_pop(id);
          }

          return deleted;
        });
      };
    }
    default: {
      throw new Error(`Array.${key} is not supported`);
    }
  }
}

function registerCustomClass(klass) {
  customClasses.push(klass);
}

function __setReadOnly() {
  Deno.core.ops.op_cloudstate_set_read_only();
}

globalThis.getRoot = getRoot;
globalThis.setRoot = setRoot;
globalThis.commit = commit;
globalThis.getCloudstate = getCloudstate;
globalThis.registerCustomClass = registerCustomClass;
globalThis.__setReadOnly = __setReadOnly;
