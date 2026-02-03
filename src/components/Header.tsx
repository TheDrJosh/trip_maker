import { Link, useNavigate } from "@tanstack/react-router";
import { useCurrentUser } from "@/hooks/auth";
import { loginFn } from "@/lib/auth/index.functions";
import { Button } from "./ui/button";

export default function Header() {
    return (
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
    );
}

function AuthHeader() {
    const { data } = useCurrentUser();

    const nav = useNavigate();
    // const login_fn = useServerFn(loginFn);

    if (data) {
        return (
            <Button asChild>
                <Link to="/account">{data.properties.username}</Link>
            </Button>
        );
    }

    return (
        <Button
            // asChild
            onClick={async () => {
                const url = await loginFn();

                if (url) {
                    nav({ href: url });
                } else {
                    nav({ to: "/" });
                }
            }}
        >
            {" "}
            Sign in
        </Button>
    );
}
