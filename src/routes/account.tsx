import { createFileRoute } from "@tanstack/react-router";
import { Button } from "@/components/ui/button";
import { logoutFn } from "@/lib/auth/index.functions";

export const Route = createFileRoute("/account")({
    component: RouteComponent,
});

function RouteComponent() {
    return <div><Button onClick={() => logoutFn()}>Logout</Button></div>;
}
