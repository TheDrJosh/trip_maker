import { withForm } from "@/hooks/form";
import { defaultGenerateSettings } from "@/routes/_authed/generate";

export const DistanceFields = withForm({
    defaultValues: defaultGenerateSettings,

    render: function Render({ form }) {
        return (
            <div className="flex flex-col gap-4 flex-1">
                <form.AppField
                    name="distanceUnit"
                    children={(field) => (
                        <field.SelectField
                            label={<>Distance Unit</>}
                            description={
                                <>
                                    Unit to use for the unit of{" "}
                                    <b>Max Distance</b> and for distances in
                                    results.
                                </>
                            }
                            values={[
                                {
                                    show: <>Miles</>,
                                    value: "mi",
                                },
                                {
                                    show: <>Kilometers</>,
                                    value: "km",
                                },
                            ]}
                        />
                    )}
                />
                <form.AppField
                    name="maxDistance"
                    children={(field) => (
                        <field.TextField
                            label="Max Distance"
                            description="Max distance the given location."
                            placeholder="20"
                        />
                    )}
                />
            </div>
        );
    },
});
