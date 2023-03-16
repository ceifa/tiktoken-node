const tiktoken = require('../index.js')

let enc = tiktoken.getEncoding("gpt2")
console.assert(enc.decode(enc.encode("hello world")) == "hello world")

enc = tiktoken.encodingForModel("text-davinci-003")