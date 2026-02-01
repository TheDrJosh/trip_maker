import { issuer } from "@openauthjs/openauth";
import { createClient } from "@openauthjs/openauth/client";
import { PasswordProvider } from "@openauthjs/openauth/provider/password";
import { MemoryStorage } from "@openauthjs/openauth/storage/memory";
import { PasswordUI } from "@openauthjs/openauth/ui/password";
import { eq } from "drizzle-orm";
import { db } from "@/db";
import { users } from "@/db/schema";
import { env } from "@/env";
import { subjects } from "./subjects";

export const client = createClient({
    clientID: "trip-maker",
    issuer: env.OPENAUTH_URL,
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
