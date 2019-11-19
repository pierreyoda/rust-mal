import { createContext } from "react";

export const malInterpreterOutputTypes = ["empty", "error", "result"] as const;
export type MalInterpreterOutputType = typeof malInterpreterOutputTypes[number];

/**
 * Descripes the possible outputs of the MAL Interpreter
 * for a given raw input.
 */
export type MalInterpreterRepOutput =
  | MalInterpreterRepEmptyOutput
  | MalInterpreterRepErrorOutput
  | MalInterpreterRepResultOutput;

export interface MalInterpreterRepEmptyOutput {
  type: "empty";
}

export interface MalInterpreterRepErrorOutput {
  type: "error";
  text: string;
}

export interface MalInterpreterRepResultOutput {
  type: "result";
  text: string;
}

export interface MalInterpreterVm {
  /**
   * Basic version information from the implementing library.
   */
  version(): string;

  /**
   * Read-Eval-Print the given input.
   */
  rep(input: string): Promise<MalInterpreterRepOutput>;
}

const InterpreterContext = createContext<MalInterpreterVm | null>(null);

export default InterpreterContext;
