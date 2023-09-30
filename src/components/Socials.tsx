import { A } from "@solidjs/router";
import dc_logo from "../assets/socials/discord-mark-blue.svg";
import github_logo from "../assets/socials/github-mark-white.svg";
import mastadon_logo from "../assets/socials/mastadon.svg";
import matrix_logo from "../assets/socials/Matrix.svg";
import mail_icon from "../assets/socials/email.svg";

function ListItem(logo: string | any, name: string, href: string) {
    return (
        <table>
            <tbody>
                <tr>
                    <td>
                        <a href={href}><img width={32} height={32} src={logo}></img></a>
                    </td>
                    <td>
                        <a href={href}><h2 class="m-0" style={"margin:3px;"}>{name}</h2></a>
                    </td>
                </tr>
            </tbody>
        </table>
    )
}

export default function Socials() {
    return (
        <>
            <h1>Here are my socials</h1>


            {ListItem(mail_icon, "mcorangecodes@gmail.com", "mailto:mcorangecodes@gmail.com")}
            {ListItem(dc_logo, "@mcorange", "")}
            {ListItem(github_logo, "MCorange99", "/r/github")}
            {ListItem(mastadon_logo, "@river.group.lt@mcorange", "/r/mastadon")}
            {ListItem(matrix_logo, "@mcorange:matrix.mcorangehq.xyz", "")}
        </>
    )
}