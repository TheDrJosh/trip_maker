import { useStore } from "@tanstack/react-form";
import type { ReactNode } from "react";
import { useFieldContext } from "@/hooks/form-context";
import { Field, FieldDescription, FieldError, FieldLabel } from "../ui/field";
import { Input } from "../ui/input";

export default function TextField({
    label,
    description,
    placeholder,
}: {
    label: ReactNode;
    description?: ReactNode;
    placeholder?: string,
}) {
    const field = useFieldContext<string>();

    const errors = useStore(field.store, (state) => state.meta.errors);

    const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid;

    return (
        <Field data-invalid={isInvalid}>
            <FieldLabel htmlFor={field.name}>{label}</FieldLabel>
            <Input
                id={field.name}
                name={field.name}
                value={field.state.value}
                onBlur={field.handleBlur}
                onChange={(e) => field.handleChange(e.target.value)}
                aria-invalid={isInvalid}
                placeholder={placeholder}
            />
            {description ? (
                <FieldDescription>{description}</FieldDescription>
            ) : null}
            {isInvalid && <FieldError errors={errors} />}
        </Field>
    );
}
