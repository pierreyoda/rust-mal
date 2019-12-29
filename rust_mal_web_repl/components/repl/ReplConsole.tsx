import React, { FunctionComponent, useReducer, useCallback, useContext, useMemo } from "react";

import { ReplLineOutputDatum, ReplLineDatum, ReplLine } from "./ReplLine";
import InterpreterContext, { MalInterpreterVm } from "../vm/InterpreterContext";
import { valueIsDefined } from "@/utils/types";

export interface ReplConsoleProps {
  initialPrompt: readonly string[];
}

type ReplConsoleLineDatum = ReplLineDatum & { index: number };

interface ReplConsoleState {
  vm: {
    ready: boolean;
    busy: boolean;
  };
  linesHistory: ReplConsoleLineDatum[];
  currentInput: string;
}

const initialState = (initialPrompts: readonly string[]): ReplConsoleState => ({
  vm: {
    ready: false,
    busy: false,
  },
  linesHistory: initialPrompts.map((print, index): ReplConsoleLineDatum => ({
    index,
    print,
    type: "host",
  })),
  currentInput: "",
});

type ReplConsoleReducerAction =
  | { type: "vm-loaded" }
  | { type: "vm-busy"; payload: boolean }
  | { type: "reset"; payload: Parameters<typeof initialState> }
  | { type: "current-prompt-clear" }
  | { type: "current-prompt-input"; payload: { input: string } }
  | { type: "push-current-prompt-output"; payload: ReplLineOutputDatum }
;

const replConsoleReducer = (
  state: ReplConsoleState,
  action: ReplConsoleReducerAction,
): ReplConsoleState => {
  switch (action.type) {
    case "reset": return initialState(...action.payload);
    case "vm-loaded": return { ...state, vm: { ready: true, busy: false } };
    case "vm-busy": return { ...state, vm: { ...state.vm, busy: action.payload } };
    case "current-prompt-clear": return { ...state, currentInput: "" };
    case "current-prompt-input": return { ...state, currentInput: action.payload.input };
    case "push-current-prompt-output": return {
      ...state,
      linesHistory: [...state.linesHistory, {
        type: "input",
        value: state.currentInput,
        index: state.linesHistory.length,
      }, {
        ...action.payload,
        index: state.linesHistory.length + 1,
      }],
      currentInput: "",
    };
    default: return state;
  }
};

const ReplConsole: FunctionComponent<ReplConsoleProps> = ({
  initialPrompt,
}) => {
  const [state, dispatch] = useReducer(replConsoleReducer, initialState(initialPrompt));

  // VM
  const vm: MalInterpreterVm | null = useContext(InterpreterContext);
  const loading = useMemo(() => valueIsDefined(vm), [vm]); // TODO: look into React Suspense

  const onCurrentInputChange = useCallback(
    (input: string) => dispatch({ type: "current-prompt-input", payload: { input } }),
    [],
  );
  const handleCurrentInputSubmit = useCallback(
    (input: string) => {
      if (!valueIsDefined(vm)) {
        throw new Error("ReplConsole: vm from context must not be null or undefined.");
      }
      dispatch({
        type: "vm-busy",
        payload: true,
      });
      vm.rep(input).then(
        result => {
          dispatch({
            type: "push-current-prompt-output",
            payload: { type: result.type, print: result.type !== "empty" ? result.text : "" },
          });
          dispatch({
            type: "vm-busy",
            payload: false,
          });
        },
      );
    },
    [vm],
  );

  return (
    <div className="flex flex-col p-2 bg-gray-600">
      {state.linesHistory.map(({ index, ...datum }) => (
        <ReplLine key={index} {...datum} />
      ))}
      <ReplLine
        type="current-input"
        value={state.currentInput}
        onChange={onCurrentInputChange}
        onSubmit={handleCurrentInputSubmit}
        pending={state.vm.busy}
      />
    </div>
  );
};

export default ReplConsole;
