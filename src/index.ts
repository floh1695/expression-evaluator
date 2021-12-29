import * as patterns from './patterns';

export const tokenize = (raw: string): string[] => {
  const pattern = new RegExp(patterns.token, 'g');
  const groups = Array.from(raw.matchAll(pattern));
  const tokens = groups.map(group => group[0]);

  return tokens;
};

export const main = (instructions: string[]) => {
  const tokenized = instructions.map(tokenize);

  tokenized.forEach(tokens => console.log(tokens));
};

const instructions = process.argv.slice(2);
if (instructions.length > 0) {
  main(instructions);
}
