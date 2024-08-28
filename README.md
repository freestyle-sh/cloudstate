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

## Support for JavaScript APIs

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

### Boolean

| Constructor | Status     | Notes |
| ----------- | ---------- | ----- |
| Boolean()   | 🚧 Planned |       |

| Instance method | Status     | Notes |
| --------------- | ---------- | ----- |
| .toString()     | 🚧 Planned |       |
| .valueOf()      | 🚧 Planned |       |

### DataView

🚧 Planned

### Date

| Constructor | Status           | Notes |
| ----------- | ---------------- | ----- |
| Date()      | ❌ Not Supported |       |

| Static method | Status           | Notes |
| ------------- | ---------------- | ----- |
| Date.now()    | ❌ Not Supported |       |
| Date.parse()  | ❌ Not Supported |       |
| Date.UTC()    | ❌ Not Supported |       |

| Instance method         | Status           | Notes |
| ----------------------- | ---------------- | ----- |
| .getDate()              | ❌ Not Supported |       |
| .getDay()               | ❌ Not Supported |       |
| .getFullYear()          | ❌ Not Supported |       |
| .getHours()             | ❌ Not Supported |       |
| .getMilliseconds()      | ❌ Not Supported |       |
| .getMinutes()           | ❌ Not Supported |       |
| .getMonth()             | ❌ Not Supported |       |
| .getSeconds()           | ❌ Not Supported |       |
| .getTime()              | ❌ Not Supported |       |
| .getTimezoneOffset()    | ❌ Not Supported |       |
| .getUTCDate()           | ❌ Not Supported |       |
| .getUTCDay()            | ❌ Not Supported |       |
| .getUTCFullYear()       | ❌ Not Supported |       |
| .getUTCHours()          | ❌ Not Supported |       |
| .getUTCMilliseconds()   | ❌ Not Supported |       |
| .getUTCMinutes()        | ❌ Not Supported |       |
| .getUTCMonth()          | ❌ Not Supported |       |
| .getUTCSeconds()        | ❌ Not Supported |       |
| ⚠️ .getYear()           | ❌ Not Supported |       |
| .setDate()              | ❌ Not Supported |       |
| .setFullYear()          | ❌ Not Supported |       |
| .setHours()             | ❌ Not Supported |       |
| .setMilliseconds()      | ❌ Not Supported |       |
| .setMinutes()           | ❌ Not Supported |       |
| .setMonth()             | ❌ Not Supported |       |
| .setSeconds()           | ❌ Not Supported |       |
| .setTime()              | ❌ Not Supported |       |
| .setUTCDate()           | ❌ Not Supported |       |
| .setUTCFullYear()       | ❌ Not Supported |       |
| .setUTCHours()          | ❌ Not Supported |       |
| .setUTCMilliseconds()   | ❌ Not Supported |       |
| .setUTCMinutes()        | ❌ Not Supported |       |
| .setUTCMonth()          | ❌ Not Supported |       |
| .setUTCSeconds()        | ❌ Not Supported |       |
| ⚠️ .setYear()           | ❌ Not Supported |       |
| \[Symbol.toPrimitive]() | ❌ Not Supported |       |
| .toDateString()         | ❌ Not Supported |       |
| .toISOString()          | ❌ Not Supported |       |
| .toJSON()               | ❌ Not Supported |       |
| .toLocaleDateString()   | ❌ Not Supported |       |
| .toLocaleString()       | ❌ Not Supported |       |
| .toLocaleTimeString()   | ❌ Not Supported |       |
| .toTimeString()         | ❌ Not Supported |       |
| .toUTCString()          | ❌ Not Supported |       |
| .valueOf()              | ❌ Not Supported |       |

### Error?

❌ Not Supported

### Map

| Constructor | Status           | Notes |
| ----------- | ---------------- | ----- |
| Map()       | ❌ Not Supported |       |

| Static method | Status           | Notes |
| ------------- | ---------------- | ----- |
| Map.groupBy() | ❌ Not Supported |       |

| Instance method      | Status           | Notes |
| -------------------- | ---------------- | ----- |
| .clear()             | ❌ Not Supported |       |
| .delete()            | ❌ Not Supported |       |
| .entries()           | ❌ Not Supported |       |
| .forEach()           | ❌ Not Supported |       |
| .get()               | ❌ Not Supported |       |
| .has()               | ❌ Not Supported |       |
| .keys()              | ❌ Not Supported |       |
| .set()               | ❌ Not Supported |       |
| \[Symbol.iterator]() | ❌ Not Supported |       |
| .values()            | ❌ Not Supported |       |

| Instance property | Status           | Notes |
| ----------------- | ---------------- | ----- |
| .size             | ❌ Not Supported |       |

### Number

| Constructor | Status           | Notes |
| ----------- | ---------------- | ----- |
| Number()    | ❌ Not Supported |       |

