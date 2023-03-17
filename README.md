# ⏳ tiktoken-node

tiktoken is a fast [BPE](https://en.wikipedia.org/wiki/Byte_pair_encoding) tokeniser for use with
OpenAI's models.

```js
const tiktoken = require('tiktoken-node')
let enc = tiktoken.getEncoding("gpt2")
console.assert(enc.decode(enc.encode("hello world")) == "hello world")

// To get the tokeniser corresponding to a specific model in the OpenAI API:
enc = tiktoken.encodingForModel("text-davinci-003")
```

The open source version of `tiktoken-node` can be installed from npm:
```
npm install tiktoken-node
```