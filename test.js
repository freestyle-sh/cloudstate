// console.log(new Proxy({}) instanceof Array);
const proxy = new Proxy(
  {},
  {
    get: (target, key) => {
      console.log("get", key);
      return target[key];
    },
    set: (target, key, value) => {
      console.log("set", key, value);
      target[key] = value;
      return true;
    },
  }
);

Object.setPrototypeOf(proxy, Array.prototype);

console.log(proxy instanceof Array);

proxy.push(1);
console.log(proxy);
