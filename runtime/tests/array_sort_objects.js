{
  const base = [
    {
      value: 5,
    },
    {
      value: 2,
    },
    {
      value: 4,
    },
    {
      value: 1,
    },
    {
      value: 3,
    },
    {
      value: 6,
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
    {
      value: 7,
    },
  ];
  const object = {
    value: [...base],
  };

  setRoot("test-root", object);
  commit();
}

// END_FILE

{
  const sortFn = (a, b) => (b.value - a.value);
  const base = [
    {
      value: 5,
    },
    {
      value: 2,
    },
    {
      value: 4,
    },
    {
      value: 1,
    },
    {
      value: 3,
    },
    {
      value: 6,
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
    {
      value: 7,
    },
  ];
  const expectedArr = [...base].sort(sortFn);

  // reverse root.value
  const root = getRoot("test-root");
  root.value.sort(sortFn);

  // check to make sure the sort worked in place (in memory)
  for (let i = 0; i < root.value.length; i++) {
    if (root.value[i].value !== expectedArr[i].value) {
      throw new Error(
        `root.value[${i}].value should be equal to expectedArr[${i}].value, ${
          root.value[i].value
        } !== ${expectedArr[i].value}`,
      );
    }
  }

  commit();
}

// END_FILE

{
  const sortFn = (a, b) => (b.value - a.value);
  const base = [
    {
      value: 5,
    },
    {
      value: 2,
    },
    {
      value: 4,
    },
    {
      value: 1,
    },
    {
      value: 3,
    },
    {
      value: 6,
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
    {
      value: 7,
    },
  ];
  const expectedArr = [...base].sort(sortFn);

  // get reversed base array from root
  const root = getRoot("test-root");

  // check to make sure the sort was saved
  for (let i = 0; i < root.value.length; i++) {
    if (root.value[i].value !== expectedArr[i].value) {
      throw new Error(
        `root.value[${i}].value should be equal to expectedArr[${i}].value, ${
          root.value[i].value
        } !== ${expectedArr[i].value}`,
      );
    }
  }
}
