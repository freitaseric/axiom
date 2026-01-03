import {Link} from '@tanstack/react-router'

export default function Header() {

    return (
        <header className="p-4 flex items-center justify-around bg-gray-800 text-white shadow-lg fixed w-full">
            <Link to="/">
                AXIOM
            </Link>
        </header>
    )
}
