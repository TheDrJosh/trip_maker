import { serve } from "@hono/node-server";
import { issuer } from "@openauthjs/openauth/issuer";
import { PasswordProvider } from "@openauthjs/openauth/provider/password";
import { MemoryStorage } from "@openauthjs/openauth/storage/memory";
import { PasswordUI } from "@openauthjs/openauth/ui/password";
import { eq } from "drizzle-orm";
import { generateFromEmail } from "unique-username-generator";
import { users } from "../db/schema";
import { subjects } from "../lib/auth/sesstion";
import { db } from "./db";
import { env } from "./env";

// import { StorageAdapter } from "@openauthjs/openauth/storage/storage";

async function getUser(email: string) {
    const db_user = await db.select().from(users).where(eq(users.email, email));

    if (db_user.length === 0) {
        const username = generateFromEmail(email);

        const created_user = (
            await db
                .insert(users)
                .values({
                    email: email,
                    username: username,
                })
                .returning()
        )[0];

        return {
            id: created_user.id,
            email: created_user.email,
            username: created_user.username,
        };
    }

    return {
        id: db_user[0].id,
        email: db_user[0].email,
        username: db_user[0].username,
    };
}

const app = issuer({
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
            const user = await getUser(value.email);

            return ctx.subject("user", user);
        }
        throw new Error("Invalid provider");
    },
});

// type RedisStorageOptions {
//     redis_url: string,
//     key_prefix?: string,
// }

// https://github.com/anomalyco/openauth/issues/37#issuecomment-2996998384
// function redisStorage(options: RedisStorageOptions): StorageAdapter {
//     return {

//     }
// }

serve({
    port: env.PORT,
    ...app,
});
