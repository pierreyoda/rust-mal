import React from "react";
import { NextPage } from "next";

import ReplConsole from "@/components/repl/ReplConsole";
import InterpreterContext, { MalInterpreterVm, MalInterpreterRepOutput } from "@/components/vm/InterpreterContext";

const mockVm: MalInterpreterVm = {
  version() {
    return "[front-end mock]";
  },
  rep(input) {
    const trimmed = input.trim();
    const result: MalInterpreterRepOutput = !trimmed
      ? { type: "empty" }
      : input.startsWith("(") && input.endsWith(")")
        ? { type: "result", text: `(mock) > ${input}` }
        : { type: "error", text: "error: does not look like Lisp" };
    const mockedDelayMs = 100 + Math.random() * 300;
    return new Promise(resolve => setTimeout(() => resolve(result), mockedDelayMs));
  },
};

const testConsoleInitialPrompts = [
  `rust-mal v.${mockVm.version()}`,
  "second prompt line",
];

const Home: NextPage = () => (
  <div className="w-full h-full bg-gray-500 flex flex-col items-stretch">
    <h1 className="text-4xl text-orange-300 text-center">
      rust-mal Web REPL
    </h1>
    <div className="flex-grow">
      <InterpreterContext.Provider value={mockVm}>
        <ReplConsole initialPrompt={testConsoleInitialPrompts} />
      </InterpreterContext.Provider>
    </div>
  </div>
);

// TODO: fix Rust setup
// Home.getInitialProps = async () => ({
//   vm: mockVm,
// });

export default Home;
