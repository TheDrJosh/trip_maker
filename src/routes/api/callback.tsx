import { createFileRoute, redirect } from "@tanstack/react-router";
import { authClient, useAuthSession } from "@/lib/auth/index.server";

export const Route = createFileRoute("/api/callback")({
    server: {
        handlers: {
            GET: async ({ request }) => {
                const url = new URL(request.url);
                const code = url.searchParams.get("code");

                const session = await useAuthSession();

                if (!code) {
                    return Response.json(
                        { message: "code search param not found" },
                        { status: 400 },
                    );
                }

                const exchanged = await authClient.exchange(
                    code,
                    `/api/callback`,
                );

                console.log(exchanged);

                if (exchanged.err) {
                    console.log("error");
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
