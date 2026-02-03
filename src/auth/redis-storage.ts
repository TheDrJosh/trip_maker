import {
    joinKey,
    type StorageAdapter,
} from "@openauthjs/openauth/storage/storage";
import { createClient } from "redis";

type RedisStorageOptions = {
    redis_url: string;
    key_prefix?: string;
    default_TTL?: number;
};

const separator = String.fromCharCode(0x1f);

// https://github.com/anomalyco/openauth/issues/37#issuecomment-2996998384
export async function RedisStorage(options: RedisStorageOptions): Promise<StorageAdapter> {
    const redis = await createClient({
        url: options.redis_url,
    })
        .on("error", (err) => console.log("Redis CLient Error", err))
        .connect();
    
    const prefix = "storage";

    function create_redis_key(key: string[]): string {
        return `${prefix}:${joinKey(key)}`;
    }

    function parse_redis_key(redisKey: string): string[] {
        const withoutPrefix = redisKey.slice(prefix.length + 1);
        return withoutPrefix.split(separator);
    }

    return {
        async get(key) {
            const redisKey = create_redis_key(key);
            const result = await redis.get(redisKey);

            if (!result) {
                return undefined;
            }

            try {
                return typeof result === "string" ? JSON.parse(result) : result;
            } catch (error) {
                if (error instanceof Error) {
                    console.log(
                        "Failed getting key in OpenAUTH Redis instance",
                        error,
                    );
                }

                return result;
            }
        },
        async remove(key) {
            const redisKey = create_redis_key(key);
            await redis.del(redisKey);
        },
        async *scan(prefix) {
            const scanPattern = `${create_redis_key(prefix)}${
                prefix.length ? separator : ""
            }*`;
            let cursor = "0";

            do {
                const { cursor: nextCursor, keys } = await redis.scan(cursor, {
                    MATCH: scanPattern,
                    COUNT: 100,
                });

                cursor = nextCursor;

                if (keys.length === 0) {
                    continue;
                }

                const values = await redis.mGet(keys);

                for (let i = 0; i < keys.length; i++) {
                    const key = keys[i];
                    if (!key) {
                        continue;
                    }
                    const keyParts = parse_redis_key(key);
                    const value = values[i];

                    if (value !== null) {
                        try {
                            const parsedValue =
                                typeof value === "string"
                                    ? JSON.parse(value)
                                    : value;
                            yield [keyParts, parsedValue];
                        } catch (error) {
                            if (error instanceof Error) {
                                console.log(
                                    "Error scanning keys in OpenAUTH Redis instance",
                                    error,
                                );
                            }
                            yield [keyParts, value];
                        }
                    }
                }
            } while (cursor !== "0");
        },
        async set(key, value, expiry) {
            const redisKey = create_redis_key(key);
            const serializedValue = JSON.stringify(value);

            if (expiry) {
                const ttlSeconds = Math.max(
                    1,
                    Math.floor((expiry.getTime() - Date.now()) / 1000),
                );
                await redis.set(redisKey, serializedValue, {
                    expiration: {
                        type: "EX",
                        value: ttlSeconds,
                    },
                });
            } else {
                await redis.set(redisKey, serializedValue);
            }
        },
    };
}
