import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/")({ component: App });

function App() {
    return (
        <section className="relative py-20 px-6 text-center overflow-hidden">
            <div className="relative max-w-5xl mx-auto">
                <div className="flex items-center justify-center gap-6 mb-6">
                    <h1 className="text-6xl md:text-7xl font-black text-white tracking-tighter">
                        <span className="text-zinc-300">Trip</span>{" "}
                        <span className="bg-linear-to-r from-amber-400 to-orange-400 bg-clip-text text-transparent">
                            Maker
                        </span>
                    </h1>
                </div>
                <p className="text-2xl md:text-3xl text-zinc-300 mb-4 font-light">
                    Create lists of potental day trips with ease.
                </p>
                <p className="text-lg text-zinc-400 max-w-3xl mx-auto mb-8">
                    expaneded description here...
                </p>
            </div>
        </section>
    );
}
