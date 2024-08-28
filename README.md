# ![cloudstate banner](https://github.com/user-attachments/assets/c580008f-98da-47d7-9a82-7abf423a426b)

<p align="center">
  <a href="https://freestyle.sh">Freestyle</a> ·
 <a href="https://docs.freestyle.dev/getting-started/intro">Get Started</a> · <a href="https://docs.freestyle.dev">Docs</a> · <a href="https://discord.gg/YTRprVkdnz">Discord</a>
</p>

<p align="center">
<b>Cloudstate</b> is a combined JavaScript Runtime and Database Engine that allows developers to persist their data with just JavaScript.
</p>

<p align="center">
Node.js enables developers to create servers in JavaScript. <b>Cloudstate</b> enables developers to create databases in JavaScript.
</p>

```ts
// schema.js
export class CounterCS {
  static id = "counter";
  count = 0;

  increment() {
    return ++this.count;
  }
}
```

```bash
cloudstate serve ./schema.js --watch
```

```ts
// node.js
import { type CounterCS } from "./schema.js";
import { useCloud } from "freestyle";

const counter = useCloud<typeof CounterCS>("counter");

await counter.increment();
```

## Support for JavaScript Objects

> [!NOTE]
> We currently support most of the core JavaScript objects, with active development on all object constructors, methods, and property getters marked as "🚧 Planned".

> [!TIP]
> Tests are essential to building a robust and reliable runtime. If you'd like to contribute in a small but meaningful way, please consider writing tests for the methods and property getters marked as "🙂 Not Tested".

### Array

| Static method     | Status        | Notes |
| ----------------- | ------------- | ----- |
| Array.from()      | 🙂 Not Tested |       |
| Array.fromAsync() | 🚧 Planned    |       |
| Array.isArray()   | 🚧 Planned    |       |
| Array.of()        | 🚧 Planned    |       |

| Instance method      | Status                                            | Notes               |
| -------------------- | ------------------------------------------------- | ------------------- |
| .at()                | ✅ [Tested](/runtime/tests/array_at.js)           |                     |
| .concat()            | 🚧 Planned                                        |                     |
| .copyWithin()        | 🚧 Planned                                        |                     |
| .entries()           | 🚧 Planned                                        |                     |
| .every()             | ✅ [Tested](/runtime/tests/array_every.js)        |                     |
| .filter()            | ✅ [Tested](/runtime/tests/array_filter.js)       |                     |
| .find()              | 🙂 Not Tested                                     |                     |
| .findIndex()         | 🙂 Not Tested                                     |                     |
| .findLastIndex()     | 🙂 Not Tested                                     |                     |
| .flat()              | 🚧 Planned                                        |                     |
| .flatMap()           | 🚧 Planned                                        |                     |
| .forEach()           | 🚧 Planned                                        |                     |
| .includes()          | ✅ [Tested](/runtime/tests/array_includes.js)     |                     |
| .indexOf()           | 🙂 Not Tested                                     |                     |
| .join()              | ✅ [Tested](/runtime/tests/array_join.js)         |                     |
| .keys()              | 🚧 Planned                                        |                     |
| .lastIndexOf()       | 🚧 Planned                                        |                     |
| .map()               | 🙂 Not Tested                                     |                     |
| .pop()               | 🙂 Not Tested                                     |                     |
| .push()              | 🙂 Not Tested                                     |                     |
| .reduce()            | ✅ [Tested](/runtime/tests/array_reduce.js)       |                     |
| .reduceRight()       | ✅ [Tested](/runtime/tests/array_reduce_right.js) |                     |
| .reverse()           | 🙂 Not Tested                                     |                     |
| .shift()             | 🙂 Not Tested                                     |                     |
| .slice()             | 🚧 Planned                                        |                     |
| .some()              | 🙂 Not Tested                                     |                     |
| .sort()              | 🚧 Planned                                        |                     |
| .splice()            | 🚧 Planned                                        |                     |
| \[Symbol.iterator]() | 🙂 Not Tested                                     |                     |
| .toLocaleString()    | 🚧 Planned                                        |                     |
| .toReversed()        | 🙂 Not Tested                                     | Not done lazily yet |
| .toSorted()          | 🚧 Planned                                        |                     |
| .toSpliced()         | 🚧 Planned                                        |                     |
| .toString()          | 🚧 Planned                                        |                     |
| .unshift()           | 🚧 Planned                                        |                     |
| .values()            | 🚧 Planned                                        |                     |
| .with()              | 🚧 Planned                                        |                     |

