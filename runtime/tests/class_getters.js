{
    class CounterCS {
        _count = 0;

        get count() {
            return this._count;
        }
    }
    registerCustomClass(CounterCS);

    setRoot("test-root", new CounterCS());
}

// END_FILE

{
    class CounterCS {
        _count = 0;

        get count() {
            return this._count;
        }
    }
    registerCustomClass(CounterCS);

    const object = getRoot("test-root");

    commit();
}
