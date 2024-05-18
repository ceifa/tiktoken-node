export function getEncoding(encoding: 'gpt2' | 'r50k_base' | 'p50k_base' | 'p50k_edit' | 'cl100k_base' | 'o200k_base'): Encoding
export function encodingForModel(modelName: string): Encoding
export class Encoding {
  encode(text: string): Array<number>
  decode(tokens: Array<number>): string
  encodeBatch(texts: Array<string>): Array<Array<number>>
}