| Instance property     | Status        | Notes |
| --------------------- | ------------- | ----- |
| .length               | 🙂 Not Tested |       |
| \[Symbol.unscopables] | ❓ Unknown    |       |

### AsyncIterator

🤔 Considering

### BigInt

| Static method    | Status     | Notes |
| ---------------- | ---------- | ----- |
| BigInt.asIntN()  | 🚧 Planned |       |
| BigInt.asUintN() | 🚧 Planned |       |

| Instance method   | Status     | Notes |
| ----------------- | ---------- | ----- |
| .toLocaleString() | 🚧 Planned |       |
| .toString()       | 🚧 Planned |       |
| .valueOf()        | 🚧 Planned |       |

### BigInt64Array

🤔 Considering

### BigUint64Array

🤔 Considering

### Boolean

| Constructor | Status     | Notes |
| ----------- | ---------- | ----- |
| Boolean()   | 🚧 Planned |       |

| Instance method | Status     | Notes |
| --------------- | ---------- | ----- |
| .toString()     | 🚧 Planned |       |
| .valueOf()      | 🚧 Planned |       |

### DataView

🤔 Considering

### Date

| Constructor | Status        | Notes |
| ----------- | ------------- | ----- |
| Date()      | 🙂 Not Tested |       |

| Static method | Status        | Notes |
| ------------- | ------------- | ----- |
| Date.now()    | 🙂 Not Tested |       |
| Date.parse()  | 🙂 Not Tested |       |
| Date.UTC()    | 🙂 Not Tested |       |

| Instance method         | Status        | Notes                                                                                                          |
| ----------------------- | ------------- | -------------------------------------------------------------------------------------------------------------- |
| .getDate()              | 🙂 Not Tested |                                                                                                                |
| .getDay()               | 🙂 Not Tested |                                                                                                                |
| .getFullYear()          | 🙂 Not Tested |                                                                                                                |
| .getHours()             | 🙂 Not Tested |                                                                                                                |
| .getMilliseconds()      | 🙂 Not Tested |                                                                                                                |
| .getMinutes()           | 🙂 Not Tested |                                                                                                                |
| .getMonth()             | 🙂 Not Tested |                                                                                                                |
| .getSeconds()           | 🙂 Not Tested |                                                                                                                |
| .getTime()              | 🙂 Not Tested |                                                                                                                |
| .getTimezoneOffset()    | 🙂 Not Tested |                                                                                                                |
| .getUTCDate()           | 🙂 Not Tested |                                                                                                                |
| .getUTCDay()            | 🙂 Not Tested |                                                                                                                |
| .getUTCFullYear()       | 🙂 Not Tested |                                                                                                                |
| .getUTCHours()          | 🙂 Not Tested |                                                                                                                |
| .getUTCMilliseconds()   | 🙂 Not Tested |                                                                                                                |
| .getUTCMinutes()        | 🙂 Not Tested |                                                                                                                |
| .getUTCMonth()          | 🙂 Not Tested |                                                                                                                |
| .getUTCSeconds()        | 🙂 Not Tested |                                                                                                                |
| .getYear()              | 🙂 Not Tested | ⚠️ [Deprecated](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getYear) |
| .setDate()              | 🙂 Not Tested |                                                                                                                |
| .setFullYear()          | 🙂 Not Tested |                                                                                                                |
| .setHours()             | 🙂 Not Tested |                                                                                                                |
| .setMilliseconds()      | 🙂 Not Tested |                                                                                                                |
| .setMinutes()           | 🙂 Not Tested |                                                                                                                |
| .setMonth()             | 🙂 Not Tested |                                                                                                                |
| .setSeconds()           | 🙂 Not Tested |                                                                                                                |
| .setTime()              | 🙂 Not Tested |                                                                                                                |
| .setUTCDate()           | 🙂 Not Tested |                                                                                                                |
| .setUTCFullYear()       | 🙂 Not Tested |                                                                                                                |
| .setUTCHours()          | 🙂 Not Tested |                                                                                                                |
| .setUTCMilliseconds()   | 🙂 Not Tested |                                                                                                                |
| .setUTCMinutes()        | 🙂 Not Tested |                                                                                                                |
| .setUTCMonth()          | 🙂 Not Tested |                                                                                                                |
| .setUTCSeconds()        | 🙂 Not Tested |                                                                                                                |
| .setYear()              | 🙂 Not Tested | ⚠️ [Deprecated](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/setYear) |
| \[Symbol.toPrimitive]() | ❓ Unknown    |                                                                                                                |
| .toDateString()         | 🙂 Not Tested |                                                                                                                |
| .toISOString()          | 🙂 Not Tested |                                                                                                                |
| .toJSON()               | 🙂 Not Tested |                                                                                                                |
| .toLocaleDateString()   | 🙂 Not Tested |                                                                                                                |
| .toLocaleString()       | 🙂 Not Tested |                                                                                                                |
| .toLocaleTimeString()   | 🙂 Not Tested |                                                                                                                |
| .toTimeString()         | 🙂 Not Tested |                                                                                                                |
| .toUTCString()          | 🙂 Not Tested |                                                                                                                |
| .valueOf()              | 🙂 Not Tested |                                                                                                                |

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

