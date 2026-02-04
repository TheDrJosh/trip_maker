import { auth } from "@clerk/tanstack-react-start/server";
import { redirect } from "@tanstack/react-router";
import { createServerFn } from "@tanstack/react-start";

export const authStateFn = createServerFn({ method: "GET" }).handler(
    async () => {
        const { isAuthenticated, userId } = await auth();

        if (!isAuthenticated) {
            // This will error because you're redirecting to a path that doesn't exist yet
            // You can create a sign-in route to handle this
            throw redirect({
                to: "/sign-in/$",
            });
        }

        return { userId };
    },
);
