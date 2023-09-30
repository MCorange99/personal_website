import logoSvg from "../assets/logo.svg";
// import userSvg from "../assets/user.svg";
// import {} from "../App";
import { A } from '@solidjs/router';


export default function Header() {

    return (
        <>
        <nav id="main-header">
            <A href="/">
                <img height={42} width={42} id="main-header-logo" src={logoSvg}></img>
            </A>
            <A class="main-header-button" href="/" >Home</A>
            {/* <A class="main-header-button" href="/blog" >Blog</A> */}
            <A class="main-header-button" href="/social" >Social</A>
            <A class="main-header-button" href="/donate" >Donate</A>
            {/* <A class="main-header-button" href="/about" >About</A> */}
        </nav>
        </>
    )
}