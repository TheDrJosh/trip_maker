import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/login")({
    component: RouteComponent,
});

function RouteComponent() {
    return (
        <section className="flex flex-col items-center">
            <div className="mt-16 bg-zinc-800 rounded-xl px-4 py-2">
                <h2 className="text-4xl font-bold mb-4">Log in</h2>
            </div>
        </section>
    );
}
