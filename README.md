<h1 align="center">Cloudstate</h1>

<p align="center">
 <a href="https://docs.freestyle.dev/getting-started/intro">Get Started</a> ·  <a href="https://freestyle.sh">Freestyle</a> · <a href="https://docs.freestyle.dev">Docs</a> · <a href="https://discord.gg/YTRprVkdnz">Discord</a>
</p>

<b>Cloudstate</b> is a JavaScript database runtime. It is a foundational component of <a href="https://freestyle.sh">Freestyle</a>'s full stack JavaScript hosting.

We recommend you try out cloudstate via a freestyle template. Read our [getting started](https://docs.freestyle.dev/getting-started/intro) guide to learn more.

If you're interested in learning more about how cloudstate works behind the scenes, read on.

You can install the cloudstate cli alongside the freestyle cli. Run `npm install -g freestyle-sh@beta` or you can build it from source.

### `cloudstate run ./script.js`

The lowest level way to store data in cloudstate is via the `cloudstate run` command. You can use the global `setRoot` function with and id and object to store data.

```ts
const object = {
  counter: 0,
};

setRoot("test-root", object);
```

To retrieve an object from the database, call `getRoot` and pass in the identifier you used to store the object.

```ts
const object = getRoot("test-root");
```

If you have multiple references to the same object, those references will be preserved. The values of each property are also lazy loaded, so you don't need to worry about the complexity of objects stored in a single `setRoot` call.

```ts
const obj = {};
const objects = {
  a: obj,
  b: obj,
};

setRoot("objects", objects);
```

```ts
const objects = getRoot("objects");
objects.a === objects.b; // true
```

### `cloudstate serve ./script.js`

A more structured way to store data in cloudstate is via the `cloudstate serve` command. Instead of writing what the script should execute, you write classes. When you put a static id on a class, it will be automatically constructed and stored using `setRoot` for you. Methods will be exposed as endpoints which you can call via http.

```ts
export class CounterCS {
  static id = "counter";
  count = 0;

  increment() {
    return ++this.count;
  }
}
```

```
curl -X POST http://localhost:3000/cloudstate/instances/counter/increment -H "Content-Type: application/json" -d '{"params": []}'
```

### `npx freestyle dev`

The highest level api is built into freestyle's dev tooling. You can define classes anywhere in a full stack project using a decorator and they be automatically compiled into a single file and served.

```ts
import { cloudstate } from "freestyle-sh";

@cloudstate
class CounterCS {
  static id = "counter";
  count = 0;

  increment() {
    return ++this.count;
  }
}
```

Then you can easily query that data using `useCloud`.

```ts
import { type CounterCS } from "./schema.js";
import { useCloud } from "freestyle-sh";

const counter = useCloud<typeof CounterCS>("counter");

await counter.increment();
```

To learn more read the [freestyle docs](https://docs.freestyle.dev/getting-started/intro).

## Contributing

- Check out the [contributing guide](CONTRIBUTING.md) to learn about our development process.

## Support for JavaScript Objects

> [!NOTE]
> We currently support _most_ of the core JavaScript objects, with active development on all object constructors, methods, and property getters marked as "🚧 Planned".

> [!TIP]
> Tests are essential to building a robust and reliable runtime. If you'd like to contribute in a small but meaningful way, **please consider writing tests** for the methods and property getters marked as "🙂 Not Tested".

### Array

| Static method     | Status                                    | Notes |
| ----------------- | ----------------------------------------- | ----- |
| Array.from()      | ✅ [Tested](/runtime/tests/array_from.js) |       |
| Array.fromAsync() | 🚧 Planned                                |       |
| Array.isArray()   | 🚧 Planned                                |       |
| Array.of()        | 🚧 Planned                                |       |

| Instance method      | Status                                               | Notes                                             |
| -------------------- | ---------------------------------------------------- | ------------------------------------------------- |
| .at()                | ✅ [Tested](/runtime/tests/array_at.js)              |                                                   |
| .concat()            | 🚧 Planned                                           |                                                   |
| .copyWithin()        | 🚧 Planned                                           |                                                   |
| .entries()           | 🚧 Planned                                           |                                                   |
| .every()             | ✅ [Tested](/runtime/tests/array_every.js)           |                                                   |
| .filter()            | ✅ [Tested](/runtime/tests/array_filter.js)          |                                                   |
| .find()              | ✅ [Tested](/runtime/tests/array_find.js)            |                                                   |
| .findIndex()         | ✅ [Tested](/runtime/tests/array_find_index.js)      |                                                   |
| .findLast()          | ✅ [Tested](/runtime/tests/array_find_last.js)       |                                                   |
| .findLastIndex()     | ✅ [Tested](/runtime/tests/array_find_last_index.js) |                                                   |
| .flat()              | 🚧 Planned                                           |                                                   |
| .flatMap()           | 🚧 Planned                                           |                                                   |
| .forEach()           | 🚧 Planned                                           |                                                   |
| .includes()          | ✅ [Tested](/runtime/tests/array_includes.js)        |                                                   |
| .indexOf()           | ✅ [Tested](/runtime/tests/array_index_of.js)        |                                                   |
| .join()              | ✅ [Tested](/runtime/tests/array_join.js)            |                                                   |
| .keys()              | 🚧 Planned                                           |                                                   |
| .lastIndexOf()       | ✅ [Tested](/runtime/tests/array_last_index_of.js)   |                                                   |
| .map()               | ✅ [Tested](/runtime/tests/array_map.js)             |                                                   |
| .pop()               | ✅ [Tested](/runtime/tests/array_pop.js)             |                                                   |
| .push()              | ✅ [Tested](/runtime/tests/array_push.js)            |                                                   |
| .reduce()            | ✅ [Tested](/runtime/tests/array_reduce.js)          |                                                   |
| .reduceRight()       | ✅ [Tested](/runtime/tests/array_reduce_right.js)    |                                                   |
| .reverse()           | ✅ [Tested](/runtime/tests/array_reverse.js)         |                                                   |
| .shift()             | ✅ [Tested](/runtime/tests/array_shift.js)           |                                                   |
| .slice()             | 🚧 Planned                                           |                                                   |
| .some()              | ✅ [Tested](/runtime/tests/array_some.js)            |                                                   |
| .sort()              | 🚧 Planned                                           |                                                   |
| .splice()            | 🚧 Planned                                           |                                                   |
| \[Symbol.iterator]() | ✅ [Tested](/runtime/tests/array_iterator.js)        |                                                   |
| .toLocaleString()    | 🚧 Planned                                           |                                                   |
| .toReversed()        | ✅ [Tested](/runtime/tests/array_to_reversed.js)     | Not done lazily yet. See [#15](/../../issues/15). |
| .toSorted()          | 🚧 Planned                                           |                                                   |
| .toSpliced()         | 🚧 Planned                                           |                                                   |
| .toString()          | 🚧 Planned                                           |                                                   |
| .unshift()           | 🚧 Planned                                           |                                                   |
| .values()            | 🚧 Planned                                           |                                                   |
| .with()              | 🚧 Planned                                           |                                                   |

| Instance property     | Status                                      | Notes |
| --------------------- | ------------------------------------------- | ----- |
| .length               | ✅ [Tested](/runtime/tests/array_length.js) |       |
| \[Symbol.unscopables] | ❓ Unknown                                  |       |

### AsyncIterator

🤔 Considering

### BigInt

✅ Unchanged from V8

> [!NOTE]
> BigInts are stored as a `Box<\[u64]>` in Cloudstate.

### BigInt64Array

🤔 Considering

### BigUint64Array

🤔 Considering

### Boolean

✅ Unchanged from V8

### DataView

🤔 Considering

### Date

✅ Unchanged from V8

### Error

🤔 Considering

### EvalError

🤔 Considering

### Float16Array

🚧 Planned

### Float32Array

🚧 Planned

### Float64Array

🚧 Planned

### Int8Array

🚧 Planned

### Int16Array

🚧 Planned

### Int32Array

🚧 Planned

### Iterator

🤔 Considering

### Map

| Constructor | Status                              | Notes |
| ----------- | ----------------------------------- | ----- |
| Map()       | ✅ [Tested](/runtime/tests/maps.js) |       |

| Static method | Status                                      | Notes |
| ------------- | ------------------------------------------- | ----- |
| Map.groupBy() | ✅ [Tested](/runtime/tests/map_group_by.js) |       |

| Instance method      | Status                                       | Notes                                                       |
| -------------------- | -------------------------------------------- | ----------------------------------------------------------- |
| .clear()             | ✅ [Tested](/runtime/tests/map_clear.js)     |                                                             |
| .delete()            | ✅ [Tested](/runtime/tests/map_delete.js)    |                                                             |
| .entries()           | ✅ [Tested](/runtime/tests/map_entries.js)   |                                                             |
| .forEach()           | ✅ [Tested](/runtime/tests/map_for_each.js)  |                                                             |
| .get()               | ❌ [Tested](/runtime/tests/map_get.js)       | Crashes for non-existent keys. See [#12](/../../issues/12). |
| .has()               | ❌ [Tested](/runtime/tests/map_has.js)       | Crashes for non-existent keys. See [#13](/../../issues/13). |
| .keys()              | ✅ [Tested](/runtime/tests/map_keys.js)      |                                                             |
| .set()               | ✅ [Tested](/runtime/tests/map_empty_set.js) |                                                             |
| \[Symbol.iterator]() | ❌ [Tested](/runtime/tests/map_iterator.js)  | Zero iterations are made. See [#14](/../../issues/14).      |
| .values()            | ✅ [Tested](/runtime/tests/map_values.js)    |                                                             |

| Instance property | Status                                  | Notes |
| ----------------- | --------------------------------------- | ----- |
| .size             | ✅ [Tested](/runtime/tests/map_size.js) |       |

### Number

✅ Unchanged from V8

### Object

| Constructor                         | Status        | Notes |
| ----------------------------------- | ------------- | ----- |
| Object()                            | 🙂 Not Tested |       |
| Object initializer / literal syntax | 🙂 Not Tested |       |

| Static method                      | Status        | Notes |
| ---------------------------------- | ------------- | ----- |
| Object.assign()                    | 🙂 Not Tested |       |
| Object.create()                    | 🙂 Not Tested |       |
| Object.defineProperties()          | 🙂 Not Tested |       |
| Object.defineProperty()            | 🙂 Not Tested |       |
| Object.entries()                   | 🙂 Not Tested |       |
| Object.freeze()                    | 🙂 Not Tested |       |
| Object.fromEntries()               | 🙂 Not Tested |       |
| Object.getOwnPropertyDescriptor()  | 🙂 Not Tested |       |
| Object.getOwnPropertyDescriptors() | 🙂 Not Tested |       |
| Object.getOwnPropertyNames()       | 🙂 Not Tested |       |
| Object.getOwnPropertySymbols()     | 🙂 Not Tested |       |
| Object.getPrototypeOf()            | 🙂 Not Tested |       |
| Object.groupBy()                   | 🙂 Not Tested |       |
| Object.hasOwn()                    | 🙂 Not Tested |       |
| Object.is()                        | 🙂 Not Tested |       |
| Object.isExtensible()              | 🙂 Not Tested |       |
| Object.isFrozen()                  | 🙂 Not Tested |       |
| Object.isSealed()                  | 🙂 Not Tested |       |
| Object.keys()                      | 🙂 Not Tested |       |
| Object.preventExtensions()         | 🙂 Not Tested |       |
| Object.seal()                      | 🙂 Not Tested |       |
| Object.setPrototypeOf()            | 🙂 Not Tested |       |
| Object.values()                    | 🙂 Not Tested |       |

| Instance method         | Status        | Notes                                                                                                                     |
| ----------------------- | ------------- | ------------------------------------------------------------------------------------------------------------------------- |
| .\_\_defineGetter\_\_() | 🙂 Not Tested | ⚠️ [Deprecated](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/__defineGetter__) |
| .\_\_defineSetter\_\_() | 🙂 Not Tested | ⚠️ [Deprecated](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/__defineSetter__) |
| .\_\_lookupGetter\_\_() | 🙂 Not Tested | ⚠️ [Deprecated](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/__lookupGetter__) |
| .\_\_lookupSetter\_\_() | 🙂 Not Tested | ⚠️ [Deprecated](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/__lookupSetter__) |
| .hasOwnProperty()       | 🙂 Not Tested |                                                                                                                           |
| .isPrototypeOf()        | 🙂 Not Tested |                                                                                                                           |
| .propertyIsEnumerable() | 🙂 Not Tested |                                                                                                                           |
| .toLocaleString()       | 🙂 Not Tested |                                                                                                                           |
| .toString()             | 🙂 Not Tested |                                                                                                                           |
| .valueOf()              | 🙂 Not Tested |                                                                                                                           |

| Instance property | Status        | Notes                                                                                                          |
| ----------------- | ------------- | -------------------------------------------------------------------------------------------------------------- |
| .constructor      | 🙂 Not Tested |                                                                                                                |
| .\_\_proto\_\_    | 🙂 Not Tested | ⚠️ [Deprecated](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/proto) |

### RangeError

🤔 Considering

### ReferenceError

🤔 Considering

### RegExp

| Constructor | Status     | Notes |
| ----------- | ---------- | ----- |
| RegExp()    | 🚧 Planned |       |

| Instance method      | Status     | Notes                                                                                                            |
| -------------------- | ---------- | ---------------------------------------------------------------------------------------------------------------- |
| .compile()           | 🚧 Planned | ⚠️ [Deprecated](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/compile) |
| .exec()              | 🚧 Planned |                                                                                                                  |
| \[Symbol.match]()    | 🚧 Planned |                                                                                                                  |
| \[Symbol.matchAll]() | 🚧 Planned |                                                                                                                  |
| \[Symbol.replace]()  | 🚧 Planned |                                                                                                                  |
| \[Symbol.search]()   | 🚧 Planned |                                                                                                                  |
| \[Symbol.split]()    | 🚧 Planned |                                                                                                                  |
| .test()              | 🚧 Planned |                                                                                                                  |
| .toString()          | 🚧 Planned |                                                                                                                  |

| Instance property | Status     | Notes |
| ----------------- | ---------- | ----- |
| .dotAll           | 🚧 Planned |       |
| .flags            | 🚧 Planned |       |
| .global           | 🚧 Planned |       |
| .hasIndices       | 🚧 Planned |       |
| .ignoreCase       | 🚧 Planned |       |
| .lastIndex        | 🚧 Planned |       |
| .multiline        | 🚧 Planned |       |
| .source           | 🚧 Planned |       |
| .sticky           | 🚧 Planned |       |
| .unicode          | 🚧 Planned |       |
| .unicodeSets      | 🚧 Planned |       |

### Set

| Constructor | Status     | Notes |
| ----------- | ---------- | ----- |
| Set()       | 🚧 Planned |       |

| Instance method        | Status     | Notes |
| ---------------------- | ---------- | ----- |
| .add()                 | 🚧 Planned |       |
| .clear()               | 🚧 Planned |       |
| .delete()              | 🚧 Planned |       |
| .difference()          | 🚧 Planned |       |
| .entries()             | 🚧 Planned |       |
| .forEach()             | 🚧 Planned |       |
| .has()                 | 🚧 Planned |       |
| .intersection()        | 🚧 Planned |       |
| .isDisjointFrom()      | 🚧 Planned |       |
| .isSubsetOf()          | 🚧 Planned |       |
| .isSupersetOf()        | 🚧 Planned |       |
| .keys()                | 🚧 Planned |       |
| \[Symbol.iterator]()   | 🚧 Planned |       |
| .symmetricDifference() | 🚧 Planned |       |
| .union()               | 🚧 Planned |       |
| .values()              | 🚧 Planned |       |

| Instance property | Status     | Notes |
| ----------------- | ---------- | ----- |
| .size             | 🚧 Planned |       |

### SharedArrayBuffer

🤔 Considering

### String

✅ Unchanged from V8

### Symbol

🤔 Considering

### SyntaxError

🤔 Considering

### TypeError

🤔 Considering

### Uint8Array

🚧 Planned

### Uint8ClampedArray

🚧 Planned

### Uint16Array

🚧 Planned

### Uint32Array

🚧 Planned

### URIError

🤔 Considering

### WeakMap

🤔 Considering

### WeakSet

🤔 Considering

## Out of Scope

> [!IMPORTANT]
> The following objects are out of scope and will not be supported in Cloudstate.

- Function
- FinalizationRegistry
- globalThis
- InternalError ([⚠️ Non-standard](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/InternalError))
- Promise
- Proxy
- WeakRef
