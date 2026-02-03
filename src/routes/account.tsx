import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { Button } from "@/components/ui/button";
import { logoutFn } from "@/lib/auth/index.functions";

export const Route = createFileRoute("/account")({
    component: RouteComponent,
});

function RouteComponent() {
    const navigate = useNavigate();

    return (
        <div>
            <Button
                onClick={() => logoutFn().then(() => navigate({ to: "/" }))}
            >
                Logout
            </Button>
        </div>
    );
}