| Constructor | Status        | Notes |
| ----------- | ------------- | ----- |
| Map()       | 🙂 Not Tested |       |

| Static method | Status        | Notes |
| ------------- | ------------- | ----- |
| Map.groupBy() | 🙂 Not Tested |       |

| Instance method      | Status        | Notes |
| -------------------- | ------------- | ----- |
| .clear()             | 🙂 Not Tested |       |
| .delete()            | 🙂 Not Tested |       |
| .entries()           | 🙂 Not Tested |       |
| .forEach()           | 🙂 Not Tested |       |
| .get()               | 🙂 Not Tested |       |
| .has()               | 🙂 Not Tested |       |
| .keys()              | 🙂 Not Tested |       |
| .set()               | 🙂 Not Tested |       |
| \[Symbol.iterator]() | 🙂 Not Tested |       |
| .values()            | 🙂 Not Tested |       |

| Instance property | Status        | Notes |
| ----------------- | ------------- | ----- |
| .size             | 🙂 Not Tested |       |

### Number

| Constructor | Status        | Notes |
| ----------- | ------------- | ----- |
| Number()    | 🙂 Not Tested |       |

| Static method          | Status        | Notes |
| ---------------------- | ------------- | ----- |
| Number.isFinite()      | 🙂 Not Tested |       |
| Number.isInteger()     | 🙂 Not Tested |       |
| Number.isNaN()         | 🙂 Not Tested |       |
| Number.isSafeInteger() | 🙂 Not Tested |       |
| Number.parseFloat()    | 🙂 Not Tested |       |
| Number.parseInt()      | 🙂 Not Tested |       |

| Instance method   | Status        | Notes |
| ----------------- | ------------- | ----- |
| .toExponential()  | 🙂 Not Tested |       |
| .toFixed()        | 🙂 Not Tested |       |
| .toLocaleString() | 🙂 Not Tested |       |
| .toPrecision()    | 🙂 Not Tested |       |
| .toString()       | 🙂 Not Tested |       |
| .valueOf()        | 🙂 Not Tested |       |

### Object

| Constructor                         | Status        | Notes |
| ----------------------------------- | ------------- | ----- |
| Object()                            | 🙂 Not Tested |       |
| Object initializer / literal syntax | 🙂 Not Tested |       |

