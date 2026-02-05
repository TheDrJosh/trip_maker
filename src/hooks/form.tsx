import { createFormHook } from "@tanstack/react-form";
import { lazy } from "react";
import { fieldContext, formContext } from "./form-context.tsx";

const TextField = lazy(() => import("@/components/form/TextField.tsx"));
const SelectField = lazy(() => import("@/components/form/SelectField.tsx"));
const SliderField = lazy(() => import("@/components/form/SliderField.tsx"));

export const { useAppForm, withForm, withFieldGroup } = createFormHook({
    fieldComponents: {
        TextField,
        SelectField,
        SliderField,
    },
    formComponents: {},
    fieldContext,
    formContext,
});
