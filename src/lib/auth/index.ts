import { createClient } from "@openauthjs/openauth/client";
import { useSession } from "@tanstack/react-start/server";
import { env } from "@/env";

export const client = createClient({
    clientID: "trip-maker",
    issuer: env.RAILWAY_PUBLIC_DOMAIN,
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

