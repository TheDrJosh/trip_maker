import { redirect } from "@tanstack/react-router";
import { createServerFn } from "@tanstack/react-start";
import { getRequestHeader } from "@tanstack/react-start/server";
import { client, useAuthSession } from ".";
import { subjects } from "./subjects.server";

export const loginFn = createServerFn({ method: "POST" }).handler(async () => {
    const session = await useAuthSession();
    if (session.data.accessToken) {
        const verified = await client.verify(
            subjects,
            session.data.accessToken,
            {
                refresh: session.data.refreshToken,
            },
        );
        if (!verified.err && verified.tokens) {
            session.update({
                accessToken: verified.tokens.access,
                refreshToken: verified.tokens.refresh,
            });
            throw redirect({ to: "/" });
        }
    }

    const host = getRequestHeader("host");
    const protocol = host?.includes("localhost") ? "http" : "https";
    const { url } = await client.authorize(
        `${protocol}://${host}/api/callback`,
        "code",
    );
    throw redirect({ to: url });
});

export const logoutFn = createServerFn({ method: "POST" }).handler(async () => {
    const session = await useAuthSession();

    session.clear();

    throw redirect({ to: "/" });
});

export const getCurrentUserFn = createServerFn({ method: "GET" }).handler(
    async () => {
        const session = await useAuthSession();

        if (!session.data.accessToken) {
            return false;
        }

        const verified = await client.verify(
            subjects,
            session.data.accessToken,
            {
                refresh: session.data.refreshToken,
            },
        );

        if (verified.err) {
            return false;
        }

        if (verified.tokens) {
            session.update({
                accessToken: verified.tokens.access,
                refreshToken: verified.tokens.refresh,
            });
        }

        return verified.subject;
    },
);