| Static method          | Status           | Notes |
| ---------------------- | ---------------- | ----- |
| Number.isFinite()      | ❌ Not Supported |       |
| Number.isInteger()     | ❌ Not Supported |       |
| Number.isNaN()         | ❌ Not Supported |       |
| Number.isSafeInteger() | ❌ Not Supported |       |
| Number.parseFloat()    | ❌ Not Supported |       |
| Number.parseInt()      | ❌ Not Supported |       |

| Instance method   | Status           | Notes |
| ----------------- | ---------------- | ----- |
| .toExponential()  | ❌ Not Supported |       |
| .toFixed()        | ❌ Not Supported |       |
| .toLocaleString() | ❌ Not Supported |       |
| .toPrecision()    | ❌ Not Supported |       |
| .toString()       | ❌ Not Supported |       |
| .valueOf()        | ❌ Not Supported |       |

### Object

| Constructor                         | Status           | Notes |
| ----------------------------------- | ---------------- | ----- |
| Object()                            | ❌ Not Supported |       |
| Object initializer / literal syntax | ❌ Not Supported |       |

| Static method                      | Status           | Notes |
| ---------------------------------- | ---------------- | ----- |
| Object.assign()                    | ❌ Not Supported |       |
| Object.create()                    | ❌ Not Supported |       |
| Object.defineProperties()          | ❌ Not Supported |       |
| Object.defineProperty()            | ❌ Not Supported |       |
| Object.entries()                   | ❌ Not Supported |       |
| Object.freeze()                    | ❌ Not Supported |       |
| Object.fromEntries()               | ❌ Not Supported |       |
| Object.getOwnPropertyDescriptor()  | ❌ Not Supported |       |
| Object.getOwnPropertyDescriptors() | ❌ Not Supported |       |
| Object.getOwnPropertyNames()       | ❌ Not Supported |       |
| Object.getOwnPropertySymbols()     | ❌ Not Supported |       |
| Object.getPrototypeOf()            | ❌ Not Supported |       |
| Object.groupBy()                   | ❌ Not Supported |       |
| Object.hasOwn()                    | ❌ Not Supported |       |
| Object.is()                        | ❌ Not Supported |       |
| Object.isExtensible()              | ❌ Not Supported |       |
| Object.isFrozen()                  | ❌ Not Supported |       |
| Object.isSealed()                  | ❌ Not Supported |       |
| Object.keys()                      | ❌ Not Supported |       |
| Object.preventExtensions()         | ❌ Not Supported |       |
| Object.seal()                      | ❌ Not Supported |       |
| Object.setPrototypeOf()            | ❌ Not Supported |       |
| Object.values()                    | ❌ Not Supported |       |

| Instance method         | Status           | Notes |
| ----------------------- | ---------------- | ----- |
| ⚠️ .**defineGetter**()  | ❌ Not Supported |       |
| ⚠️ .**defineSetter**()  | ❌ Not Supported |       |
| ⚠️ .**lookupGetter**()  | ❌ Not Supported |       |
| ⚠️ .**lookupSetter**()  | ❌ Not Supported |       |
| .hasOwnProperty()       | ❌ Not Supported |       |
| .isPrototypeOf()        | ❌ Not Supported |       |
| .propertyIsEnumerable() | ❌ Not Supported |       |
| .toLocaleString()       | ❌ Not Supported |       |
| .toString()             | ❌ Not Supported |       |
| .valueOf()              | ❌ Not Supported |       |

| Instance property | Status           | Notes |
| ----------------- | ---------------- | ----- |
| .constructor      | ❌ Not Supported |       |
| ⚠️ .**proto**     | ❌ Not Supported |       |

### RegExp

| Constructor | Status           | Notes |
| ----------- | ---------------- | ----- |
| RegExp()    | ❌ Not Supported |       |

| Instance method      | Status           | Notes |
| -------------------- | ---------------- | ----- |
| ⚠️ .compile()        | ❌ Not Supported |       |
| .exec()              | ❌ Not Supported |       |
| \[Symbol.match]()    | ❌ Not Supported |       |
| \[Symbol.matchAll]() | ❌ Not Supported |       |
| \[Symbol.replace]()  | ❌ Not Supported |       |
| \[Symbol.search]()   | ❌ Not Supported |       |
| \[Symbol.split]()    | ❌ Not Supported |       |
| .test()              | ❌ Not Supported |       |
| .toString()          | ❌ Not Supported |       |

| Instance property | Status           | Notes |
| ----------------- | ---------------- | ----- |
| .dotAll           | ❌ Not Supported |       |
| .flags            | ❌ Not Supported |       |
| .global           | ❌ Not Supported |       |
| .hasIndices       | ❌ Not Supported |       |
| .ignoreCase       | ❌ Not Supported |       |
| .lastIndex        | ❌ Not Supported |       |
| .multiline        | ❌ Not Supported |       |
| .source           | ❌ Not Supported |       |
| .sticky           | ❌ Not Supported |       |
| .unicode          | ❌ Not Supported |       |
| .unicodeSets      | ❌ Not Supported |       |

