import { createFileRoute, redirect } from "@tanstack/react-router";
import z from "zod";
import { client, useAuthSession } from "@/lib/auth";

const callbackSearchParams = z.object({
    code: z.string(),
});

export const Route = createFileRoute("/api/callack")({
    validateSearch: callbackSearchParams,
    server: {
        handlers: {
            GET: async () => {
                const code = Route.useSearch().code;

                const session = await useAuthSession();

                const exchanged = await client.exchange(code, `/api/callback`);

                if (exchanged.err) {
                    return Response.json(exchanged.err, { status: 400 });
                }

                session.update({
                    accessToken: exchanged.tokens.access,
                    refreshToken: exchanged.tokens.refresh,
                });

                throw redirect({ to: "/" });
            },
        },
    },
});
