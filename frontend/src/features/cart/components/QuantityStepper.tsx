import { Minus, Plus } from "lucide-react";

import { Button } from "@/components/ui/button";

interface QuantityStepperProps {
  value: number;
  min?: number;
  max?: number;
  disabled?: boolean;
  onChange: (value: number) => void;
}

export function QuantityStepper({ value, min = 1, max, disabled, onChange }: QuantityStepperProps) {
  const canDecrement = value > min && !disabled;
  const canIncrement = (max === undefined || value < max) && !disabled;

  return (
    <div className="flex items-center gap-2">
      <Button
        type="button"
        variant="outline"
        size="icon"
        className="size-7"
        disabled={!canDecrement}
        onClick={() => onChange(value - 1)}
      >
        <Minus className="size-3" />
      </Button>
      <span className="w-6 text-center text-sm tabular-nums">{value}</span>
      <Button
        type="button"
        variant="outline"
        size="icon"
        className="size-7"
        disabled={!canIncrement}
        onClick={() => onChange(value + 1)}
      >
        <Plus className="size-3" />
      </Button>
    </div>
  );
}
