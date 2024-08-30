const base_array = [1];

{
  const object = {
    value: [...base_array],
  };

  setRoot("test-root", object);
  commit();
}
{
  const root = getRoot("test-root");

  if (!root) {
    throw new Error("root should exist");
  }

  if (!root.value) {
    throw new Error("root.value should exist");
  }

  if (root.value.length !== base_array.length) {
    throw new Error(`root.value should have length ${base_array.length}`);
  }

  if (!root.value.every((value, index) => value === base_array[index])) {
    throw new Error("root.value should be equal to base_array");
  }

  // reverse
  root.value.sort((a, b) => b - a);

  const reversed_array = [...base_array].reverse();

  if (!root.value.every((value, index) => value === reversed_array[index])) {
    throw new Error("root.value should be equal to reversed_array");
  }

  commit();
}

{
  // testing undefined sort function
  const root = getRoot("test-root");

  if (!root) {
    throw new Error("root should exist");
  }

  if (!root.value) {
    throw new Error("root.value should exist");
  }

  if (root.value.length !== base_array.length) {
    throw new Error(`root.value should have length ${base_array.length}`);
  }

  root.value.sort();

  if (!root.value.every((value, index) => value === base_array[index])) {
    throw new Error("root.value should be equal to base_array");
  }
}
