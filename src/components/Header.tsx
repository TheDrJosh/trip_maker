import {
    SignedIn,
    SignedOut,
    SignInButton,
    UserButton,
} from "@clerk/tanstack-react-start";
import { Link } from "@tanstack/react-router";
import { Button } from "./ui/button";

export default function Header() {
    return (
        <>
            <header className="p-4 flex flex-row items-center bg-zinc-800 text-white shadow-lg">
                <h1 className="ml-4 text-4xl font-black tracking-tighter">
                    <Link to="/">
                        <span className="text-zinc-300">Trip</span>{" "}
                        <span className="bg-linear-to-r from-amber-400 to-orange-400 bg-clip-text text-transparent">
                            Maker
                        </span>
                    </Link>
                </h1>
                <div className="flex-1"></div>
                <div>
                    <SignedIn>
                        <UserButton />
                    </SignedIn>
                    <SignedOut>
                        <Button asChild>
                            <SignInButton />
                        </Button>
                    </SignedOut>
                </div>
            </header>
            <SignedIn>
                <nav className="flex flex-col sm:flex-row bg-zinc-800">
                    <Button variant="link" asChild>
                        <Link to="/generate">Generate</Link>
                    </Button>
                    <Button variant="link" asChild>
                        <Link to="/favorites">Favorites</Link>
                    </Button>
                </nav>
            </SignedIn>
        </>
    );
}
