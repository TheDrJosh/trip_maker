import { useStore } from "@tanstack/react-form";
import type { ReactNode } from "react";
import { useFieldContext } from "@/hooks/form-context";
import { Field, FieldDescription, FieldError, FieldLabel } from "../ui/field";
import { Input } from "../ui/input";
import { Slider } from "../ui/slider";

export default function TextField({
    label,
    description,
    min,
    max,
    step,
    default_value,
}: {
    label: ReactNode;
    description?: ReactNode;
    default_value?: number;
    min?: number;
    max?: number;
    step?: number;
}) {
    const field = useFieldContext<string>();

    const errors = useStore(field.store, (state) => state.meta.errors);

    const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid;

    return (
        <Field data-invalid={isInvalid}>
            <FieldLabel htmlFor={field.name}>{label}</FieldLabel>
            <div className="flex flex-row gap-2">
                <Slider
                    id={`${field.name}-slider`}
                    name={field.name}
                    value={(!isInvalid) ? [Number(field.state.value)] : []}
                    min={min}
                    max={max}
                    step={step}
                    onBlur={field.handleBlur}
                    onValueChange={(e) => field.handleChange(e[0].toString())}
                    aria-invalid={isInvalid}
                    className="flex-4"
                />
                <Input
                    id={field.name}
                    name={field.name}
                    value={field.state.value}
                    onBlur={field.handleBlur}
                    onChange={(e) => field.handleChange(e.target.value)}
                    aria-invalid={isInvalid}
                    placeholder={default_value?.toString()}
                    className="flex-1"
                />
            </div>

            {description ? (
                <FieldDescription>{description}</FieldDescription>
            ) : null}
            {isInvalid && <FieldError errors={errors} />}
        </Field>
    );
}
