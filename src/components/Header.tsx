import { Link, useNavigate } from "@tanstack/react-router";
import { useAuth } from "@/contexts/auth";
import { loginUrlFn } from "@/lib/auth/index.functions";
import { Button } from "./ui/button";

export default function Header() {
    const { user } = useAuth();

    return (
        <>
            <header className="p-4 flex items-center bg-zinc-800 text-white shadow-lg">
                <h1 className="ml-4 text-4xl font-black tracking-tighter">
                    <Link to="/">
                        <span className="text-zinc-300">Trip</span>{" "}
                        <span className="bg-linear-to-r from-amber-400 to-orange-400 bg-clip-text text-transparent">
                            Maker
                        </span>
                    </Link>
                </h1>
                <div className="flex-1"></div>

                <AuthHeader />
            </header>
            {user ? (
                <nav className="flex flex-col sm:flex-row bg-zinc-800">
                    <Button variant="link" asChild>
                        <Link to="/generate">Generate</Link>
                    </Button>
                    <Button variant="link" asChild>
                        <Link to="/favorites">Favorites</Link>
                    </Button>
                </nav>
            ) : null}
        </>
    );
}

function AuthHeader() {
    const { user } = useAuth();

    const nav = useNavigate();

    if (user) {
        return (
            <Button asChild>
                <Link to="/account">{user.username}</Link>
            </Button>
        );
    }

    return (
        <Button
            onClick={async () => {
                const url = await loginUrlFn();

                if (url) {
                    nav({ href: url });
                } else {
                    nav({ to: "/" });
                }
            }}
        >
            Sign in
        </Button>
    );
}
