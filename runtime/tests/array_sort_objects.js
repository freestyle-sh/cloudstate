const base_array = [
  {
    value: 1,
  },
  {
    value: 2,
  },
  {
    value: 3,
  },
  {
    value: 4,
  },
  {
    value: 5,
  },
  {
    value: 6,
  },
  {
    value: 7,
  },
  {
    value: 8,
  },
  {
    value: 9,
  },
  {
    value: 10,
  },
];

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

  if (
    !root.value.every((value, index) => value.value === base_array[index].value)
  ) {
    throw new Error("root.value should be equal to base_array");
  }

  // reverse
  root.value.sort((a, b) => b.value - a.value);

  const reversed_array = [...base_array].sort((a, b) => b.value - a.value);

  for (let i = 0; i < root.value.length; i++) {
    if (root.value[i].value !== reversed_array[i].value) {
      console.log("0 INDEX IS", root.value[0].value);

      throw new Error(
        `root.value[${i}].value should be equal to reversed_array[${i}].value, ${root.value[i].value} !== ${reversed_array[i].value} | ${root.value[0].value}`
      );
    }
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

  if (
    !root.value.every((value, index) => value.value === base_array[index].value)
  ) {
    throw new Error("root.value should be equal to base_array");
  }

  commit();
}
