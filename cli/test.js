export class TestCS {
  static id = "test";
  count = 0;

  increment() {
    this.count += 2;
    console.log(this.count);
    return this.count;
  }

  fetch() {
    return new Response(
      JSON.stringify({
        count: this.count,
        message: "Ben was here",
      }),
      {
        headers: {
          "content-type": "application/json",
        },
      }
    );
  }
}

export class TodoListCS {
  static id = "todo-list";

  items = [];

  addItem(text) {
    const item = new TodoItemCS(text);
    // this.items.set(item.id, item);
    this.items.push(item);

    return item.info();
  }

  getItems() {
    // return Array.from(this.items.values())
    //   .map((item) => item.info())
    //   .toReversed();
    return this.items.map((item) => item.info());
  }
}

export class TodoItemCS {
  id = crypto.randomUUID();
  completed = false;

  constructor(text) {
    this.text = text;
  }

  info() {
    return {
      id: this.id,
      text: this.text,
      completed: this.completed,
    };
  }

  toggleCompletion() {
    this.completed = !this.completed;
    return {
      completed: this.completed,
    };
  }
}
