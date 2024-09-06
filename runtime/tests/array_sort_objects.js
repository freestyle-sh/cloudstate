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

  // reverse
  root.value.sort((a, b) => b.value - a.value);

  const reversed_array = [...base_array].reverse();

  // check to make sure the sort worked in place
  for (let i = 0; i < root.value.length; i++) {
    if (root.value[i].value !== reversed_array[i].value) {
      throw new Error(
        `root.value[${i}].value should be equal to reversed_array[${i}].value, ${root.value[i].value} !== ${reversed_array[i].value} | ${root.value[0].value}`
      );
    }
  }

  commit();
}

{
  const root = getRoot("test-root");

  const reversed_array = [...base_array].reverse();

  // check to make sure the sort was saved
  // TODO: this is now cached so we need to validate that it's actually saved
  for (let i = 0; i < root.value.length; i++) {
    if (root.value[i].value !== reversed_array[i].value) {
      throw new Error(
        `root.value[${i}].value should be equal to reversed_array[${i}].value, ${root.value[i].value} !== ${reversed_array[i].value} | ${root.value[0].value}`
      );
    }
  }
}
