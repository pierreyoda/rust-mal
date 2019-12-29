import React, {
  useRef,
  useMemo,
  useEffect,
  useCallback,
  FormEventHandler,
  FunctionComponent,
} from "react";

export type ReplLineInputPrompt = () => React.ReactNode;

export type ReplLineType = ReplLineInputType | ReplLineOutputType;

export type ReplLineInputType = "input" | "current-input";

export type ReplLineInputDatum = {
  type: "input";
  value: string;
};

export type ReplLineCurrentInputDatum = {
  type: "current-input";
  value: string;
  pending: boolean;
  onChange: (newValue: string) => void;
  onSubmit: (value: string) => void;
  prompt?: ReplLineInputPrompt;
};

const ReplInputDefaultPrompt: ReplLineInputPrompt = () => (
  <span className="mr-2">&gt;</span>
);

const ReplLineActiveInput: FunctionComponent<ReplLineCurrentInputDatum> = ({
  value,
  onChange,
  onSubmit,
  pending,
  prompt,
}) => {
  const inputRef = useRef<HTMLInputElement>(null);

  const promptDisplay = useMemo(
    () => (prompt || ReplInputDefaultPrompt)(),
    [prompt],
  );

  const handleSubmit = useCallback<FormEventHandler>(
    e => {
      e.preventDefault();
      if (pending) { return; }
      onSubmit(value);
    },
    [value, pending, onSubmit],
  );

  // auto-focus on VM result
  useEffect(() => {
    if (pending) {
      inputRef.current?.blur();
    } else {
      inputRef.current?.focus()
    }
  }, [pending]);

  return (
    <form className="flex items-center" onSubmit={handleSubmit}>
      {promptDisplay}
      <input
        ref={inputRef}
        value={value}
        onChange={e => onChange(e.target.value)}
        disabled={pending}
      />
    </form>
  );
};

const ReplLinePastInput: FunctionComponent<ReplLineInputDatum> = ({
  value,
}) => (
  <div className="w-full flex items-center">
    <span className="text-gray-400">{value}</span>
  </div>
);

export type ReplLineOutputType = "empty" | "error" | "result" | "host";

export interface ReplLineOutputDatum {
  type: ReplLineOutputType;
  print: string;
}

const ReplLineOutput: FunctionComponent<ReplLineOutputDatum> = ({ type, print }) => {
  const contentClassName = useMemo(
    () => type === "error"
      ? "red-500"
      : type === "result"
        ? "green-500"
        : "gray-500",
    [type],
  );
  return (
    <div className="w-full flex items-center">
      <span className={contentClassName}>{print}</span>
    </div>
  );
};

export type ReplLineDatum = ReplLineInputDatum | ReplLineCurrentInputDatum | ReplLineOutputDatum;

export const ReplLine: FunctionComponent<ReplLineDatum> = props => useMemo(
  () => {
    switch (props.type) {
      case "input": return <ReplLinePastInput {...props} />;
      case "current-input": return <ReplLineActiveInput {...props} />;
      default: return <ReplLineOutput {...props} />;
    }
  }, [props],
);
