# 【JavaScript】Proxy 实例方法（二）

> 以下内容为学习记录，可以参考 [MDN][1] 原文。

## 环境

- node v12.18.1
- npm 6.14.5
- vscode 1.46
- Microsoft Edge 83

## 概念

Proxy 对象用于定义基本操作的自定义行为（如属性查找、赋值、枚举、函数调用等）。

## has

handler.has() 方法是针对 in 操作符的代理方法。

```js
const handler1 = {
  has(target, key) {
    if (key[0] === '_') {
      return false;
    }
    return key in target;
  }
};

const monster1 = {
  _secret: 'easily scared',
  eyeCount: 4
};

const proxy1 = new Proxy(monster1, handler1);
console.log('eyeCount' in proxy1);
// expected output: true

console.log('_secret' in proxy1);
// expected output: false

console.log('_secret' in monster1);
// expected output: true
```

## isExtensible

handler.isExtensible() 方法用于拦截对对象的 Object.isExtensible()。

```js
const monster1 = {
  canEvolve: true
};

const handler1 = {
  isExtensible(target) {
    return Reflect.isExtensible(target);
  },
  preventExtensions(target) {
    target.canEvolve = false;
    return Reflect.preventExtensions(target);
  }
};

const proxy1 = new Proxy(monster1, handler1);

console.log(Object.isExtensible(proxy1));
// expected output: true

console.log(monster1.canEvolve);
// expected output: true

Object.preventExtensions(proxy1);

console.log(Object.isExtensible(proxy1));
// expected output: false

console.log(monster1.canEvolve);
// expected output: false
```

## ownKeys

handler.ownKeys() 方法用于拦截 Reflect.ownKeys().

```js
const monster1 = {
  _age: 111,
  [Symbol('secret')]: 'I am scared!',
  eyeCount: 4
};

const handler1 = {
  ownKeys(target) {
    return Reflect.ownKeys(target);
  }
};

const proxy1 = new Proxy(monster1, handler1);

for (let key of Object.keys(proxy1)) {
  console.log(key);
  // expected output: "_age"
  // expected output: "eyeCount"
}
```

## preventExtensions

handler.preventExtensions() 方法用于设置对 Object.preventExtensions() 的拦截。

```js
const monster1 = {
  canEvolve: true
};

const handler1 = {
  preventExtensions(target) {
    target.canEvolve = false;
    Object.preventExtensions(target);
    return true;
  }
};

const proxy1 = new Proxy(monster1, handler1);

console.log(monster1.canEvolve);
// expected output: true

Object.preventExtensions(proxy1);

console.log(monster1.canEvolve);
// expected output: false
```

## set

handler.set() 方法是设置属性值操作的捕获器。

```js
function Monster() {
  this.eyeCount = 4;
}

const handler1 = {
  set(obj, prop, value) {
    if ((prop === 'eyeCount') && ((value % 2) !== 0)) {
      console.log('Monsters must have an even number of eyes');
    } else {
      return Reflect.set(...arguments);
    }
  }
};

const monster1 = new Monster();
const proxy1 = new Proxy(monster1, handler1);
proxy1.eyeCount = 1;
// expected output: "Monsters must have an even number of eyes"

console.log(proxy1.eyeCount);
// expected output: 4
```

### setPrototypeOf

handler.setPrototypeOf() 方法主要用来拦截 Object.setPrototypeOf()。

```js
const handler1 = {
  setPrototypeOf(monster1, monsterProto) {
    monster1.geneticallyModified = true;
    return false;
  }
};

const monsterProto = {};
const monster1 = {
  geneticallyModified: false
};

const proxy1 = new Proxy(monster1, handler1);
// Object.setPrototypeOf(proxy1, monsterProto); // throws a TypeError

console.log(Reflect.setPrototypeOf(proxy1, monsterProto));
// expected output: false

console.log(monster1.geneticallyModified);
// expected output: true
```

[1]: https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Global_Objects/Proxy
