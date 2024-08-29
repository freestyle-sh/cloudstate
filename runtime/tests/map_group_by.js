const inventoryRestock = [
    { name: "bananas", type: "fruit", quantity: 5 },
];
const inventorySufficient = [
    { name: "asparagus", type: "vegetables", quantity: 9 },
    { name: "goat", type: "meat", quantity: 23 },
    { name: "cherries", type: "fruit", quantity: 12 },
    { name: "fish", type: "meat", quantity: 22 },
];
const inventory = inventoryRestock.concat(inventorySufficient);

{
    const object = {
        value: inventory,
    };

    setRoot("test-root", object);
    commit();
}

{
    const object = getRoot("test-root");

    const restock = { restock: true };
    const sufficient = { restock: false };

    const result = Map.groupBy(
        object.value,
        ({ quantity }) => quantity < 6 ? restock : sufficient,
    );
    if (result.size !== 2) {
        throw new Error(`Expected 2 groups, got ${result.size}`);
    }
    if (result.get(restock).length !== 1) {
        throw new Error(
            `Expected 1 item in restock group, got ${
                result.get(restock).length
            }`,
        );
    }
    if (result.get(sufficient).length !== 4) {
        throw new Error(
            `Expected 4 items in sufficient group, got ${
                result.get(sufficient).length
            }`,
        );
    }
    for (const item of result.get(restock)) {
        if (!inventoryRestock.some(({ name }) => name === item.name)) {
            throw new Error(`Expected ${JSON.stringify(item)} to exist`);
        }
        const orginalItem = inventoryRestock.find(({ name }) =>
            name === item.name
        );
        if (orginalItem.quantity !== item.quantity) {
            throw new Error(
                `Expected ${JSON.stringify(orginalItem.quantity)}, got ${
                    JSON.stringify(item.quantity)
                }`,
            );
        }
        if (orginalItem.type !== item.type) {
            throw new Error(
                `Expected ${JSON.stringify(orginalItem.type)}, got ${
                    JSON.stringify(item.type)
                }`,
            );
        }
    }
    for (const item of result.get(sufficient)) {
        if (!inventorySufficient.some(({ name }) => name === item.name)) {
            throw new Error(`Expected ${JSON.stringify(item)} to exist`);
        }
        const orginalItem = inventorySufficient.find(({ name }) =>
            name === item.name
        );
        if (orginalItem.quantity !== item.quantity) {
            throw new Error(
                `Expected ${JSON.stringify(orginalItem.quantity)}, got ${
                    JSON.stringify(item.quantity)
                }`,
            );
        }
        if (orginalItem.type !== item.type) {
            throw new Error(
                `Expected ${JSON.stringify(orginalItem.type)}, got ${
                    JSON.stringify(item.type)
                }`,
            );
        }
    }
}
