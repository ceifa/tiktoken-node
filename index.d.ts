export function getEncoding(encoding: string): Encoding
export function encodingForModel(modelName: string): Encoding
export class Encoding {
  encode(text: string): Array<number>
  decode(tokens: Array<number>): string
}
