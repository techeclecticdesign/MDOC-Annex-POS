import Autocomplete from "@mui/material/Autocomplete";
import TextField from "@mui/material/TextField";
import AppButton from "../../../components/AppButton";
import { OperatorDto } from "../../../hooks/useOperators";

interface Props {
  options: OperatorDto[];
  selected: OperatorDto | null;
  onChange: (op: OperatorDto | null) => void;
  onAddClick: () => void;
}

export default function OperatorSelector({
  options,
  selected,
  onChange,
  onAddClick,
}: Props) {
  return (
    <div className="flex gap-4 mb-6">
      <Autocomplete
        className="w-full max-w-sm"
        options={options}
        getOptionLabel={(o) => o.name}
        value={selected}
        onChange={(_, o) => onChange(o)}
        renderInput={(params) => (
          <TextField {...params} label="Select operator" variant="outlined" />
        )}
      />
      <AppButton onClick={onAddClick} text="Add Operator" />
    </div>
  );
}
