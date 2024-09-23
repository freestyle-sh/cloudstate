export class TestCS {
  static id = "counter";
  count = 0;

  increment() {
    return ++this.count;
  }
}
