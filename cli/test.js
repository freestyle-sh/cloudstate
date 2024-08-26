export class TestCS {
  static id = "test";
  count = 0;

  increment() {
    this.count += 1;
    console.log(this.count);
  }
}
