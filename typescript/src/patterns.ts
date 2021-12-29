const match = (raw: string): string => `(${raw})`;
const matchAny = (patterns: string[]): string => patterns.join('|');

export const number = match('\\d+');
export const symbol = match('\\w+');
export const value = matchAny([
    number,
    symbol
  ]);

export const addition = match('\\+');
export const subtraction = match('\\-');
export const multiplication = match('\\*');
export const division = match('\\/');
export const operator = matchAny([
    addition,
    subtraction,
    multiplication,
    division
  ]);

export const token = matchAny([
    value,
    operator
  ]);
