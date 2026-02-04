import { createFileRoute } from "@tanstack/react-router";
import { authStateFn } from "@/lib/auth.functions";

export const Route = createFileRoute("/_authed")({
    beforeLoad: async () => await authStateFn(),
    loader: async ({ context }) => {
        return { userId: context.userId };
    },
});
