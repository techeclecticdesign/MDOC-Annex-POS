import TextField from "@mui/material/TextField";
import AppButton from "../../../components/AppButton";
import type { OperatorDto } from "../../../hooks/useOperators";

interface Props {
  operator: OperatorDto;
  onTerminate: () => void;
  onRehire: () => void;
}

export default function OperatorDetails({
  operator,
  onTerminate,
  onRehire,
}: Props) {
  return (
    <div className="space-y-6">
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6 max-w-2xl">
        {["MDOC", "Name", "Start Time", "Stop Time"].map((label, i) => {
          const value =
            i === 0
              ? operator.id
              : i === 1
                ? operator.name
                : i === 2
                  ? new Date(operator.start).toLocaleString()
                  : operator.stop
                    ? new Date(operator.stop).toLocaleString()
                    : "—";
          return (
            <TextField
              key={i}
              label={label}
              value={value}
              slotProps={{ input: { readOnly: true } }}
              variant="filled"
            />
          );
        })}
        <AppButton
          variant="contained"
          onClick={onTerminate}
          text="Terminate Operator"
        />
        <AppButton
          variant="contained"
          onClick={onRehire}
          text="Rehire Operator"
        />
      </div>
    </div>
  );
}
