import { Label } from "@radix-ui/react-label";
import { useForm } from "@tanstack/react-form";
import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { createServerFn } from "@tanstack/react-start";
import { Check, Pencil, X } from "lucide-react";
import { useState } from "react";
import z from "zod";
import { Button } from "@/components/ui/button";
import { FieldError } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { Separator } from "@/components/ui/separator";
import { useAuth } from "@/contexts/auth";
import { env } from "@/env";
import { logoutFn, setUsernameFn } from "@/lib/auth/index.functions";

export const Route = createFileRoute("/_authed/account")({
    component: RouteComponent,
});

function RouteComponent() {
    const navigate = useNavigate();

    const { user, refetch } = useAuth();

    const [editUsername, setEditUsername] = useState(false);

    const nav = useNavigate();

    const form = useForm({
        validators: {
            onChange: z.object({
                username: z.string().min(3),
            }),
        },
        defaultValues: {
            username: user?.username ?? "",
        },
        onSubmit: async ({ value }) => {
            if (user) {
                await setUsernameFn({
                    data: {
                        user_id: user?.id,
                        username: value.username,
                    },
                });
                refetch();
                setEditUsername(false);
            }
        },
    });

    if (!user) {
        return <div>Loading...</div>;
    }

    return (
        <div className="mx-8 my-6 flex flex-col gap-4 items-start">
            <div className="flex flex-row w-full gap-8">
                <h2 className="text-3xl tracking-tight font-bold">Account</h2>
                <div className="flex-1"></div>
                <Button
                    className="text-lg"
                    onClick={async () => {
                        await logoutFn();
                        refetch();
                        navigate({ to: "/" });
                    }}
                >
                    Logout
                </Button>
            </div>
            <Separator />
            <div className="flex flex-col gap-1">
                <Label htmlFor="username">Username</Label>
                {!editUsername ? (
                    <div className="flex flex-row gap-6 items-center">
                        <h3
                            id="username"
                            className="text-xl font-bold tracking-tight"
                        >
                            {user.username}
                        </h3>

                        <Button
                            variant="secondary"
                            size="icon"
                            onClick={() => {
                                form.setFieldValue("username", user.username);
                                setEditUsername(true);
                            }}
                        >
                            <Pencil />
                        </Button>
                    </div>
                ) : (
                    <form
                        className="flex flex-row gap-2 items-center"
                        onSubmit={(e) => {
                            e.preventDefault();
                            e.stopPropagation();
                            form.handleSubmit();
                        }}
                    >
                        <form.Field
                            name="username"
                            // biome-ignore lint/correctness/noChildrenProp: Tanstack From Api
                            children={(field) => {
                                return (
                                    <div className="flex flex-col self-start">
                                        <Input
                                            id={field.name}
                                            name={field.name}
                                            value={field.state.value}
                                            onBlur={field.handleBlur}
                                            onChange={(e) =>
                                                field.handleChange(
                                                    e.target.value,
                                                )
                                            }
                                        />
                                        <FieldError />
                                    </div>
                                );
                            }}
                        />

                        <Button variant="secondary" size="icon" type="submit">
                            <Check />
                        </Button>
                        <Button
                            variant="secondary"
                            size="icon"
                            type="button"
                            onClick={() => {
                                setEditUsername(false);
                            }}
                        >
                            <X />
                        </Button>
                    </form>
                )}
            </div>

            <Button
                className="mt-8"
                onClick={async () => {
                    const url = await passwordChangeFn();
                    await nav({ href: url });
                }}
            >
                Change Password
            </Button>
        </div>
    );
}
//size={256}
// Change Password URL: /password/change

export const passwordChangeFn = createServerFn({ method: "POST" }).handler(
    async () => {
        return `${env.AUTH_SERVER_URL}/password/change`;
    },
);
