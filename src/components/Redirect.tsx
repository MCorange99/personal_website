import { Route, Router, Routes } from "@solidjs/router";


export default () => {
    const p = window.location.pathname;
    console.log(p);
    if (p.startsWith("/r")) {
        switch (p) {
            case "/r/github":
                redirect("https://github.com/MCorange99");
                break
        }
    }
   return (
       <>
        </>
   );
}






function redirect(p: string) {
    window.location.assign(p);
}