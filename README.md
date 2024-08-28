# cloudstate
- Node.js enables developers to create servers in JavaScript.
- Cloudstate enables developers to create databases in JavaScript.

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
