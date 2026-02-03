import { createFileRoute, redirect } from "@tanstack/react-router";
import { getCurrentUserFn, loginUrlFn } from "@/lib/auth/index.functions";

export const Route = createFileRoute("/_authed")({
    beforeLoad: async () => {
        const subject = await getCurrentUserFn();

        if (!subject) {
            const url = await loginUrlFn();
            if (url) {
                throw redirect({ href: url });
            } else {
                throw redirect({ to: "/" });
            }
        }

        return { subject };
    },
});
