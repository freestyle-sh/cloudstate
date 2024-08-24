export class TodoListCS {
  static id = "todo-list";

  items = new Map();

  addItem(text) {
    const item = new TodoItemCS(text);
    this.items.set(item.id, item);

    return item.info();
  }

  getItems() {
    return Array.from(this.items.values())
      .map((item) => item.info())
      .toReversed();
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
