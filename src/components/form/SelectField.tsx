import { useStore } from "@tanstack/react-form";
import type { ReactNode } from "react";
import { useFieldContext } from "@/hooks/form-context";
import {
    Field,
    FieldContent,
    FieldDescription,
    FieldError,
    FieldLabel,
} from "../ui/field";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "../ui/select";

export default function SelectField({
    label,
    description,
    values,
}: {
    label: ReactNode;
    description?: ReactNode;
    values: {
        show: ReactNode;
        value: string;
    }[];
}) {
    const field = useFieldContext<string>();

    const errors = useStore(field.store, (state) => state.meta.errors);

    const isInvalid = field.state.meta.isTouched && !field.state.meta.isValid;

    return (
        <Field orientation="responsive" data-invalid={isInvalid}>
            <FieldContent>
                <FieldLabel htmlFor={field.name}>{label}</FieldLabel>
                {description ? (
                    <FieldDescription>{description}</FieldDescription>
                ) : null}

                {isInvalid && <FieldError errors={errors} />}
            </FieldContent>
            <Select
                name={field.name}
                value={field.state.value}
                onValueChange={field.handleChange}
            >
                <SelectTrigger
                    id={field.name}
                    aria-invalid={isInvalid}
                    className="min-w-30"
                >
                    <SelectValue placeholder="Select" />
                </SelectTrigger>
                <SelectContent position="item-aligned">
                    {values.map(({ value, show }) => (
                        <SelectItem key={value} value={value}>
                            {show}
                        </SelectItem>
                    ))}
                </SelectContent>
            </Select>
        </Field>
    );
}
