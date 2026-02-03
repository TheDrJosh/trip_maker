import { serve } from "@hono/node-server";
import { issuer } from "@openauthjs/openauth/issuer";
import { PasswordProvider } from "@openauthjs/openauth/provider/password";
// import { MemoryStorage } from "@openauthjs/openauth/storage/memory";
import { PasswordUI } from "@openauthjs/openauth/ui/password";
import { eq } from "drizzle-orm";
import { generateFromEmail } from "unique-username-generator";
import { users } from "../db/schema.ts";
import { subjects } from "../lib/auth/sesstion.ts";
import { db } from "./db.ts";
import { env } from "./env.ts";
import { RedisStorage } from "./redis-storage.ts";

async function getUser(email: string) {
    const db_user = await db.select().from(users).where(eq(users.email, email));

    if (db_user[0]) {
        return {
            id: db_user[0].id,
            email: db_user[0].email,
            username: db_user[0].username,
        };
    }
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

    if (!created_user) {
        throw new Error("Failed to create user");
    }

    return {
        id: created_user.id,
        email: created_user.email,
        username: created_user.username,
    };
}

const app = issuer({
    subjects,
    storage: await RedisStorage({ redis_url: env.REDIS_URL }),
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

serve({
    port: env.PORT,
    ...app,
});