### Set

| Constructor | Status           | Notes |
| ----------- | ---------------- | ----- |
| Set()       | ❌ Not Supported |       |

| Instance method        | Status           | Notes |
| ---------------------- | ---------------- | ----- |
| .add()                 | ❌ Not Supported |       |
| .clear()               | ❌ Not Supported |       |
| .delete()              | ❌ Not Supported |       |
| .difference()          | ❌ Not Supported |       |
| .entries()             | ❌ Not Supported |       |
| .forEach()             | ❌ Not Supported |       |
| .has()                 | ❌ Not Supported |       |
| .intersection()        | ❌ Not Supported |       |
| .isDisjointFrom()      | ❌ Not Supported |       |
| .isSubsetOf()          | ❌ Not Supported |       |
| .isSupersetOf()        | ❌ Not Supported |       |
| .keys()                | ❌ Not Supported |       |
| \[Symbol.iterator]()   | ❌ Not Supported |       |
| .symmetricDifference() | ❌ Not Supported |       |
| .union()               | ❌ Not Supported |       |
| .values()              | ❌ Not Supported |       |

| Instance property | Status           | Notes |
| ----------------- | ---------------- | ----- |
| .size             | ❌ Not Supported |       |

### String

| Constructor | Status           | Notes |
| ----------- | ---------------- | ----- |
| String()    | ❌ Not Supported |       |

| Static method          | Status           | Notes |
| ---------------------- | ---------------- | ----- |
| String.fromCharCode()  | ❌ Not Supported |       |
| String.fromCodePoint() | ❌ Not Supported |
| String.raw()           | ❌ Not Supported |       |

| Instance method      | Status           | Notes |
| -------------------- | ---------------- | ----- |
| ⚠️ .anchor()         | ❌ Not Supported |       |
| .at()                | ❌ Not Supported |       |
| ⚠️ .big()            | ❌ Not Supported |       |
| ⚠️ .blink()          | ❌ Not Supported |       |
| ⚠️ .bold()           | ❌ Not Supported |       |
| .charAt()            | ❌ Not Supported |       |
| .charCodeAt()        | ❌ Not Supported |       |
| .codePointAt()       | ❌ Not Supported |       |
| .concat()            | ❌ Not Supported |       |
| .endsWith()          | ❌ Not Supported |       |
| ⚠️ .fixed()          | ❌ Not Supported |       |
| ⚠️.fontcolor()       | ❌ Not Supported |       |
| ⚠️.fontsize()        | ❌ Not Supported |       |
| .includes()          | ❌ Not Supported |       |
| .indexOf()           | ❌ Not Supported |       |
| .isWellFormed()      | ❌ Not Supported |       |
| ⚠️ .italics()        | ❌ Not Supported |       |
| .lastIndexOf()       | ❌ Not Supported |       |
| ⚠️ .link()           | ❌ Not Supported |       |
| .localeCompare()     | ❌ Not Supported |       |
| .match()             | ❌ Not Supported |       |
| .matchAll()          | ❌ Not Supported |       |
| .normalize()         | ❌ Not Supported |       |
| .padEnd()            | ❌ Not Supported |       |
| .padStart()          | ❌ Not Supported |       |
| .repeat()            | ❌ Not Supported |       |
| .replace()           | ❌ Not Supported |       |
| .replaceAll()        | ❌ Not Supported |       |
| .search()            | ❌ Not Supported |       |
| .slice()             | ❌ Not Supported |       |
| ⚠️ .small()          | ❌ Not Supported |       |
| .split()             | ❌ Not Supported |       |
| .startsWith()        | ❌ Not Supported |       |
| ⚠️ .strike()         | ❌ Not Supported |       |
| ⚠️ .sub()            | ❌ Not Supported |       |
| ⚠️ .substr()         | ❌ Not Supported |       |
| .substring()         | ❌ Not Supported |       |
| ⚠️ .sup()            | ❌ Not Supported |       |
| \[Symbol.iterator]() | ❌ Not Supported |       |
| .toLocaleLowerCase() | ❌ Not Supported |       |
| .toLocaleUpperCase() | ❌ Not Supported |       |
| .toLowerCase()       | ❌ Not Supported |       |
| .toString()          | ❌ Not Supported |       |
| .toUpperCase()       | ❌ Not Supported |       |
| .toWellFormed()      | ❌ Not Supported |       |
| .trim()              | ❌ Not Supported |       |
| .trimEnd()           | ❌ Not Supported |       |
| .trimStart()         | ❌ Not Supported |       |
| .valueOf()           | ❌ Not Supported |       |

| Instance property | Status           | Notes |
| ----------------- | ---------------- | ----- |
| .length           | ❌ Not Supported |       |

### Symbol

❌ Not Supported

### WeakMap

❌ Not Supported

### WeakSet

❌ Not Supported

## Not planned

- Function
