import { createServerFn } from "@tanstack/react-start";
import { getRequestHeader } from "@tanstack/react-start/server";
import { subjects } from "@/lib/auth/sesstion";
import { authClient, useAuthSession } from "./index.server";

export const loginFn = createServerFn({ method: "POST" }).handler(async () => {
    const session = await useAuthSession();
    if (session.data.accessToken) {
        const verified = await authClient.verify(
            subjects,
            session.data.accessToken,
            {
                refresh: session.data.refreshToken,
            },
        );
        if (!verified.err && verified.tokens) {
            await session.update({
                accessToken: verified.tokens.access,
                refreshToken: verified.tokens.refresh,
            });
            return false;
        }
    }

    const host = getRequestHeader("host");
    const protocol = host?.includes("localhost") ? "http" : "https";
    const redirect_url = `${protocol}://${host}/api/callback`;
    console.log(redirect_url);
    const { url } = await authClient.authorize(redirect_url, "code");

    return url;
});

export const logoutFn = createServerFn({ method: "POST" }).handler(async () => {
    const session = await useAuthSession();

    await session.clear();
});

export const getCurrentUserFn = createServerFn({ method: "GET" }).handler(
    async () => {
        console.log("get user 1");
        const session = await useAuthSession();
        console.log("get user 2");

        if (!session.data.accessToken) {
            return false;
        }

        console.log("get user 3");

        const verified = await authClient.verify(
            subjects,
            session.data.accessToken,
            {
                refresh: session.data.refreshToken,
            },
        );

        console.log("get user 4");

        if (verified.err) {
            console.log(verified.err);
            return false;
        }

        console.log("get user 5");

        if (verified.tokens) {
            console.log("get user 6");

            await session.update({
                accessToken: verified.tokens.access,
                refreshToken: verified.tokens.refresh,
            });
        }

        console.log("get user 7");

        return verified.subject;
    },
);
