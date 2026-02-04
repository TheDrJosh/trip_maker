import { SignIn } from "@clerk/tanstack-react-start";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/sign-in/$")({
    component: RouteComponent,
});

function RouteComponent() {
    return (
        <div className="flex flex-col items-center mt-16">
            <SignIn />
        </div>
    );
}
