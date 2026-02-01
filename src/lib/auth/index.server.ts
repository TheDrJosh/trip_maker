// import { MemoryStorage } from "@openauthjs/openauth/storage/memory";
import { issuer } from "@openauthjs/openauth";
import { PasswordProvider } from "@openauthjs/openauth/provider/password";
import { MemoryStorage } from "@openauthjs/openauth/storage/memory";
import { PasswordUI } from "@openauthjs/openauth/ui/password";
import { redirect } from "@tanstack/react-router";
import { createServerFn } from "@tanstack/react-start";
import { getRequestHeader } from "@tanstack/react-start/server";
import { eq } from "drizzle-orm";
import { db } from "@/db";
import { users } from "@/db/schema";
import { client, useAuthSession } from ".";
import { subjects } from "./subjects";

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

async function getUser(email: string) {
    const db_user = await db.select().from(users).where(eq(users.email, email));

    if (db_user.length === 0) {
        throw new Error("User not found");
    }

    return {
        id: db_user[0].id,
        email: db_user[0].email,
        username: db_user[0].username,
    };
}

export default issuer({
    subjects,
    storage: MemoryStorage(),
    providers: {
        password: PasswordProvider(
            PasswordUI({
                copy: {
                    error_email_taken: "This email is already taken.",
                },
                sendCode: async (email, code) => console.log(email, code),
            }),
        ),
    },
    success: async (ctx, value) => {
        if (value.provider === "password") {
            return ctx.subject("user", await getUser(value.email));
        }
        throw new Error("Invalid provider");
    },
});
