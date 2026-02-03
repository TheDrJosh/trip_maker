import { createSubjects } from "@openauthjs/openauth/subject";
import z from "zod";

export const subjects = createSubjects({
    user: z.object({
        id: z.int(),
        email: z.string(),
    }),
});
