import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/_authed/generate")({
    component: RouteComponent,
});

function RouteComponent() {
    return <div>Hello "/generate"!</div>;
}
