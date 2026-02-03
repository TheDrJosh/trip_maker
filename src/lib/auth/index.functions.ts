import { createServerFn } from "@tanstack/react-start";
import { getRequestHeader } from "@tanstack/react-start/server";
import { eq } from "drizzle-orm";
import z from "zod";
import { db } from "@/db";
import { users } from "@/db/schema";
import { subjects } from "@/lib/auth/sesstion";
import { authClient, useAuthSession } from "./index.server";

export const loginUrlFn = createServerFn({ method: "POST" }).handler(
    async () => {
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
                return null;
            }
        }

        const host = getRequestHeader("host");
        const protocol = host?.includes("localhost") ? "http" : "https";
        const redirect_url = `${protocol}://${host}/api/callback`;
        const { url } = await authClient.authorize(redirect_url, "code");

        return url;
    },
);

export const logoutFn = createServerFn({ method: "POST" }).handler(async () => {
    const session = await useAuthSession();

    await session.clear();
});

export const getCurrentUserFn = createServerFn({ method: "GET" }).handler(
    async () => {
        const session = await useAuthSession();

        if (!session.data.accessToken) {
            return null;
        }

        const verified = await authClient.verify(
            subjects,
            session.data.accessToken,
            {
                refresh: session.data.refreshToken,
            },
        );

        if (verified.err) {
            return null;
        }

        if (verified.tokens) {
            await session.update({
                accessToken: verified.tokens.access,
                refreshToken: verified.tokens.refresh,
            });
        }

        const user = await db
            .select()
            .from(users)
            .where(eq(users.id, verified.subject.properties.id));

        if (user.length === 1) {
            return user[0];
        }
        return null;
    },
);

export const setUsernameFn = createServerFn({ method: "POST" })
    .inputValidator(
        z.object({
            user_id: z.int(),
            username: z.string().min(3),
        }),
    )
    .handler(async ({ data }) => {
        await db
            .update(users)
            .set({
                username: data.username,
            })
            .where(eq(users.id, data.user_id));
    });
