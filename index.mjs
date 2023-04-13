import { createRequire } from 'node:module';
const { platform, arch } = process

const require = createRequire(import.meta.url);

let nativeBinding = undefined

if (platform === 'win32' && arch === 'x64') {
    nativeBinding = require('./dist/tiktoken-node.win32-x64-msvc.node')
} else if (platform === 'linux') {
    if (arch === 'x64') {
        nativeBinding = require('./dist/tiktoken-node.linux-x64-gnu.node')
    } else if (arch === 'arm64') {
        nativeBinding = require('./dist/tiktoken-node.linux-arm64-gnu.node')
    }
} else if (platform === 'darwin') {
    if (arch === 'x64') {
        nativeBinding = require('./dist/tiktoken-node.darwin-x64.node')
    } else if (arch === 'arm64') {
        nativeBinding = require('./dist/tiktoken-node.darwin-arm64.node')
    }
} else {
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
}

export default nativeBinding
