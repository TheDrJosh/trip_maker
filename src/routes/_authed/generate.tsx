import { createFileRoute } from "@tanstack/react-router";
import z from "zod";
import { DistanceFields } from "@/components/generate/DistanceFields";
import { LocationFeilds } from "@/components/generate/LocationFeilds";
import { OtherFields } from "@/components/generate/OtherFields";
import { Button } from "@/components/ui/button";
import {
    Card,
    CardContent,
    CardFooter,
    CardHeader,
    CardTitle,
} from "@/components/ui/card";
import { useAppForm } from "@/hooks/form";

export const Route = createFileRoute("/_authed/generate")({
    component: RouteComponent,
});

const generateSettings = z.object({
    location: z.union([
        z.object({
            latitude: z.coerce.number<string>(),
            longitude: z.coerce.number<string>(),
        }),
        z.object({ address: z.string() }),
        z.literal("currentPosition"),
    ]),
    distanceUnit: z.literal(["mi", "km"]),
    maxDistance: z.coerce.number<string>().min(0),
    closenessBias: z.coerce.number<string>().min(0.2).max(5),
    minimumRating: z.coerce.number<string>().min(0).max(5),
    numberToGenerate: z.coerce.number<string>().int().min(1).max(15),
});

export const defaultGenerateSettings: z.input<typeof generateSettings> = {
    location: "currentPosition",
    distanceUnit: "mi",
    maxDistance: "20",
    closenessBias: "1",
    minimumRating: "2.5",
    numberToGenerate: "5",
};

function RouteComponent() {
    const form = useAppForm({
        defaultValues: defaultGenerateSettings,
        validators: {
            onSubmit: generateSettings,
            onChange: generateSettings,
            onBlur: generateSettings,
        },
        onSubmit: async ({ value }) => {
            console.log(value);
        },
    });

    return (
        <div className="flex flex-col items-center m-12">
            <Card className="min-w-64">
                <CardHeader>
                    <CardTitle>Search Settings</CardTitle>
                </CardHeader>
                <CardContent>
                    <form
                        onSubmit={(e) => {
                            e.preventDefault();
                            form.handleSubmit();
                        }}
                        className="flex flex-col lg:flex-row gap-8"
                        id="generate-form"
                    >
                        <LocationFeilds form={form} />
                        <DistanceFields form={form} />
                        <OtherFields form={form} />
                    </form>
                </CardContent>
                <CardFooter>
                    <form.Subscribe
                        selector={(state) => [
                            state.canSubmit,
                            state.isSubmitting,
                        ]}
                        children={([canSubmit, isSubmitting]) => (
                            <Button type="submit" disabled={!canSubmit}>
                                {isSubmitting ? "..." : "Generate"}
                            </Button>
                        )}
                    />
                </CardFooter>
            </Card>
        </div>
    );
}
