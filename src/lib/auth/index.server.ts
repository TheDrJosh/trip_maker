import { issuer } from "@openauthjs/openauth";
import { createClient } from "@openauthjs/openauth/client";
import { PasswordProvider } from "@openauthjs/openauth/provider/password";
import { MemoryStorage } from "@openauthjs/openauth/storage/memory";
import { createSubjects } from "@openauthjs/openauth/subject";
import { PasswordUI } from "@openauthjs/openauth/ui/password";
import { useSession } from "@tanstack/react-start/server";
import { eq } from "drizzle-orm";
import z from "zod";
import { db } from "@/db";
import { users } from "@/db/schema";
import { env } from "@/env";

export const authClient = createClient({
    clientID: "trip-maker",
    issuer: env.RAILWAY_PUBLIC_DOMAIN,
});

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

export const subjects = createSubjects({
    user: z.object({
        id: z.int(),
        username: z.string(),
        email: z.string(),
    }),
});


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

export type SessionData = {
    accessToken: string;
    refreshToken: string;
};
export function useAuthSession() {
    return useSession<SessionData>({
        name: "trip_maker",
        password: env.SESSION_SECRET,
        cookie: {
            secure: process.env.NODE_ENV === "production",
            sameSite: "lax",
            httpOnly: true,
        },
    });
}

