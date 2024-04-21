import { Client } from "revolt.js";
import Image from "next/image";

const revolt: Client = new Client();

// temporaily here for debugging purposes
const email: string = "";
const password: string = "";

export default async function Home() {
    /* await revolt.login({
    "friendly_name": "Neovolt",
    "email": email,
    "password": password
    }); */

    return (
        <>
        <h1>Neovolt</h1>
        <p>The Revolt client of the yet</p>

        <small className="footer">
            <a href="https://revolt.chat/aup" target="_blank" className="footer-text">Acceptable Use Policy </a>
            •
            <a href="https://revolt.chat/terms" target="_blank" className="footer-text"> Terms of Service </a>
            •
            <a href="https://revolt.chat/terms" target="_blank" className="footer-text"> Privacy Policy</a>
        </small>
        </>
    );
}
