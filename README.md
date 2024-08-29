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

## Building Locally

- Check out the guide for [building locally](LOCAL.md) to get started.

## Support for JavaScript Objects

- Check out the [JavaScript Objects](OBJECTS.md) guide to learn more about the objects we support.

> [!NOTE]
> We currently support _most_ of the core JavaScript objects, with active development on all object constructors, methods, and property getters marked as "🚧 Planned".
> [!TIP]
> <<<<<<< HEAD
> Tests are essential to building a robust and reliable runtime. If you'd like to contribute in a small but meaningful way, **please consider writing tests** for the methods and property getters marked as "🙂 Not Tested".
