import { withForm } from "@/hooks/form";
import { defaultGenerateSettings } from "@/routes/_authed/generate";

export const OtherFields = withForm({
    defaultValues: defaultGenerateSettings,

    render: function Render({ form }) {
        return (
            <div className="flex flex-col gap-4 flex-1">
                <form.AppField
                    name="closenessBias"
                    children={(field) => (
                        <field.SliderField
                            label={<>Closeness Bias</>}
                            description={
                                <>
                                    Raise percent distance to this power to bias
                                    the distance.
                                </>
                            }
                            min={0.2}
                            max={5}
                            step={0.1}
                            default_value={1}
                        />
                    )}
                />
                <form.AppField
                    name="minimumRating"
                    children={(field) => (
                        <field.SliderField
                            label={<>Minimum Rating</>}
                            description={
                                <>Minimum rating that the for search.</>
                            }
                            min={0.0}
                            max={5}
                            step={0.1}
                            default_value={2.5}
                        />
                    )}
                />
                <form.AppField
                    name="numberToGenerate"
                    children={(field) => (
                        <field.TextField
                            label={<>Number to Generate</>}
                            description={<>Number of results to generate.</>}
                            placeholder={"5"}
                        />
                    )}
                />
            </div>
        );
    },
});
