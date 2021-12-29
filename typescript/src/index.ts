import * as patterns from './patterns';

export const tokenize = (raw: string): string[] => {
  const pattern = new RegExp(patterns.token, 'g');
  const groups = Array.from(raw.matchAll(pattern));
  const tokens = groups.map(group => group[0]);

  return tokens;
};

export const main = (expressions: string[]) => {
  const tokenized = expressions.map(tokenize);

  tokenized.forEach(tokens => console.log(tokens));
};

const expressions = process.argv.slice(2);
if (expressions.length > 0) {
  main(expressions);
}