| Static method                      | Status     | Notes |
| ---------------------------------- | ---------- | ----- |
| Object.assign()                    | 🚧 Planned |       |
| Object.create()                    | 🚧 Planned |       |
| Object.defineProperties()          | 🚧 Planned |       |
| Object.defineProperty()            | 🚧 Planned |       |
| Object.entries()                   | 🚧 Planned |       |
| Object.freeze()                    | 🚧 Planned |       |
| Object.fromEntries()               | 🚧 Planned |       |
| Object.getOwnPropertyDescriptor()  | 🚧 Planned |       |
| Object.getOwnPropertyDescriptors() | 🚧 Planned |       |
| Object.getOwnPropertyNames()       | 🚧 Planned |       |
| Object.getOwnPropertySymbols()     | 🚧 Planned |       |
| Object.getPrototypeOf()            | 🚧 Planned |       |
| Object.groupBy()                   | 🚧 Planned |       |
| Object.hasOwn()                    | 🚧 Planned |       |
| Object.is()                        | 🚧 Planned |       |
| Object.isExtensible()              | 🚧 Planned |       |
| Object.isFrozen()                  | 🚧 Planned |       |
| Object.isSealed()                  | 🚧 Planned |       |
| Object.keys()                      | 🚧 Planned |       |
| Object.preventExtensions()         | 🚧 Planned |       |
| Object.seal()                      | 🚧 Planned |       |
| Object.setPrototypeOf()            | 🚧 Planned |       |
| Object.values()                    | 🚧 Planned |       |

| Instance method            | Status     | Notes                                                                                                                     |
| -------------------------- | ---------- | ------------------------------------------------------------------------------------------------------------------------- |
| ⚠️ .\_\_defineGetter\_\_() | 🚧 Planned | ⚠️ [Deprecated](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/__defineGetter__) |
| ⚠️ .\_\_defineSetter\_\_() | 🚧 Planned | ⚠️ [Deprecated](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/__defineSetter__) |
| ⚠️ .\_\_lookupGetter\_\_() | 🚧 Planned | ⚠️ [Deprecated](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/__lookupGetter__) |
| ⚠️ .\_\_lookupSetter\_\_() | 🚧 Planned | ⚠️ [Deprecated](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/__lookupSetter__) |
| .hasOwnProperty()          | 🚧 Planned |                                                                                                                           |
| .isPrototypeOf()           | 🚧 Planned |                                                                                                                           |
| .propertyIsEnumerable()    | 🚧 Planned |                                                                                                                           |
| .toLocaleString()          | 🚧 Planned |                                                                                                                           |
| .toString()                | 🚧 Planned |                                                                                                                           |
| .valueOf()                 | 🚧 Planned |                                                                                                                           |

| Instance property | Status     | Notes                                                                                                          |
| ----------------- | ---------- | -------------------------------------------------------------------------------------------------------------- |
| .constructor      | 🚧 Planned |                                                                                                                |
| ⚠️ .\_\_proto\_\_ | 🚧 Planned | ⚠️ [Deprecated](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/proto) |

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
| ⚠️ .compile()        | 🚧 Planned | ⚠️ [Deprecated](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/compile) |
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

| Constructor | Status        | Notes |
| ----------- | ------------- | ----- |
| String()    | 🙂 Not Tested |       |

| Static method          | Status        | Notes |
| ---------------------- | ------------- | ----- |
| String.fromCharCode()  | 🙂 Not Tested |       |
| String.fromCodePoint() | 🙂 Not Tested |
| String.raw()           | 🙂 Not Tested |       |

