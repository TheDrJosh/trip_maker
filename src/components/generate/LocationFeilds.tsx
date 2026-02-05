import { withForm } from "@/hooks/form";
import { defaultGenerateSettings } from "@/routes/_authed/generate";
import { Button } from "../ui/button";
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

// function getLocation(
//     setLocation: (location: { latitude: number; longitude: number }) => void,
// ) {
//     navigator.geolocation.getCurrentPosition(
//         (pos) => {
//             console.log(pos);
//             setLocation({
//                 latitude: pos.coords.latitude,
//                 longitude: pos.coords.longitude,
//             });
//         },
//         async (err) => {
//             console.log(err);

//             const loc = z
//                 .object({ latitude: z.number(), longitude: z.number() })
//                 .parse(await (await fetch("http://ipwho.is")).json());

//             setLocation(loc);
//         },
//     );
// }

export const LocationFeilds = withForm({
    defaultValues: defaultGenerateSettings,

    render: function Render({ form }) {
        return (
            <div className="flex flex-col gap-4 flex-1">
                <form.AppField
                    name="location"
                    children={(field) => {
                        const isInvalid =
                            field.state.meta.isTouched &&
                            !field.state.meta.isValid;

                        return (
                            <Field
                                orientation="responsive"
                                data-invalid={isInvalid}
                            >
                                <FieldContent>
                                    <FieldLabel htmlFor={field.name}>
                                        Location
                                    </FieldLabel>
                                    <FieldDescription>
                                        The Type of location
                                    </FieldDescription>

                                    {isInvalid && (
                                        <FieldError
                                            errors={field.state.meta.errors}
                                        />
                                    )}
                                </FieldContent>
                                <Select
                                    name={field.name}
                                    value={
                                        field.state.value === "currentPosition"
                                            ? "currentPosition"
                                            : "address" in field.state.value
                                              ? "address"
                                              : "coordinate"
                                    }
                                    onValueChange={(value) => {
                                        if (value === "currentPosition") {
                                            field.handleChange(
                                                "currentPosition",
                                            );
                                        } else if (value === "address") {
                                            field.handleChange({
                                                address: "",
                                            });
                                        } else {
                                            field.handleChange({
                                                latitude: "",
                                                longitude: "",
                                            });
                                        }
                                    }}
                                >
                                    <SelectTrigger
                                        id={field.name}
                                        aria-invalid={isInvalid}
                                        className="min-w-30"
                                    >
                                        <SelectValue placeholder="Select" />
                                    </SelectTrigger>
                                    <SelectContent position="item-aligned">
                                        <SelectItem value="currentPosition">
                                            Current Position
                                        </SelectItem>
                                        <SelectItem value="coordinate">
                                            Coordinate
                                        </SelectItem>
                                        <SelectItem value="address">
                                            Address
                                        </SelectItem>
                                    </SelectContent>
                                </Select>
                            </Field>
                        );
                    }}
                />

                <form.Subscribe
                    selector={(state) => state.values.location}
                    children={(location) => {
                        if (location === "currentPosition") {
                            return null;
                        } else if ("address" in location) {
                            return (
                                <form.AppField
                                    name="location.address"
                                    children={(field) => (
                                        <field.TextField label="Address" />
                                    )}
                                />
                            );
                        } else {
                            return (
                                <>
                                    <form.AppField
                                        name="location.latitude"
                                        children={(field) => (
                                            <field.TextField label="Latitude" />
                                        )}
                                    />
                                    <form.AppField
                                        name="location.longitude"
                                        children={(field) => (
                                            <field.TextField label="Longitude" />
                                        )}
                                    />
                                </>
                            );
                        }
                    }}
                />

                <div className="flex flex-row gap-2">
                    <Button type="button">Save</Button>
                    <Button type="button">Load</Button>
                </div>
            </div>
        );
    },
});
