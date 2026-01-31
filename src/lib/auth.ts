import { eq, type InferSelectModel } from "drizzle-orm";
import { db } from "@/db/index.ts";
import { sessions } from "../db/schema.ts";

function generateSecureRandomString(): string {
    // Human readable alphabet (a-z, 0-9 without l, o, 0, 1 to avoid confusion)
    const alphabet = "abcdefghijkmnpqrstuvwxyz23456789";

    // Generate 24 bytes = 192 bits of entropy.
    // We're only going to use 5 bits per byte so the total entropy will be 192 * 5 / 8 = 120 bits
    const bytes = new Uint8Array(24);
    crypto.getRandomValues(bytes);

    let id = "";
    for (let i = 0; i < bytes.length; i++) {
        // >> 3 "removes" the right-most 3 bits of the byte
        id += alphabet[bytes[i] >> 3];
    }
    return id;
}
11;
async function createSession(): Promise<SessionWithToken> {
    const now = new Date();

    const id = generateSecureRandomString();
    const secret = generateSecureRandomString();
    const secretHash = Buffer.from(await hashSecret(secret));

    const token = `${id}.${secret}`;

    const session_times = await db.insert(sessions).values({
        id: id,
        createdAt: now,
        lastVerifiedAt: now,
        secretHash: secretHash,
    });

    const session_with_token: SessionWithToken = {
        id: id,
        secretHash: secretHash,
        token: token,
        createdAt: now,
        lastVerifiedAt: now,
        ...session_times,
    };

    return session_with_token;
}

async function hashSecret(secret: string): Promise<Uint8Array> {
    const secretBytes = new TextEncoder().encode(secret);
    const secretHashBuffer = await crypto.subtle.digest("SHA-256", secretBytes);
    return new Uint8Array(secretHashBuffer);
}

interface SessionWithToken extends Session {
    token: string;
}

type Session = InferSelectModel<typeof sessions>;

const inactivityTimeoutSeconds = 60 * 60 * 24 * 10; // 10 days
const activityCheckIntervalSeconds = 60 * 60; // 1 hour

async function validateSessionToken(token: string): Promise<Session | null> {
    const now = new Date();

    const tokenParts = token.split(".");
    if (tokenParts.length !== 2) {
        return null;
    }
    const sessionId = tokenParts[0];
    const sessionSecret = tokenParts[1];

    const session = await getSession(sessionId);
    if (!session) {
        return null;
    }

    const tokenSecretHash = await hashSecret(sessionSecret);
    const validSecret = constantTimeEqual(tokenSecretHash, session.secretHash);
    if (!validSecret) {
        return null;
    }

    if (
        now.getTime() - session.lastVerifiedAt.getTime() >=
        activityCheckIntervalSeconds * 1000
    ) {
        session.lastVerifiedAt = now;

        await db
            .update(sessions)
            .set({
                lastVerifiedAt: now,
            })
            .where(eq(sessions.id, sessionId));
    }

    return session;
}

async function getSession(sessionId: string): Promise<Session | null> {
    const now = new Date();

    const result = await db
        .select()
        .from(sessions)
        .where(eq(sessions.id, sessionId));

    if (result.length !== 1) {
        return null;
    }
    const session = result[0];

    // Inactivity timeout
    if (
        now.getTime() - session.lastVerifiedAt.getTime() >=
        inactivityTimeoutSeconds * 1000
    ) {
        await deleteSession(sessionId);
        return null;
    }

    return session;
}

async function deleteSession(sessionId: string): Promise<void> {
    await db.delete(sessions).where(eq(sessions.id, sessionId));
}

function constantTimeEqual(a: Uint8Array, b: Uint8Array): boolean {
    if (a.byteLength !== b.byteLength) {
        return false;
    }
    let c = 0;
    for (let i = 0; i < a.byteLength; i++) {
        c |= a[i] ^ b[i];
    }
    return c === 0;
}

function encodeSessionPublicJSON(session: Session): string {
	// Omit Session.secretHash
	const json = JSON.stringify({
		id: session.id,
		created_at: Math.floor(session.createdAt.getTime() / 1000)
	});
	return json;
}
