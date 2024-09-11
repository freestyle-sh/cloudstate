{
  class TodoItem {
    constructor(title, completed) {
      this.title = title;
      this.completed = completed;
    }
    toggleCompletion() {
      this.completed = !this.completed;
    }
  }
  class TodoListCS {
    constructor() {
      this.items = new Map();
    }
    addItem(title) {
      const item = new TodoItem(title, false);
      this.items.set(item.title, item);
      return item;
    }
    getItemsKeys() {
      return Array.from(this.items.keys());
    }
    getItems() {
      return Array.from(this.items.values());
    }
  }
  registerCustomClass(TodoListCS);
  registerCustomClass(TodoItem);

  const root = {
    value: new TodoListCS(),
  };
  setRoot("test-root", root);
  commit();
}

// END_FILE

{
  class TodoItem {
    constructor(title, completed) {
      this.title = title;
      this.completed = completed;
    }
    toggleCompletion() {
      this.completed = !this.completed;
    }
  }
  class TodoListCS {
    constructor() {
      this.items = new Map();
    }
    addItem(title) {
      const item = new TodoItem(title, false);
      this.items.set(item.title, item);
      return item;
    }
    getItemsKeys() {
      return Array.from(this.items.keys());
    }
    getItems() {
      return Array.from(this.items.values());
    }
  }
  registerCustomClass(TodoListCS);
  registerCustomClass(TodoItem);

  const root = getRoot("test-root");

  const item = root.value.addItem("First item");
  if (item.title !== "First item") {
    throw new Error(`Expected title to be "First item", got ${item.title}`);
  }

  console.log("ROUND 2");
  commit();
}

// END_FILE

{
  class TodoItem {
    constructor(title, completed) {
      this.title = title;
      this.completed = completed;
    }
    toggleCompletion() {
      this.completed = !this.completed;
    }
  }
  class TodoListCS {
    constructor() {
      this.items = new Map();
    }
    addItem(title) {
      const item = new TodoItem(title, false);
      this.items.set(item.title, item);
      return item;
    }
    getItemsKeys() {
      return Array.from(this.items.keys());
    }
    getItems() {
      return Array.from(this.items.values());
    }
  }
  registerCustomClass(TodoListCS);
  registerCustomClass(TodoItem);

  console.log("ROUND 3");
  const root = getRoot("test-root");
  console.log("got root");
  const items = root.value.getItems();
  if (items.length !== 1) {
    throw new Error(`Expected items length to be 1, got ${items.length}`);
  }
  if (items[0].title !== "First item") {
    throw new Error(`Expected title to be "First item", got ${items[0].title}`);
  }

  commit();
}
