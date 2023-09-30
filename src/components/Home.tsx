import "../css/home.css"

import rust_logo from "../assets/lang_icons/rust-logo-blk.svg"
import ts_logo from "../assets/lang_icons/ts-logo-128.svg"
import nasm_logo from "../assets/lang_icons/nasm.svg"
import python_logo from "../assets/lang_icons/python.svg"
import c_logo from "../assets/lang_icons/C.svg"
import cpp_logo from "../assets/lang_icons/CPP.svg"
import js_logo from "../assets/lang_icons/js.png" // :(

function ListItem(logo: string | any, name: string) {
    return (
        <table>
            <tbody>
                <tr>
                    <td>
                        <img width={32} height={32} src={logo}></img>
                    </td>
                    <td>
                        <h2 class="m-0" style={"margin:3px;"}>{name}</h2>
                    </td>
                </tr>
            </tbody>
        </table>
    )
}




export default function Home() {
    return (
        <>  
            <h1>Hi, im MCorange, i... code stuff!</h1>

            <h2>Here are all the languages i have used ordered by how much i like it:</h2>

            <div style={"margin-left: 20px"}>
                {ListItem(rust_logo, "Rust")}
                {ListItem(ts_logo, "Typescript")}
                {ListItem(nasm_logo, "NASM")}
                {ListItem(python_logo, "Python 3")}
                {ListItem(c_logo, "C")}
                {ListItem(js_logo, "Javascript")}
                {ListItem(cpp_logo, "C++")}
            </div>

            <h1>Other stuff about me</h1>

            <h2>You can get my contact info and socials <a href="/socials">here</a></h2>
            <h2>Buy me a monster <a>here</a></h2>
            <h2>Check out my work at my <a href="/r/github">GitHub</a></h2>

            <h5>Note: I am a horrible web developer, if you want to help me improve my website you can do that <a href="https://github.com/MCorange99/personal_website">here</a></h5>
        </>
    )
}