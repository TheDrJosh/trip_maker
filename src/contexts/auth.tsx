import { useQuery } from "@tanstack/react-query";
import { useServerFn } from "@tanstack/react-start";
import type { InferSelectModel } from "drizzle-orm";
import { createContext, type ReactNode, useContext } from "react";
import type { users } from "@/db/schema";
import { getCurrentUserFn } from "@/lib/auth/index.functions";

type AuthContextType = {
    user?: InferSelectModel<typeof users> | null;
    isLoading: boolean;
    refetch: () => void;
};

const AuthContext = createContext<AuthContextType | null>(null);

export function AuthProvider({ children }: { children: ReactNode }) {
    const getUser = useServerFn(getCurrentUserFn);

    const {
        data: user,
        isLoading,
        refetch,
    } = useQuery({
        queryKey: ["currentUser"],
        queryFn: getUser,
    });

    return (
        <AuthContext.Provider value={{ user, isLoading, refetch }}>
            {children}
        </AuthContext.Provider>
    );
}

export function useAuth() {
    const context = useContext(AuthContext);
    if (!context) {
        throw new Error("useAuth must be used within AuthProvider");
    }
    return context;
}
