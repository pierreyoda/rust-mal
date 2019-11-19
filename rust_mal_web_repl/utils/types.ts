/**
 * Same as standard `Omit` construct but with a constraint on the keys,
 * enabling auto-completion.
 */
export type Omit<T, K extends keyof T> = {
  [P in Exclude<keyof T, K>]: T[P];
};

export type PartiallyRequired<T, K extends keyof T> = Omit<T, K> & Required<Pick<T, K>>;

export const valueIsDefined = <T>(value: T | null | undefined): value is T =>
  value !== null && value !== undefined;
