import { Client } from "revolt.js";
import Image from "next/image";

const revolt: Client = new Client();

// temporaily here for debugging purposes
const email: string = "";
const password: string = "";

export default async function Home() {
    await revolt.login({
        "friendly_name": "Neovolt",
        "email": email,
        "password": password
    });

    return (
        <>
            <h1>Neovolt</h1>
            <p>The Revolt client of the yet</p>
        </>
    );
}