| Instance method      | Status        | Notes                                                                                                              |
| -------------------- | ------------- | ------------------------------------------------------------------------------------------------------------------ |
| ⚠️ .anchor()         | 🙂 Not Tested | ⚠️ [Deprecated](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/anchor)    |
| .at()                | 🙂 Not Tested |                                                                                                                    |
| ⚠️ .big()            | 🙂 Not Tested | ⚠️ [Deprecated](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/big)       |
| ⚠️ .blink()          | 🙂 Not Tested | ⚠️ [Deprecated](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/blink)     |
| ⚠️ .bold()           | 🙂 Not Tested | ⚠️ [Deprecated](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/bold)      |
| .charAt()            | 🙂 Not Tested |                                                                                                                    |
| .charCodeAt()        | 🙂 Not Tested |                                                                                                                    |
| .codePointAt()       | 🙂 Not Tested |                                                                                                                    |
| .concat()            | 🙂 Not Tested |                                                                                                                    |
| .endsWith()          | 🙂 Not Tested |                                                                                                                    |
| ⚠️ .fixed()          | 🙂 Not Tested | ⚠️ [Deprecated](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/fixed)     |
| ⚠️.fontcolor()       | 🙂 Not Tested | ⚠️ [Deprecated](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/fontcolor) |
| ⚠️.fontsize()        | 🙂 Not Tested | ⚠️ [Deprecated](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/fontsize)  |
| .includes()          | 🙂 Not Tested |                                                                                                                    |
| .indexOf()           | 🙂 Not Tested |                                                                                                                    |
| .isWellFormed()      | 🙂 Not Tested |                                                                                                                    |
| ⚠️ .italics()        | 🙂 Not Tested | ⚠️ [Deprecated](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/italics)   |
| .lastIndexOf()       | 🙂 Not Tested |                                                                                                                    |
| ⚠️ .link()           | 🙂 Not Tested | ⚠️ [Deprecated](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/link)      |
| .localeCompare()     | 🙂 Not Tested |                                                                                                                    |
| .match()             | 🙂 Not Tested |                                                                                                                    |
| .matchAll()          | 🙂 Not Tested |                                                                                                                    |
| .normalize()         | 🙂 Not Tested |                                                                                                                    |
| .padEnd()            | 🙂 Not Tested |                                                                                                                    |
| .padStart()          | 🙂 Not Tested |                                                                                                                    |
| .repeat()            | 🙂 Not Tested |                                                                                                                    |
| .replace()           | 🙂 Not Tested |                                                                                                                    |
| .replaceAll()        | 🙂 Not Tested |                                                                                                                    |
| .search()            | 🙂 Not Tested |                                                                                                                    |
| .slice()             | 🙂 Not Tested |                                                                                                                    |
| ⚠️ .small()          | 🙂 Not Tested | ⚠️ [Deprecated](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/small)     |
| .split()             | 🙂 Not Tested |                                                                                                                    |
| .startsWith()        | 🙂 Not Tested |                                                                                                                    |
| ⚠️ .strike()         | 🙂 Not Tested | ⚠️ [Deprecated](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/strike)    |
| ⚠️ .sub()            | 🙂 Not Tested | ⚠️ [Deprecated](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/sub)       |
| ⚠️ .substr()         | 🙂 Not Tested | ⚠️ [Deprecated](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/substr)    |
| .substring()         | 🙂 Not Tested |                                                                                                                    |
| ⚠️ .sup()            | 🙂 Not Tested | ⚠️ [Deprecated](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/sup)       |
| \[Symbol.iterator]() | 🙂 Not Tested |                                                                                                                    |
| .toLocaleLowerCase() | 🙂 Not Tested |                                                                                                                    |
| .toLocaleUpperCase() | 🙂 Not Tested |                                                                                                                    |
| .toLowerCase()       | 🙂 Not Tested |                                                                                                                    |
| .toString()          | 🙂 Not Tested |                                                                                                                    |
| .toUpperCase()       | 🙂 Not Tested |                                                                                                                    |
| .toWellFormed()      | 🙂 Not Tested |                                                                                                                    |
| .trim()              | 🙂 Not Tested |                                                                                                                    |
| .trimEnd()           | 🙂 Not Tested |                                                                                                                    |
| .trimStart()         | 🙂 Not Tested |                                                                                                                    |
| .valueOf()           | 🙂 Not Tested |                                                                                                                    |

| Instance property | Status        | Notes |
| ----------------- | ------------- | ----- |
| .length           | 🙂 Not Tested |       |

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
> The following APIs are not planned to ever be supported in Cloudstate.

- Function
- FinalizationRegistry
- globalThis
- InternalError ([⚠️ Non-standard](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/InternalError))
- Promise
- Proxy
- WeakRef
