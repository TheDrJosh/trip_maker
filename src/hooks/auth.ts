import { useQuery } from "@tanstack/react-query";
import { useServerFn } from "@tanstack/react-start";
import { getCurrentUserFn } from "../lib/auth/index.functions";

export function useCurrentUser() {
    const getUser = useServerFn(getCurrentUserFn);

    return useQuery({
        queryKey: ["currentUser"],
        queryFn: getUser,
    });
}
