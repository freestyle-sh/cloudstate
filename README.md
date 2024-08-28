![cloudstate banner](https://github.com/user-attachments/assets/c580008f-98da-47d7-9a82-7abf423a426b)
<p align="center">
 <a href="https://docs.freestyle.dev/getting-started/intro">Get Started</a> · <a href="https://docs.freestyle.dev">Docs</a> · <a href="https://discord.gg/YTRprVkdnz">Discord</a>
</p>
<p align="center">
<b>Cloudstate</b> is a combined JavaScript Runtime and Database Engine that allows JavaScript developers to persist their data with just JavaScript.
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
